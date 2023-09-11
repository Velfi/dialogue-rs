use super::{Command, Prefix, State, Suffix};

pub struct Goto {
    pub label: String,
}

impl Command for Goto {
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
