//! Script parsing and representation. Scripts are a collection of lines and blocks.

pub mod block;
pub mod command;
pub mod comment;
pub mod element;
pub mod line;
pub mod marker;
pub(crate) mod parser;

use self::{block::Block, element::TopLevelElement, line::Line};
use anyhow::bail;
use parser::{Parser, Rule};
use pest::{iterators::Pair, Parser as PestParser};
use std::fmt;

/// A collection of lines and blocks, acting as a state machine for dialogue.
#[derive(Debug, Default)]
pub struct Script(pub Vec<TopLevelElement>);

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for el in &self.0 {
            write!(f, "{el}")?;
        }

        Ok(())
    }
}

impl Script {
    /// Create an empty `Script`.
    pub fn empty() -> Self {
        Default::default()
    }

    /// Parse a `Script` from a string.
    pub fn parse(script_str: &str) -> Result<Self, anyhow::Error> {
        let mut pairs = Parser::parse(Rule::Script, script_str)?;
        let pair = pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None);

        pair.try_into()
    }

    pub fn iter(&self) -> impl Iterator<Item = &TopLevelElement> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TopLevelElement> {
        self.0.iter_mut()
    }
}

impl std::iter::IntoIterator for Script {
    type Item = TopLevelElement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl TryFrom<Pair<'_, Rule>> for Script {
    type Error = anyhow::Error;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::Script => {
                let inner = pair
                    .into_inner()
                    .map(|pair| match pair.as_rule() {
                        Rule::Block => Block::parse(pair.as_str()).map(Into::into),
                        Rule::Line => Line::parse(pair.as_str()).map(Into::into),
                        _ => unreachable!(
                        "Scripts can't contain anything other than blocks or lines but found {:?}",
                        pair.as_rule()
                    ),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Self(inner))
            }
            _ => bail!("Pair is not a script: {:#?}", pair),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Script;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_complex_is_nesting_parsed_correctly() {
        let input = "%START%
|TEST| A
    |TEST| B
        |TEST| C
            |TEST| D
    |TEST| E
    |TEST| F
|TEST| G
%END%
";
        let script = Script::parse(input).expect("a script can be parsed");
        let actual = script.to_string();

        assert_eq!(input, actual);
    }

    #[test]
    fn test_script_to_string_matches_input_script_1() {
        let input = std::fs::read_to_string("../example-scripts/daisy-and-luigi.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }

    #[test]
    fn test_script_to_string_matches_input_script_2() {
        let input = std::fs::read_to_string("../example-scripts/capital-of-spain.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }

    #[test]
    fn test_script_to_string_matches_input_script_3() {
        let input = std::fs::read_to_string("../example-scripts/jimi.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }

    #[test]
    fn test_script_to_string_matches_input_script_4() {
        let input = std::fs::read_to_string("../example-scripts/three-line.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(
            input,
            script.to_string(),
            "Script.to_string() didn't match input: {script:#?}"
        );
    }

    #[test]
    fn test_script_to_string_matches_input_script_5() {
        let input = std::fs::read_to_string("../example-scripts/two-line.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }
}
