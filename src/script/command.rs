use crate::script::parser::{Parser, Rule};
use anyhow::bail;
use pest::iterators::Pair;
use pest::Parser as PestParser;
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Command {
    name: Cow<'static, str>,
    prefix: Option<Cow<'static, str>>,
    suffix: Option<Cow<'static, str>>,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(prefix) = &self.prefix {
            write!(f, "{prefix} ")?;
        }

        write!(f, "|{}|", self.name)?;

        if let Some(suffix) = &self.suffix {
            write!(f, " {suffix}")?;
        }

        Ok(())
    }
}

impl Command {
    pub fn new<T: Into<Cow<'static, str>>>(name: T, prefix: Option<T>, suffix: Option<T>) -> Self {
        Self {
            name: name.into(),
            prefix: prefix.map(Into::into),
            suffix: suffix.map(Into::into),
        }
    }

    pub fn parse(command_str: &str) -> Result<Self, anyhow::Error> {
        let mut pairs = Parser::parse(Rule::Command, command_str)?;
        let pair = pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None);

        pair.try_into()
    }
}

impl TryFrom<Pair<'_, Rule>> for Command {
    type Error = anyhow::Error;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::Command => {
                let inner_pairs = pair.into_inner();
                let mut prefix = None;
                let mut command_name = None;
                let mut suffix = None;

                for pair in inner_pairs {
                    match pair.as_rule() {
                        Rule::CommandName => {
                            command_name = Some(
                                pair.as_str()
                                    .trim_start_matches('|')
                                    .trim_end_matches('|')
                                    .to_owned(),
                            );
                        }
                        Rule::Prefix => {
                            if pair.as_str().trim().is_empty() {
                                continue;
                            }

                            prefix = Some(pair.as_str().trim().to_owned());
                        }
                        Rule::Text => {
                            suffix = Some(pair.as_str().trim().to_owned());
                        }
                        _ => unreachable!("hit unexpected pair: {pair}"),
                    }
                }

                let command_name = command_name.expect("all commands have a name");

                Ok(Self::new(command_name, prefix, suffix))
            }
            _ => bail!("Pair is not a command: {:#?}", pair),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Command;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_infix_command_parse() {
        let command_str = "ZELDA |SAY| \"Hello, world!\"";
        let actual = Command::parse(command_str).expect("command is valid");
        let expected = Command::new("SAY", Some("ZELDA"), Some("\"Hello, world!\""));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_postfix_command_parse() {
        let command_str = "|CHOICE| Do the thing";
        let expected = Command::new("CHOICE".to_owned(), None, Some("Do the thing".to_owned()));
        let actual = Command::parse(command_str).expect("command is valid");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_round_trip() {
        let input = "ZELDA |SAY| \"Hello, world!\"";
        let command = Command::parse(input).expect("command is valid");
        let output = command.to_string();
        assert_eq!(input, output);
    }
}
