use anyhow::bail;

use crate::script::command::Command;

pub(crate) fn check_trigger(trigger_command: &Command) -> Result<(), anyhow::Error> {
    debug_assert!(trigger_command.name() == "TRIGGER");
    if let Some(prefix) = trigger_command.prefix() {
        bail!(
            "The TRIGGER command doesn't support a prefix, but one was found: {}",
            prefix
        )
    }

    if let Some(_suffix) = trigger_command.suffix() {
        // TODO should I validate trigger names?
        Ok(())
    } else {
        bail!("The TRIGGER command requires a suffix, but none was found",)
    }
}
