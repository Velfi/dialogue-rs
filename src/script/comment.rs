//! # Comments
//!
//! Comments start with a `//` and continue until the end of the line. They can be used to annotate a script.
//!
//! ```text
//! // This is a comment
//! ```

use crate::script::parser::{Parser, Rule};
use anyhow::bail;
use pest::iterators::Pair;
use pest::Parser as PestParser;
use std::fmt;

/// A comment in a script.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Comment {
    text: String,
}

impl fmt::Display for Comment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "// {}", self.text)
    }
}

impl Comment {
    /// Create a new [Comment] from a string.
    pub fn new(text: String) -> Self {
        Self { text }
    }

    /// Parse a [Comment] from a string.
    pub fn parse(comment_str: &str) -> Result<Self, anyhow::Error> {
        let mut pairs = Parser::parse(Rule::Comment, comment_str)?;
        let pair = pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None);

        pair.try_into()
    }
}

impl TryFrom<Pair<'_, Rule>> for Comment {
    type Error = anyhow::Error;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::Comment => {
                let mut inner_pairs = pair.into_inner();
                let pair = inner_pairs.next().expect("a pair exists");
                assert!(inner_pairs.next().is_none(), "no other pairs exist");
                let text = pair.as_str().trim().to_owned();

                Ok(Self::new(text))
            }
            _ => bail!("Pair is not a comment: {:#?}", pair),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Comment;

    #[test]
    fn test_comment_parse() {
        let comment = Comment::parse("// This is a comment\n").unwrap();
        assert_eq!(comment.text, "This is a comment");
    }

    #[test]
    fn test_round_trip() {
        let input = "// This is a comment\n";
        let output = Comment::parse(input).unwrap().to_string();
        assert_eq!(input, output);
    }
}
