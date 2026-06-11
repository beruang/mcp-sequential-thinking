use std::sync::LazyLock;

use regex::Regex;

pub struct RedactionPattern {
    pub kind: &'static str,
    pub regex: Regex,
}

pub static REDACTION_PATTERNS: LazyLock<Vec<RedactionPattern>> = LazyLock::new(|| {
    vec![
        // Bearer tokens
        RedactionPattern {
            kind: "bearer_token",
            regex: Regex::new(r"(?i)bearer\s+[a-zA-Z0-9\-._~+/]+=*").unwrap(),
        },
        // Authorization headers
        RedactionPattern {
            kind: "auth_header",
            regex: Regex::new(r"(?i)authorization:\s*[^\s]+").unwrap(),
        },
        // API keys
        RedactionPattern {
            kind: "api_key",
            regex: Regex::new(r"(?i)(?:api[_-]?key|apikey)\s*[:=]\s*[^\s,;]+").unwrap(),
        },
        // Private keys (PEM)
        RedactionPattern {
            kind: "private_key",
            regex: Regex::new(
                r"-----BEGIN\s+(RSA\s+|EC\s+|OPENSSH\s+)?PRIVATE\s+KEY-----[\s\S]*?-----END",
            )
            .unwrap(),
        },
        // Passwords
        RedactionPattern {
            kind: "password",
            regex: Regex::new(r#"(?i)passwo?r?d\s*[:=]\s*["']?[^\s"',;}]+"#).unwrap(),
        },
        // DB URLs with credentials
        RedactionPattern {
            kind: "db_credentials",
            regex: Regex::new(r"(?i)(?:mysql|postgres|postgresql|mongodb|redis)://[^:]+:[^@]+@")
                .unwrap(),
        },
        // GitHub tokens
        RedactionPattern {
            kind: "github_token",
            regex: Regex::new(r"(?i)gh[ps]_[a-zA-Z0-9]{36,}").unwrap(),
        },
        // AWS access keys
        RedactionPattern {
            kind: "aws_key",
            regex: Regex::new(r"(?i)AKIA[0-9A-Z]{16}").unwrap(),
        },
        // AWS secret keys
        RedactionPattern {
            kind: "aws_secret",
            regex: Regex::new(r#"(?i)aws[_-]secret\s*[:=]\s*["']?[^"'\s]+"#).unwrap(),
        },
        // OpenAI-style keys
        RedactionPattern {
            kind: "openai_key",
            regex: Regex::new(r"(?i)sk-(?:proj-)?[a-zA-Z0-9]{32,}").unwrap(),
        },
        // JWTs
        RedactionPattern {
            kind: "jwt",
            regex: Regex::new(r"eyJ[a-zA-Z0-9\-_]+\.eyJ[a-zA-Z0-9\-_]+\.[a-zA-Z0-9\-_]+").unwrap(),
        },
    ]
});
