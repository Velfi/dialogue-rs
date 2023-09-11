mod choice;
mod goto;
mod r#if;
mod say;
mod set;
mod trigger;

use self::{
    choice::check_choice, goto::check_goto, r#if::check_if, say::check_say, set::check_set,
    trigger::check_trigger,
};

use super::{RuleSeverity, SyntaxCheckerOptions};
use crate::script::{block::Block, command::Command};
use anyhow::bail;

pub const CHOICE_COMMAND: &str = "CHOICE";
pub const GOTO_COMMAND: &str = "GOTO";
pub const IF_COMMAND: &str = "IF";
// TODO add ELSE command
pub const SAY_COMMAND: &str = "SAY";
pub const SET_COMMAND: &str = "SET";
pub const TRIGGER_COMMAND: &str = "TRIGGER";

pub(super) fn check_command(
    command: &Command,
    maybe_block: Option<&Block>,
    options: &SyntaxCheckerOptions,
) -> Result<(), anyhow::Error> {
    match command.name() {
        SAY_COMMAND => check_say(command),
        CHOICE_COMMAND => check_choice(command, maybe_block),
        // TODO anything following this command is unreachable, unless it's a Marker
        GOTO_COMMAND => check_goto(command, maybe_block.is_some()),
        SET_COMMAND => check_set(command),
        IF_COMMAND => check_if(command, maybe_block),
        TRIGGER_COMMAND => check_trigger(command),
        name if options.unknown_commands() == RuleSeverity::Deny => {
            bail!("Unknown command: {name}")
        }
        name if options.unknown_commands() == RuleSeverity::Warn => {
            tracing::warn!("Unknown command: {name}");
            Ok(())
        }
        _name if options.unknown_commands() == RuleSeverity::Allow => {
            // Do nothing...;
            Ok(())
        }
        _ => unreachable!("RuleSeverity checks cover all cases of unknown commands."),
    }
}
