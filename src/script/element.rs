use super::{block::Block, command::Command, comment::Comment, line::Line, marker::Marker};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ScriptElement {
    Block(Block),
    Line(Line),
    Comment(Comment),
}

impl fmt::Display for ScriptElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block(block) => write!(f, "{block}"),
            Self::Line(line) => write!(f, "{line}"),
            Self::Comment(comment) => write!(f, "{comment}"),
        }
    }
}

impl From<Line> for ScriptElement {
    fn from(line: Line) -> Self {
        Self::Line(line)
    }
}

impl From<Block> for ScriptElement {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}

impl From<Comment> for ScriptElement {
    fn from(comment: Comment) -> Self {
        Self::Comment(comment)
    }
}

impl From<Marker> for ScriptElement {
    fn from(marker: Marker) -> Self {
        Self::Line(Line::Marker(marker))
    }
}

impl From<Command> for ScriptElement {
    fn from(command: Command) -> Self {
        Self::Line(Line::Command(command))
    }
}

#[cfg(test)]
mod tests {
    use super::ScriptElement;
    use crate::script::{block::Block, comment::Comment, line::Line};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_comment_round_trip() {
        let input = "// hello\n";
        let comment: ScriptElement = Comment::parse(input).unwrap().into();
        let actual = comment.to_string();

        assert_eq!(input, actual);
    }

    #[test]
    fn test_line_round_trip() {
        let input = "|CHOICE| do the thing\n";
        let line: ScriptElement = Line::parse(input).unwrap().into();
        let actual = line.to_string();

        assert_eq!(input, actual);
    }

    #[test]
    fn test_block_round_trip_with_single_line() {
        let input = "    |CHOICE| do the thing\n";
        let block: ScriptElement = Block::parse(input).unwrap().into();
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
        let block: ScriptElement = Block::parse(input).unwrap().into();
        let actual = block.to_string();

        assert_eq!(input, actual);
    }

    #[test]
    fn test_block_round_trip_with_multiple_lines_2() {
        let input = "    |TEST| A
        |TEST| B
        |TEST| C
";
        let block: ScriptElement = Block::parse(input).unwrap().into();
        let actual = block.to_string();

        assert_eq!(input, actual);
    }
}
