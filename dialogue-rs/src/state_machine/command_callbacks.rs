//! Commands that can be executed by the state machine.
//!
//! This module contains the commands that can be executed by the state machine.
//! Adding custom commands is possible. They only need to implement the `Command` trait.

mod choice;
mod goto;
mod r#if;
mod say;
mod set;
mod trigger;

pub use choice::Choice;
pub use goto::Goto;
pub use r#if::If;
pub use say::Say;
pub use set::Set;
pub use trigger::Trigger;

use super::State;
use anyhow::anyhow;
use std::collections::HashMap;

#[derive(Debug, Default)]
/// A collection of registered commands
pub struct CommandCallbacks {
    commands: HashMap<&'static str, ()>,
    branching_commands: HashMap<&'static str, ()>,
}

pub type Prefix = Option<String>;
pub type Suffix = String;
pub type CommandArgs = (Prefix, &'static str, Suffix);

pub enum CommandType {
    OneLine,
    Branching,
}

impl CommandCallbacks {
    /// Create a new `CommandCallbacks`.
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
            branching_commands: HashMap::new(),
        }
    }

    /// Get the names of all registered commands.
    pub fn command_names(&self) -> impl Iterator<Item = &'static str> {
        self.commands.keys().chain(self.branching_commands.keys())
    }

    pub fn get_command_info_by_name(&self, name: &'static str) -> Option<CommandType> {
        if self.commands.contains_key(name) {
            Some(CommandType::OneLine)
        } else if self.branching_commands.contains_key(name) {
            Some(CommandType::Branching)
        } else {
            None
        }
    }

    pub fn with_command_callback(mut self, name: &'static str, command_callback: ()) -> Self {
        self.commands.insert(name, command_callback);
        self.command_names.push(name);
        self
    }

    pub fn with_branching_command_callback(
        mut self,
        name: &'static str,
        branching_command_callback: (),
    ) -> Self {
        self.branching_commands
            .insert(name, branching_command_callback);
        self.command_names.push(name);
        self
    }

    /// Given a command name
    pub fn execute(&self, command: CommandArgs, state: &mut State) -> Result<(), anyhow::Error> {
        let (prefix, command_name, suffix) = command;
        self.commands
            .get(command_name)
            .ok_or(anyhow!("Unknown command: {command_name}"))
            .and_then(|command| command.execute(state, prefix, suffix))
    }
}
