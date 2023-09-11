//! A parser for the `.script` format.

use pest_derive::Parser;
use std::fmt;

#[derive(Parser)]
#[grammar = "script.pest"]
/// A parser for the `.script` format.
pub(crate) struct Parser;

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Rule {
    /// Get the name of the rule as a string.
    pub fn as_str(&self) -> &'static str {
        use Rule::*;

        match self {
            AllowedSymbols => "AllowedSymbols",
            Char => "Char",
            Text => "Text",
            Marker => "Marker",
            MarkerName => "MarkerName",
            Comment => "Comment",
            Prefix => "Prefix",
            Command => "Command",
            CommandName => "CommandName",
            Line => "Line",
            Block => "Block",
            Script => "Script",
        }
    }
}
