#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleSeverity {
    Allow,
    Deny,
    Warn,
}

#[derive(Debug)]
pub struct SyntaxCheckerOptions {
    /// When a command with an unknown name is encountered, should it be allowed, denied, or a warning logged?
    unknown_commands: RuleSeverity,
    /// When a block is declared just after the start marker, should it be allowed, denied, or a warning logged?
    top_level_block: RuleSeverity,
}

impl Default for SyntaxCheckerOptions {
    fn default() -> Self {
        use RuleSeverity::*;

        Self {
            unknown_commands: Deny,
            top_level_block: Allow,
        }
    }
}

impl SyntaxCheckerOptions {
    pub fn unknown_commands(&self) -> RuleSeverity {
        self.unknown_commands
    }

    pub fn top_level_block(&self) -> RuleSeverity {
        self.top_level_block
    }
}
