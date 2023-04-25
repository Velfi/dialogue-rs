use std::fmt;

use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "scriptv2.pest"]
pub struct Parser;

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Rule {
    pub fn as_str(&self) -> &'static str {
        match self {
            Rule::Script => "Script",
            Rule::Marker => "Marker",
            Rule::Line => "Line",
            Rule::Command => "Command",
            _ => unimplemented!(),
        }
    }
}
