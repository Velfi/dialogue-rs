use super::{line::Line, parser::Rule};
use crate::error::Error;
use pest::iterators::Pair;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice {
    pub text: String,
    pub sub_statements: Vec<Line>,
}

impl Choice {
    pub fn new(text: String) -> Self {
        Self {
            text,
            sub_statements: Vec::new(),
        }
    }
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CHOICE {}", self.text)
    }
}

impl From<Choice> for Line {
    fn from(choice: Choice) -> Self {
        Self::Choice(choice)
    }
}

impl<'a, 'b> TryFrom<&'a Pair<'b, Rule>> for Choice {
    type Error = Error;

    fn try_from(pair: &'a Pair<'b, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::ChoiceCommand => {
                let mut inner_pairs = pair.clone().into_inner();
                let text_pair = &inner_pairs.next().expect("a pair exists");
                assert!(inner_pairs.next().is_none(), "no other pairs exist");
                let text = text_pair.as_str().to_owned();
                Ok(Self::new(text))
            }
            _ => Err(Error::unexpected_pair("CHOICE", pair.as_str().to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Choice;
    use crate::script::parser::{Rule, ScriptParser};
    use pest::Parser;

    #[test]
    fn test_parse_choice() {
        let input = "CHOICE It's time to choose.";
        let mut pairs =
            ScriptParser::parse(Rule::ChoiceCommand, input).expect("input is a valid CHOICE");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let choice: Choice = pair.try_into().expect("a choice can be parsed");
        assert_eq!(choice.text, "It's time to choose.");
    }
}
