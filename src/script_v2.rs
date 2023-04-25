pub mod block;
pub mod command;
pub mod comment;
pub mod line;
pub mod marker;
pub mod parser;

use std::fmt;

use block::LineOrBlock;
use parser::{Parser, Rule};
use pest::Parser as PestParser;

use crate::script_v2::{block::Block, line::Line};

#[derive(Debug, Default)]
pub struct ScriptV2(pub Vec<LineOrBlock>);

impl fmt::Display for ScriptV2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line_or_block in &self.0 {
            writeln!(f, "{line_or_block}")?;
        }

        Ok(())
    }
}

impl ScriptV2 {
    pub fn empty() -> Self {
        Default::default()
    }

    pub fn parse(script_str: &str) -> Result<Self, anyhow::Error> {
        let mut pairs = Parser::parse(Rule::Script, script_str)?;
        let script = pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None);
        let inner = script
            .into_inner()
            .map(|pair| match pair.as_rule() {
                Rule::Block => Block::parse(pair.as_str()).map(LineOrBlock::block),
                Rule::Line => Line::parse(pair.as_str()).map(LineOrBlock::line),
                Rule::EOI => Ok(Line::BlankLine.into()),
                _ => unreachable!(
                    "Scripts can't contain anything other than blocks or lines but found {:?}",
                    pair.as_rule()
                ),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(inner))
    }
}

#[cfg(test)]
mod tests {
    use super::ScriptV2;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_daisy_and_luigi_script_can_be_parsed_into_struct() {
        let input = std::fs::read_to_string("example_scripts/daisy-and-luigi.script")
            .expect("example script exists");
        let script = ScriptV2::parse(&input).expect("a script can be parsed");
    }

    #[test]
    fn test_script_to_string_matches_input_script_1() {
        let input = std::fs::read_to_string("example_scripts/daisy-and-luigi.script")
            .expect("example script exists");

        let script = ScriptV2::parse(&input).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }

    #[test]
    fn test_script_to_string_matches_input_script_2() {
        let input = std::fs::read_to_string("example_scripts/capital-of-spain.script")
            .expect("example script exists");

        let script = ScriptV2::parse(&input).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }

    #[test]
    fn test_script_to_string_matches_input_script_3() {
        let input =
            std::fs::read_to_string("example_scripts/jimi.script").expect("example script exists");

        let script = ScriptV2::parse(&input).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }
}
