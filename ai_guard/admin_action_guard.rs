use regex::RegexSet;
use crate::store::metrics::ResponseMetric;

#[derive(Clone, Debug)]
pub struct AdminActionGuard {
    allowed_verbs: RegexSet,
}

impl AdminActionGuard {
    pub fn new() -> Self {
        let patterns = &[
            r"^cargo\s+build(\s+--release)?$",
            r"^cargo\s+test(\s+--all-targets)?$",
            r"^cargo\s+doc(\s+--no-deps)?$",
        ];
        Self {
            allowed_verbs: RegexSet::new(patterns).expect("valid regex set"),
        }
    }

    pub fn validate(&self, cmd: &str) -> Result<ResponseMetric, String> {
        if !self.allowed_verbs.is_match(cmd) {
            return Err("AI command not in Cargo whitelist".to_string());
        }
        // Simple metric: high K, low D/DW for build/test commands
        let metric = ResponseMetric::new(
            0.92, // K
            0.18, // D
            0.10, // DW
            "AI-initiated Rust Cargo action within whitelist",
        );
        Ok(metric);
    }
}
