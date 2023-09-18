//! # Blocks
//!
//! Blocks are used to organize dialogue. They are indented by 4 spaces and can contain any number of lines or inner blocks. Blocks can be nested to any depth, though you should avoid nesting deeply, as it makes scripts difficult to read. The |CHOICE| and |GOTO| commands show examples of how blocks can be used. When a block is entered, dialogue will continue from the first line of the block. When a block is exited, dialogue will continue from the first line after the block.
//!
//! ```text
//! %START%
//! |SAY| 1
//!     |SAY| 2
//!         |SAY| 3
//!             |SAY| 4
//!                 |SAY| 5
//! %END%
//! ```
//!
//! is equivalent to
//!
//! ```text
//! %START%
//! |SAY| 1
//! |SAY| 2
//! |SAY| 3
//! |SAY| 4
//! |SAY| 5
//! %END%
//! ```
//!
//! Blocks should always be used to organize choices, and commands that result from a choice should be in a block after that choice.

use crate::script::{
    line::Line,
    parser::{Parser, Rule},
    TopLevelElement,
};
use anyhow::bail;
use pest::{iterators::Pair, Parser as PestParser};
use std::fmt;

/// A block in a script, containing a collection of [top level elements](TopLevelElement).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    inner: Vec<TopLevelElement>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 1)
    }
}

impl Block {
    /// Create a new [Block] from a [Vec] of [top level elements](TopLevelElement).
    pub fn new(inner: Vec<TopLevelElement>) -> Self {
        Self { inner }
    }

    /// Create a new empty [Block].
    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    /// Get the [top level elements](TopLevelElement) in this [Block].
    pub fn elements(&self) -> &[TopLevelElement] {
        &self.inner
    }

    /// Parse a [Block] from a string.
    pub fn parse(block_str: &str) -> Result<Self, anyhow::Error> {
        let mut pairs = Parser::parse(Rule::Block, block_str)?;
        let pair = pairs.next().expect("a pair exists");
        assert_eq!(
            pairs.next(),
            None,
            "parsing a block should only ever return one block"
        );

        pair.try_into()
    }

    /// Format this [Block] with the given indentation.
    pub fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        for el in &self.inner {
            match el {
                TopLevelElement::Block(block) => block.fmt_with_indent(f, indent + 1)?,
                TopLevelElement::Line(line) => {
                    for _ in 0..indent {
                        write!(f, "    ")?;
                    }
                    write!(f, "{line}")?;
                }
                TopLevelElement::Comment(comment) => {
                    for _ in 0..indent {
                        write!(f, "    ")?;
                    }
                    write!(f, "{comment}")?;
                }
            }
        }

        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &TopLevelElement> {
        self.inner.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut TopLevelElement> {
        self.inner.iter_mut()
    }

    pub fn into_iter(self) -> impl Iterator<Item = TopLevelElement> {
        self.inner.into_iter()
    }
}

impl std::iter::IntoIterator for Block {
    type Item = TopLevelElement;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl TryFrom<Pair<'_, Rule>> for Block {
    type Error = anyhow::Error;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::Block => {
                let inner = pair
                    .into_inner()
                    .map(|pair| match pair.as_rule() {
                        Rule::Block => Block::try_from(pair).map(TopLevelElement::Block),
                        Rule::Line => Line::try_from(pair).map(TopLevelElement::Line),
                        _ => unreachable!(
                            "Blocks can't contain anything other than inner blocks or lines"
                        ),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                Ok(Self { inner })
            }
            _ => bail!("Pair is not a block: {:#?}", pair),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Block;
    use crate::script::{command::Command, line::Line};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_single_line_parses_correctly() {
        let input = "    |CHOICE| do the thing\n";
        let expected = Block {
            inner: vec![Line::command(Command::parse("|CHOICE| do the thing").unwrap()).into()],
        };
        let actual = Block::parse(input).expect("block is valid");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiple_lines_parse_correctly_1() {
        let input = "    |CHOICE| do the thing
    |CHOICE| do the other thing
    |CHOICE| do the third thing
";
        let expected = Block {
            inner: vec![
                Line::command(Command::parse("|CHOICE| do the thing").unwrap()).into(),
                Line::command(Command::parse("|CHOICE| do the other thing").unwrap()).into(),
                Line::command(Command::parse("|CHOICE| do the third thing").unwrap()).into(),
            ],
        };
        let actual = Block::parse(input).expect("block is valid");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiple_lines_parse_correctly_2() {
        let input = "    |TEST| A1
        |TEST| B1
        |TEST| B2
        |TEST| B3
";
        let expected = Block {
            inner: vec![
                Line::command(Command::parse("|TEST| A1").unwrap()).into(),
                Block {
                    inner: vec![
                        Line::command(Command::parse("|TEST| B1").unwrap()).into(),
                        Line::command(Command::parse("|TEST| B2").unwrap()).into(),
                        Line::command(Command::parse("|TEST| B3").unwrap()).into(),
                    ],
                }
                .into(),
            ],
        };
        let actual = Block::parse(input).expect("block is valid");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nested_blocks_parse_correctly() {
        let input = "    |SAY| First level
        |SAY| Second level
            |SAY| Third level
";
        let expected = Block::new(vec![
            Command::new("SAY", None, Some("First level")).into(),
            Block::new(vec![
                Command::new("SAY", None, Some("Second level")).into(),
                Block::new(vec![Command::new("SAY", None, Some("Third level")).into()]).into(),
            ])
            .into(),
        ]);

        let actual = Block::parse(input).expect("block is valid");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_one_line_round_trip() {
        let input = "    |CHOICE| do the thing\n";
        let block = Block::parse(input).expect("block is valid");
        let output = block.to_string();

        assert_eq!(input, output);
    }

    #[test]
    fn test_multiline_round_trip_1() {
        let input = "    |CHOICE| do the thing
    |CHOICE| do the other thing
    |CHOICE| do the third thing
";
        let block = Block::parse(input).expect("block is valid");
        let output = block.to_string();

        assert_eq!(input, output);
    }

    #[test]
    fn test_multiline_round_trip_2() {
        let input = "    |TEST| A1
        |TEST| B1
        |TEST| B2
        |TEST| B3
";
        let block = Block::parse(input).unwrap();
        print!("{block:#?}");

        let output = block.to_string();

        assert_eq!(input, output);
    }

    #[test]
    fn test_multiline_round_trip_3() {
        let input = "    |SAY| First level
        |SAY| Second level
            |SAY| Third level
";
        let block = Block::parse(input).expect("block is valid");
        let output = block.to_string();

        assert_eq!(input, output);
    }
}
