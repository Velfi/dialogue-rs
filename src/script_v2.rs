pub mod block;
pub mod command;
pub mod comment;
pub mod element;
pub mod line;
pub mod marker;
pub mod parser;

use self::{block::Block, element::ScriptElement, line::Line};
use parser::{Parser, Rule};
use pest::Parser as PestParser;
use std::fmt;

#[derive(Debug, Default)]
pub struct ScriptV2(pub Vec<ScriptElement>);

impl fmt::Display for ScriptV2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for el in &self.0 {
            write!(f, "{el}")?;
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
                Rule::Block => Block::parse(pair.as_str()).map(ScriptElement::block),
                Rule::Line => Line::parse(pair.as_str()).map(ScriptElement::line),
                _ => unreachable!(
                    "Scripts can't contain anything other than blocks or lines but found {:?}",
                    pair.as_rule()
                ),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self(inner))
    }

    pub fn structure_to_string(&self) -> String {
        let mut output = String::new();

        for el in &self.0 {
            match el {
                ScriptElement::Block(block) => {
                    output.push_str("block {\n");
                    block.elements().iter().for_each(|el| {
                        output.push_str(&format!("    {}", el));
                    });
                    output.push_str("}\n");
                }
                ScriptElement::Line(line) => {
                    output.push_str(&format!("{}", line));
                }
                ScriptElement::Comment(comment) => {
                    output.push_str(&format!("{}\n", comment));
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::ScriptV2;
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
        let script = ScriptV2::parse(input).expect("a script can be parsed");
        let actual = script.to_string();

        assert_eq!(input, actual);
    }

    #[test]
    fn test_script_to_string_matches_input_script_1() {
        let input = std::fs::read_to_string("example_scripts/daisy-and-luigi.script")
            .expect("example script exists");

        let script = ScriptV2::parse(&input).expect("a script can be parsed");
        println!("{}", script.structure_to_string());
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
