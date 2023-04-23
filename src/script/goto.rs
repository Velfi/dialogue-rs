use std::fmt;

use super::{line::Line, marker::Marker, parser::Rule};
use crate::error::Error;
use pest::iterators::Pair;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Goto {
    pub marker: Marker,
}

impl Goto {
    pub fn new(marker: Marker) -> Self {
        Self { marker }
    }
}

impl fmt::Display for Goto {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GOTO {}", self.marker)
    }
}

impl From<Goto> for Line {
    fn from(goto: Goto) -> Self {
        Self::Goto(goto)
    }
}

impl<'a, 'b> TryFrom<&'a Pair<'b, Rule>> for Goto {
    type Error = Error;

    fn try_from(pair: &'a Pair<'b, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::GotoCommand => {
                let mut inner_pairs = pair.clone().into_inner();
                let marker_pair = &inner_pairs.next().expect("a pair exists");
                assert!(inner_pairs.next().is_none(), "no other pairs exist");
                let marker = Marker::try_from(marker_pair)?;
                Ok(Self::new(marker))
            }
            _ => Err(Error::unexpected_pair("GOTO", pair.as_str().to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Goto;
    use crate::script::parser::{Rule, ScriptParser};
    use pest::Parser;

    #[test]
    fn test_parse_goto() {
        let input = "GOTO %START%";
        let mut pairs =
            ScriptParser::parse(Rule::GotoCommand, input).expect("input is a valid GOTO");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let goto: Goto = pair.try_into().expect("a goto can be parsed");
        assert_eq!(goto.marker.name, "START");
    }
}
