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

impl ScriptElement {
    pub fn block(block: Block) -> Self {
        Self::Block(block)
    }

    pub fn line(line: Line) -> Self {
        Self::Line(line)
    }

    pub fn comment(comment: Comment) -> Self {
        Self::Comment(comment)
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
    use crate::script_v2::{block::Block, comment::Comment, line::Line};
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
        let input = "    |CHOICE| do the thing
        |CHOICE| do the other thing
            |CHOICE| do the third thing
    |SAY| We're back at the top level now
    |SAY| We're still at the top level
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
