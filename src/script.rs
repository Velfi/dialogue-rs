pub mod choice;
pub mod goto;
pub mod indented_block;
pub mod line;
pub mod marker;
pub mod parser;
pub mod say;

use self::{
    line::Line,
    marker::{END_MARKER, START_MARKER},
    parser::Rule,
};
use crate::{error::Error, script::parser::RULE_LINE};
use pest::iterators::Pair;
use std::fmt;

pub const INDENT: &str = "    ";

#[derive(Debug, Default)]
pub struct Script(pub Vec<Line>);

impl Script {
    pub fn empty() -> Self {
        Default::default()
    }
}

impl<'a, 'b> TryFrom<&'a Pair<'b, Rule>> for Script {
    type Error = Error;

    fn try_from(pair: &'a Pair<'b, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::Script => {
                let pairs = pair.clone().into_inner();
                let mut script = Script::empty();

                for pair in pairs {
                    match pair.as_rule() {
                        Rule::Line => {
                            let line = Line::try_from(&pair)?;
                            script.0.push(line);
                        }
                        Rule::StartOfScript => {
                            let line = Line::Marker(START_MARKER);
                            script.0.push(line);
                        }
                        Rule::EndOfScript => {
                            let line = Line::Marker(END_MARKER);
                            script.0.push(line);
                        }
                        _ => {
                            return Err(Error::unexpected_pair(
                                RULE_LINE,
                                pair.as_rule().to_string(),
                            ));
                        }
                    }
                }

                Ok(script)
            }
            _ => return Err(Error::unexpected_pair("Script", pair.as_str().to_owned())),
        }
    }
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.0 {
            write!(f, "{line}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Script;
    use crate::script::{
        marker::{END_MARKER, START_MARKER},
        parser::{Rule, ScriptParser},
    };
    use pest::Parser;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_daisy_and_luigi_script_can_be_parsed_into_struct() {
        let input = std::fs::read_to_string("example_scripts/daisy-and-luigi.script")
            .expect("example script exists");
        let mut pairs = ScriptParser::parse(Rule::Script, &input).expect("input is a valid script");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let script = Script::try_from(pair).expect("a script can be parsed");
        assert_eq!(
            script.0.len(),
            5,
            "script should have 5 lines but found {:?}",
            script.0
        );

        let mut line_iter = script.0.into_iter();

        let start_marker = line_iter.next().expect("a line exists").expect_marker();
        assert_eq!(start_marker, START_MARKER);

        let line = line_iter.next().expect("a line exists").expect_say();
        assert_eq!(line.text, r#""This is a test.""#);
        assert_eq!(line.subject.unwrap(), "DAISY");

        let line = line_iter.next().expect("a line exists").expect_say();
        assert_eq!(line.text, r#""You got that?""#);
        assert_eq!(line.subject.unwrap(), "DAISY");

        let mut sub_statement_iter = line.sub_statements.into_iter();

        let line = sub_statement_iter
            .next()
            .expect("a line exists")
            .expect_choice();
        assert_eq!(line.text, r#""Come again?""#);

        {
            let mut sub_statement_iter = line.sub_statements.into_iter();

            let line = sub_statement_iter
                .next()
                .expect("a line exists")
                .expect_say();
            assert_eq!(line.text, r#""Come again?""#);
            assert_eq!(line.subject.unwrap(), "LUIGI");

            let line = sub_statement_iter
                .next()
                .expect("a line exists")
                .expect_goto();
            assert_eq!(line.marker.name, "START");

            assert_eq!(sub_statement_iter.next(), None, "no other lines exist");
        }

        let line = sub_statement_iter
            .next()
            .expect("a line exists")
            .expect_choice();
        assert_eq!(line.text, r#""Ah, yes. Thank you.""#);

        {
            let mut sub_statement_iter = line.sub_statements.into_iter();

            let line = sub_statement_iter
                .next()
                .expect("a line exists")
                .expect_say();
            assert_eq!(line.text, r#""Ah, yes. Thank you.""#);
            assert_eq!(line.subject.unwrap(), "LUIGI");

            assert_eq!(sub_statement_iter.next(), None, "no other lines exist");
        }

        let line = line_iter.next().expect("a line exists").expect_say();
        assert_eq!(line.text, r#""You're welcome.""#);
        assert_eq!(line.subject.unwrap(), "DAISY");

        let end_marker = line_iter.next().expect("a line exists").expect_marker();
        assert_eq!(end_marker, END_MARKER);
    }

    #[test]
    fn test_script_to_string_matches_input_script_1() {
        let input = std::fs::read_to_string("example_scripts/daisy-and-luigi.script")
            .expect("example script exists");
        let mut pairs = ScriptParser::parse(Rule::Script, &input).expect("input is a valid script");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let script = Script::try_from(pair).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }

    #[test]
    fn test_script_to_string_matches_input_script_2() {
        let input = std::fs::read_to_string("example_scripts/capital-of-spain.script")
            .expect("example script exists");
        let mut pairs = ScriptParser::parse(Rule::Script, &input).expect("input is a valid script");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let script = Script::try_from(pair).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }

    #[test]
    fn test_script_to_string_matches_input_script_3() {
        let input =
            std::fs::read_to_string("example_scripts/jimi.script").expect("example script exists");
        let mut pairs = ScriptParser::parse(Rule::Script, &input).expect("input is a valid script");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let script = Script::try_from(pair).expect("a script can be parsed");
        // Empty lines are swallowed by the parser, so our test scripts don't have any.
        assert_eq!(input, script.to_string());
    }
}
