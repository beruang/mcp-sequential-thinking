pub mod patterns;

use crate::model::redaction::RedactionSummary;
use std::collections::HashMap;

pub struct RedactionResult {
    pub content: String,
    pub redactions: Vec<RedactionSummary>,
}

pub fn redact_text(text: &str) -> RedactionResult {
    let mut content = text.to_string();
    let mut counts: HashMap<String, usize> = HashMap::new();

    for pattern in patterns::REDACTION_PATTERNS.iter() {
        let re = &pattern.regex;
        let mut match_count = 0;
        // Apply replacement, counting matches
        let new_content = re.replace_all(&content, |_caps: &regex::Captures| {
            match_count += 1;
            format!("[REDACTED:{}]", pattern.kind)
        });
        if match_count > 0 {
            *counts.entry(pattern.kind.to_string()).or_insert(0) += match_count;
            content = new_content.to_string();
        }
    }

    let redactions: Vec<RedactionSummary> = counts
        .into_iter()
        .map(|(kind, count)| RedactionSummary { kind, count })
        .collect();

    RedactionResult {
        content,
        redactions,
    }
}
