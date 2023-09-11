use crate::script::command::Command;

pub(crate) fn check_set(set_command: &Command) -> Result<(), anyhow::Error> {
    debug_assert!(set_command.name() == "SET");
    todo!("Implement the check_set function")
}
