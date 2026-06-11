use crate::model::thought::{ThoughtKind, ThoughtStatus};

pub fn default_status_for_kind(kind: ThoughtKind) -> ThoughtStatus {
    match kind {
        ThoughtKind::Observation => ThoughtStatus::Verified,
        ThoughtKind::Assumption => ThoughtStatus::Unverified,
        ThoughtKind::Constraint
        | ThoughtKind::Risk
        | ThoughtKind::Option
        | ThoughtKind::NextAction
        | ThoughtKind::Question => ThoughtStatus::Active,
        ThoughtKind::Decision
        | ThoughtKind::Revision
        | ThoughtKind::Validation
        | ThoughtKind::FinalDecision => ThoughtStatus::Done,
        ThoughtKind::Blocker => ThoughtStatus::Blocked,
    }
}
