use super::{Command, Prefix, State, Suffix};

pub struct If {
    pub variable: String,
    pub value: String,
}

impl Command for If {
    fn execute(
        &self,
        _state: &mut State,
        _prefix: Prefix,
        _suffix: Suffix,
    ) -> Result<(), anyhow::Error> {
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "IF"
    }
}
