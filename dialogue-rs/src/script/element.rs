//! # Top-level elements
//!
//! A top-level element in a script. Either a [Block], [Line], or [Comment].

use super::{block::Block, command::Command, comment::Comment, line::Line, marker::Marker};
use std::fmt;

/// A top-level element in a script. Either a [Block], [Line], or [Comment].
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopLevelElement {
    /// A [Block].
    Block(Block),
    /// A [Line].
    Line(Line),
    /// A [Comment].
    Comment(Comment),
}

impl TopLevelElement {
    pub fn is_block(&self) -> bool {
        matches!(self, Self::Block(_))
    }

    pub fn is_comment(&self) -> bool {
        matches!(self, Self::Comment(_))
    }

    pub fn as_line(&self) -> Option<&Line> {
        match self {
            Self::Line(line) => Some(line),
            _ => None,
        }
    }

    pub fn as_block(&self) -> Option<&Block> {
        match self {
            Self::Block(block) => Some(block),
            _ => None,
        }
    }

    pub fn expect_line(self) -> Line {
        match self {
            Self::Line(line) => line,
            _ => panic!("expected line"),
        }
    }

    pub fn expect_block(self) -> Block {
        match self {
            Self::Block(block) => block,
            _ => panic!("expected block"),
        }
    }
}

impl fmt::Display for TopLevelElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block(block) => write!(f, "{block}"),
            Self::Line(line) => write!(f, "{line}"),
            Self::Comment(comment) => write!(f, "{comment}"),
        }
    }
}

impl From<Line> for TopLevelElement {
    fn from(line: Line) -> Self {
        Self::Line(line)
    }
}

impl From<Block> for TopLevelElement {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}

impl From<Comment> for TopLevelElement {
    fn from(comment: Comment) -> Self {
        Self::Comment(comment)
    }
}

impl From<Marker> for TopLevelElement {
    fn from(marker: Marker) -> Self {
        Self::Line(Line::Marker(marker))
    }
}

impl From<Command> for TopLevelElement {
    fn from(command: Command) -> Self {
        Self::Line(Line::Command(command))
    }
}

#[cfg(test)]
mod tests {
    use super::TopLevelElement;
    use crate::script::{block::Block, comment::Comment, line::Line};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_comment_round_trip() {
        let input = "// hello\n";
        let comment: TopLevelElement = Comment::parse(input).unwrap().into();
        let actual = comment.to_string();

        assert_eq!(input, actual);
    }

    #[test]
    fn test_line_round_trip() {
        let input = "|CHOICE| do the thing\n";
        let line: TopLevelElement = Line::parse(input).unwrap().into();
        let actual = line.to_string();

        assert_eq!(input, actual);
    }

    #[test]
    fn test_block_round_trip_with_single_line() {
        let input = "    |CHOICE| do the thing\n";
        let block: TopLevelElement = Block::parse(input).unwrap().into();
        let actual = block.to_string();

        assert_eq!(input, actual);
    }

    #[test]
    fn test_block_round_trip_with_multiple_lines_1() {
        let input = "    |TEST| A1
        |TEST| B1
            |TEST| C1
        |TEST| B2
";
        let block: TopLevelElement = Block::parse(input).unwrap().into();
        let actual = block.to_string();

        assert_eq!(input, actual);
    }

    #[test]
    fn test_block_round_trip_with_multiple_lines_2() {
        let input = "    |TEST| A
        |TEST| B
        |TEST| C
";
        let block: TopLevelElement = Block::parse(input).unwrap().into();
        let actual = block.to_string();

        assert_eq!(input, actual);
    }
}
