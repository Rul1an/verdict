use std::borrow::Cow;

#[derive(Debug, Clone, Copy, Default)]
pub struct RedactionPolicy {
    pub redact_prompts: bool,
}

impl RedactionPolicy {
    pub fn new(redact_prompts: bool) -> Self {
        Self { redact_prompts }
    }

    pub fn redact_prompt<'a>(&self, s: &'a str) -> Cow<'a, str> {
        if self.redact_prompts {
            "[REDACTED]".into()
        } else {
            s.into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redaction_on() {
        let policy = RedactionPolicy::new(true);
        assert_eq!(policy.redact_prompt("my secret prompt"), "[REDACTED]");
    }

    #[test]
    fn test_redaction_off() {
        let policy = RedactionPolicy::new(false);
        assert_eq!(policy.redact_prompt("safe prompt"), "safe prompt");
    }
}
