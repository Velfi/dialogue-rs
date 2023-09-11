use super::{Command, Prefix, State, Suffix};

pub struct Trigger {}

impl Command for Trigger {
    fn execute(
        &self,
        _state: &mut State,
        _prefix: Prefix,
        _suffix: Suffix,
    ) -> Result<(), anyhow::Error> {
        unimplemented!()
    }

    fn name(&self) -> &'static str {
        "TRIGGER"
    }
}
