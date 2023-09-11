use super::{Command, Prefix, State, Suffix};

pub struct Choice {
    pub choices: Vec<ChoiceOption>,
}

impl Choice {
    pub fn new() -> Self {
        Self { choices: vec![] }
    }

    pub fn push(&mut self, choice: ChoiceOption) {
        self.choices.push(choice);
    }

    pub fn pop(&mut self) -> Option<ChoiceOption> {
        self.choices.pop()
    }
}

impl Command for Choice {
    fn execute(
        &self,
        _state: &mut State,
        _prefix: Prefix,
        _suffix: Suffix,
    ) -> Result<(), anyhow::Error> {
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "GOTO"
    }
}

pub struct ChoiceOption {
    pub text: String,
    pub target: String,
}
