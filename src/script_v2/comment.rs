use std::fmt;

use crate::script_v2::parser::{Parser, Rule};
use pest::Parser as PestParser;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comment {
    text: String,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "// {}", self.text)
    }
}

impl Comment {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn parse(comment_str: &str) -> Result<Self, anyhow::Error> {
        let mut pairs = Parser::parse(Rule::Comment, comment_str)?;
        let comment = pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None);
        let mut inner_pairs = comment.into_inner();
        let pair = inner_pairs.next().expect("a pair exists");
        assert!(inner_pairs.next().is_none(), "no other pairs exist");
        let text = pair.as_str().trim().to_owned();

        Ok(Self::new(text))
    }
}

#[cfg(test)]
mod tests {
    use super::Comment;

    #[test]
    fn test_comment_parse() {
        let comment = Comment::parse("// This is a comment").unwrap();
        assert_eq!(comment.text, "This is a comment");
    }

    #[test]
    fn test_round_trip() {
        let comment = Comment::parse("// This is a comment").unwrap();
        assert_eq!(comment.to_string(), "// This is a comment");
    }
}
