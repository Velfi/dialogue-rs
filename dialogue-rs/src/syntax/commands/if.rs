use crate::script::{block::Block, command::Command};

pub(crate) fn check_if(
    if_command: &Command,
    _following_block: Option<&Block>,
) -> Result<(), anyhow::Error> {
    debug_assert!(if_command.name() == "IF");
    todo!("Implement the check_if function")
}
