use anyhow::bail;

use crate::script::{command::Command, marker::is_valid_marker_name};

pub(crate) fn check_goto(
    goto_command: &Command,
    has_following_sibling: bool,
) -> Result<(), anyhow::Error> {
    debug_assert!(goto_command.name() == "GOTO");

    if let Some(prefix) = goto_command.prefix() {
        bail!(
            "The GOTO command doesn't allow a prefix, but one was found: {}",
            prefix
        )
    }

    if has_following_sibling {
        bail!("Any lines after a GOTO command, but within the same block will be unreachable.")
    }

    if let Some(suffix) = goto_command.suffix() {
        match is_valid_marker_name(suffix) {
            true => Ok(()),
            false => bail!(
                "The GOTO command requires a valid marker name, but {} was found",
                suffix
            ),
        }
    } else {
        bail!("The GOTO command requires a suffix, but none was found",)
    }
}
