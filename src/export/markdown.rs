use crate::error::ThinkingError;
use crate::model::session::ThinkingSession;

pub fn format_session_markdown(session: &ThinkingSession) -> Result<String, ThinkingError> {
    let mut output = String::new();

    output.push_str(&format!("# Thinking Session {}\n\n", session.session_id));
    output.push_str(&format!("Status: {:?}\n", session.status));
    output.push_str(&format!("Thoughts: {}\n", session.thoughts.len()));
    output.push_str(&format!("Branches: {}\n", session.branches.len()));

    if !session.branches.is_empty() {
        output.push('\n');
        for branch in &session.branches {
            output.push_str(&format!(
                "- **{}**: {} ({:?}, {} thoughts)\n",
                branch.branch_id, branch.branch_label, branch.branch_status, branch.thought_count
            ));
        }
    }

    output.push('\n');
    output.push_str("---\n\n");

    for thought in &session.thoughts {
        output.push_str(&format!(
            "## Thought {} — {:?}\n\n",
            thought.thought_number, thought.kind
        ));
        output.push_str(&format!("**Status:** {:?}\n", thought.status));
        if let Some(conf) = thought.confidence {
            output.push_str(&format!("**Confidence:** {:.2}\n", conf));
        }
        if let Some(ref bid) = thought.branch_id {
            output.push_str(&format!("**Branch:** {}\n", bid));
        }
        if !thought.depends_on.is_empty() {
            output.push_str(&format!(
                "**Depends on:** {}\n",
                thought
                    .depends_on
                    .iter()
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if thought.is_revision {
            if let Some(rev) = thought.revises_thought {
                output.push_str(&format!("**Revises thought:** {}\n", rev));
            }
        }
        if let Some(ref summary) = thought.reason_summary {
            output.push_str(&format!("\n*{}*\n", summary));
        }
        output.push('\n');
        output.push_str(&thought.content);
        output.push_str("\n\n");

        if !thought.evidence.is_empty() {
            output.push_str("**Evidence:**\n\n");
            for ev in &thought.evidence {
                output.push_str(&format!(
                    "- [{:?}] {} ({})\n",
                    ev.evidence_type, ev.title, ev.reference
                ));
            }
            output.push('\n');
        }

        if let Some(ref risk) = thought.risk {
            output.push_str(&format!(
                "**Risk:** {:?} / {:?} — {}\n\n",
                risk.level, risk.category, risk.description
            ));
        }

        if let Some(ref action) = thought.action_proposal {
            output.push_str(&format!("**Action proposal:** `{}`\n\n", action.tool));
        }

        if !thought.tags.is_empty() {
            output.push_str(&format!("**Tags:** {}\n\n", thought.tags.join(", ")));
        }

        output.push_str("---\n\n");
    }

    Ok(output)
}
