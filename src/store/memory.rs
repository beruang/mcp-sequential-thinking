use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use chrono::Utc;

use crate::error::ThinkingError;
use crate::model::branch::{BranchStatus, BranchSummary};
use crate::model::session::{SessionSummary, ThinkingSession};
use crate::model::thought::ThoughtRecord;

use super::retention::is_expired;
use super::ThinkingStore;

pub struct MemoryStore {
    sessions: Arc<RwLock<HashMap<String, ThinkingSession>>>,
    ttl_seconds: u64,
    retention_enabled: bool,
}

impl MemoryStore {
    pub fn new(ttl_seconds: u64, retention_enabled: bool) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            ttl_seconds,
            retention_enabled,
        }
    }

    fn update_branch_summaries(session: &mut ThinkingSession, thought: &ThoughtRecord) {
        let branch_id = thought
            .branch_id
            .clone()
            .unwrap_or_else(|| "main".to_string());
        let branch_label = thought
            .branch_label
            .clone()
            .unwrap_or_else(|| "main".to_string());

        let branch = session
            .branches
            .iter_mut()
            .find(|b| b.branch_id == branch_id);
        if let Some(branch) = branch {
            branch.thought_count += 1;
            if let Some(ref bs) = thought.branch_status {
                branch.branch_status = *bs;
            }
        } else {
            session.branches.push(BranchSummary {
                branch_id: branch_id.clone(),
                branch_label,
                branch_from_thought: thought.branch_from_thought,
                branch_status: thought.branch_status.unwrap_or(BranchStatus::Active),
                thought_count: 1,
            });
        }
    }

    fn auto_supersede_revised(session: &mut ThinkingSession, thought: &ThoughtRecord) {
        if thought.is_revision {
            if let Some(revised_num) = thought.revises_thought {
                if let Some(target) = session
                    .thoughts
                    .iter_mut()
                    .find(|t| t.thought_number == revised_num)
                {
                    target.status = crate::model::thought::ThoughtStatus::Superseded;
                }
            }
        }
    }
}

impl ThinkingStore for MemoryStore {
    fn upsert_thought(&self, thought: ThoughtRecord) -> Result<ThoughtRecord, ThinkingError> {
        // Opportunistic cleanup
        if self.retention_enabled {
            drop(self.cleanup_expired());
        }

        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| ThinkingError::InternalError(e.to_string()))?;

        let session = sessions
            .entry(thought.session_id.clone())
            .or_insert_with(|| ThinkingSession::new(thought.session_id.clone()));

        let thought_clone = thought.clone();

        Self::update_branch_summaries(session, &thought_clone);
        Self::auto_supersede_revised(session, &thought_clone);

        session.thoughts.push(thought_clone);
        session.updated_at = Utc::now();

        Ok(thought)
    }

    fn get_session(&self, session_id: &str) -> Result<Option<ThinkingSession>, ThinkingError> {
        let sessions = self
            .sessions
            .read()
            .map_err(|e| ThinkingError::InternalError(e.to_string()))?;

        let Some(session) = sessions.get(session_id) else {
            return Ok(None);
        };

        if self.retention_enabled && is_expired(session.updated_at, self.ttl_seconds) {
            return Ok(None);
        }

        Ok(Some(session.clone()))
    }

    fn list_sessions(&self, limit: usize) -> Result<Vec<SessionSummary>, ThinkingError> {
        let sessions = self
            .sessions
            .read()
            .map_err(|e| ThinkingError::InternalError(e.to_string()))?;

        let mut summaries: Vec<SessionSummary> = sessions
            .values()
            .filter(|s| {
                if self.retention_enabled {
                    !is_expired(s.updated_at, self.ttl_seconds)
                } else {
                    true
                }
            })
            .map(|s| s.to_summary())
            .collect();

        summaries.sort_by_key(|b| std::cmp::Reverse(b.updated_at));
        summaries.truncate(limit);

        Ok(summaries)
    }

    fn clear_session(&self, session_id: &str) -> Result<bool, ThinkingError> {
        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| ThinkingError::InternalError(e.to_string()))?;

        Ok(sessions.remove(session_id).is_some())
    }

    fn cleanup_expired(&self) -> Result<usize, ThinkingError> {
        if !self.retention_enabled {
            return Ok(0);
        }

        let mut sessions = self
            .sessions
            .write()
            .map_err(|e| ThinkingError::InternalError(e.to_string()))?;

        let before = sessions.len();
        sessions.retain(|_, s| !is_expired(s.updated_at, self.ttl_seconds));
        Ok(before - sessions.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::thought::{ThoughtKind, ThoughtStatus};

    fn make_thought(session_id: &str, num: u32, kind: ThoughtKind) -> ThoughtRecord {
        ThoughtRecord {
            thought_id: format!("th_{:06}", num),
            session_id: session_id.to_string(),
            kind,
            content: format!("Thought {}", num),
            thought_number: num,
            total_thoughts: 10,
            next_thought_needed: true,
            status: ThoughtStatus::Active,
            confidence: None,
            reason_summary: None,
            branch_id: None,
            branch_label: None,
            branch_from_thought: None,
            branch_status: None,
            is_revision: false,
            revises_thought: None,
            depends_on: vec![],
            evidence: vec![],
            risk: None,
            action_proposal: None,
            tags: vec![],
            redactions: vec![],
            created_at: Utc::now(),
        }
    }

    #[test]
    fn test_upsert_creates_session() {
        let store = MemoryStore::new(3600, true);
        let thought = make_thought("s1", 1, ThoughtKind::Assumption);
        let result = store.upsert_thought(thought).unwrap();
        assert_eq!(result.thought_number, 1);

        let session = store.get_session("s1").unwrap().unwrap();
        assert_eq!(session.thoughts.len(), 1);
    }

    #[test]
    fn test_get_nonexistent_session() {
        let store = MemoryStore::new(3600, true);
        let result = store.get_session("missing").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_list_sessions() {
        let store = MemoryStore::new(3600, true);
        store
            .upsert_thought(make_thought("s1", 1, ThoughtKind::Assumption))
            .unwrap();
        store
            .upsert_thought(make_thought("s2", 1, ThoughtKind::Assumption))
            .unwrap();

        let list = store.list_sessions(50).unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_list_sessions_respects_limit() {
        let store = MemoryStore::new(3600, true);
        store
            .upsert_thought(make_thought("s1", 1, ThoughtKind::Assumption))
            .unwrap();
        store
            .upsert_thought(make_thought("s2", 1, ThoughtKind::Assumption))
            .unwrap();

        let list = store.list_sessions(1).unwrap();
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_clear_session() {
        let store = MemoryStore::new(3600, true);
        store
            .upsert_thought(make_thought("s1", 1, ThoughtKind::Assumption))
            .unwrap();

        assert!(store.clear_session("s1").unwrap());
        assert!(store.get_session("s1").unwrap().is_none());
    }

    #[test]
    fn test_clear_nonexistent_session() {
        let store = MemoryStore::new(3600, true);
        assert!(!store.clear_session("missing").unwrap());
    }

    #[test]
    fn test_cleanup_expired() {
        let store = MemoryStore::new(0, true); // 0 TTL — all sessions expired
        store
            .upsert_thought(make_thought("s1", 1, ThoughtKind::Assumption))
            .unwrap();

        let removed = store.cleanup_expired().unwrap();
        assert_eq!(removed, 1);
        assert!(store.get_session("s1").unwrap().is_none());
    }

    #[test]
    fn test_different_branches() {
        let store = MemoryStore::new(3600, true);
        let mut t1 = make_thought("s1", 1, ThoughtKind::Option);
        t1.branch_id = Some("branch-a".to_string());
        t1.branch_label = Some("Approach A".to_string());
        store.upsert_thought(t1).unwrap();

        let mut t2 = make_thought("s1", 2, ThoughtKind::Option);
        t2.branch_id = Some("branch-b".to_string());
        t2.branch_label = Some("Approach B".to_string());
        store.upsert_thought(t2).unwrap();

        let session = store.get_session("s1").unwrap().unwrap();
        assert_eq!(session.branches.len(), 2);
    }
}
