use super::{Command, Prefix, State, Suffix};

pub struct Say;

impl Command for Say {
    fn execute(
        &self,
        _state: &mut State,
        _prefix: Prefix,
        _suffix: Suffix,
    ) -> Result<(), anyhow::Error> {
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "SAY"
    }
}
