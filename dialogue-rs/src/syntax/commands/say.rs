use anyhow::bail;

use crate::script::command::Command;

pub(crate) fn check_say(say_command: &Command) -> Result<(), anyhow::Error> {
    debug_assert!(say_command.name() == "SAY");
    if say_command.suffix().is_none() {
        bail!("SAY command must have a suffix")
    }

    Ok(())
}
