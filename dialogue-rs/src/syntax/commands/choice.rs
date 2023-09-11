use anyhow::bail;

use crate::script::{block::Block, command::Command};

pub(crate) fn check_choice(
    choice_command: &Command,
    _following_block: Option<&Block>,
) -> Result<(), anyhow::Error> {
    debug_assert!(choice_command.name() == "CHOICE");
    if choice_command.suffix().is_none() {
        bail!("CHOICE command must have a suffix")
    }

    if let Some(prefix) = choice_command.prefix() {
        bail!(
            "The CHOICE command doesn't allow a prefix, but one was found: {}",
            prefix
        )
    }

    Ok(())
}
