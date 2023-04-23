use super::line::Line;
use crate::error::Error;
use crate::script::parser::Rule;
use pest::iterators::Pair;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Say {
    pub subject: Option<String>,
    pub text: String,
    pub sub_statements: Vec<Line>,
}

impl Say {
    pub fn new(subject: Option<String>, text: String) -> Self {
        Self {
            subject,
            text,
            sub_statements: Vec::new(),
        }
    }
}

impl fmt::Display for Say {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subject {
            Some(subject) => write!(f, "{} SAY {}", subject, self.text),
            None => write!(f, "SAY {}", self.text),
        }
    }
}

impl From<Say> for Line {
    fn from(say: Say) -> Self {
        Self::Say(say)
    }
}

impl<'a, 'b> TryFrom<&'a Pair<'b, Rule>> for Say {
    type Error = Error;

    fn try_from(pair: &'a Pair<'b, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::SayCommand => {
                let inner_pair = pair.clone().into_inner().next().expect("a pair exists");
                match inner_pair.as_rule() {
                    Rule::InfixSayCommand => {
                        let mut inner_pairs = inner_pair.into_inner();
                        let subject_pair = &inner_pairs.next().expect("a pair exists");
                        let text_pair = &inner_pairs.next().expect("a pair exists");
                        assert!(inner_pairs.next().is_none(), "no other pairs exist");
                        let subject = subject_pair.as_str().to_owned();
                        let text = text_pair.as_str().to_owned();
                        Ok(Self::new(Some(subject), text))
                    }
                    Rule::PostfixSayCommand => {
                        let mut inner_pairs = inner_pair.into_inner();
                        let text_pair = &inner_pairs.next().expect("a pair exists");
                        assert!(inner_pairs.next().is_none(), "no other pairs exist");
                        let text = text_pair.as_str().to_owned();
                        Ok(Self::new(None, text))
                    }
                    _ => unreachable!("SAY command is either infix or postfix"),
                }
            }
            _ => Err(Error::unexpected_pair("SAY", pair.as_str().to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Say;
    use crate::script::parser::{Rule, ScriptParser};
    use pest::Parser;

    #[test]
    fn test_parse_say_without_subject() {
        let input = "SAY Oh, what a beautiful mornin'";
        let mut pairs = ScriptParser::parse(Rule::SayCommand, input).expect("input is a valid SAY");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let say: Say = pair.try_into().expect("a say can be parsed");
        assert_eq!(say.text, "Oh, what a beautiful mornin'");
        assert_eq!(say.subject, None);
    }

    #[test]
    fn test_parse_say_with_subject() {
        let input = "Curly McLain SAY Oh, what a beautiful day";
        let mut pairs = ScriptParser::parse(Rule::SayCommand, input).expect("input is a valid SAY");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let say: Say = pair.try_into().expect("a say can be parsed");
        assert_eq!(say.text, "Oh, what a beautiful day");
        assert_eq!(say.subject.expect("subject exists"), "Curly McLain");
    }
}
