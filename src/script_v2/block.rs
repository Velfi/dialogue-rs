use crate::script_v2::{
    line::Line,
    parser::{Parser, Rule},
};
use pest::Parser as PestParser;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    inner: Vec<LineOrBlock>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 1)
    }
}

impl Block {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        for line_or_block in &self.inner {
            match line_or_block {
                LineOrBlock::Block(block) => block.fmt_with_indent(f, indent + 1)?,
                LineOrBlock::Line(line) => {
                    for _ in 0..indent {
                        write!(f, "    ")?;
                    }
                    write!(f, "{line}")?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LineOrBlock {
    Block(Block),
    Line(Line),
}

impl fmt::Display for LineOrBlock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block(block) => write!(f, "{block}"),
            Self::Line(line) => write!(f, "{line}"),
        }
    }
}

impl LineOrBlock {
    pub fn block(block: Block) -> Self {
        Self::Block(block)
    }

    pub fn line(line: Line) -> Self {
        Self::Line(line)
    }
}

impl From<Line> for LineOrBlock {
    fn from(line: Line) -> Self {
        Self::Line(line)
    }
}

impl From<Block> for LineOrBlock {
    fn from(block: Block) -> Self {
        Self::Block(block)
    }
}

impl Block {
    pub fn parse(block_str: &str) -> Result<Self, anyhow::Error> {
        let mut pairs = Parser::parse(Rule::Block, block_str)?;
        let block = pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None);
        let inner = block
            .into_inner()
            .map(|pair| match pair.as_rule() {
                Rule::Block => Block::parse(pair.as_str()).map(LineOrBlock::block),
                Rule::Line => Line::parse(pair.as_str()).map(LineOrBlock::line),
                _ => unreachable!("Blocks can't contain anything other than inner blocks or lines"),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { inner })
    }
}

#[cfg(test)]
mod tests {
    use super::{Block, LineOrBlock};
    use crate::script_v2::{command::Command, line::Line};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_single_line_in_block() {
        let input = "    |CHOICE| do the thing\n";
        let expected = Block {
            inner: vec![LineOrBlock::line(Line::command(
                Command::parse("|CHOICE| do the thing").unwrap(),
            ))],
        };
        let actual = Block::parse(input).expect("block is valid");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_several_lines_in_block() {
        let input = "    |CHOICE| do the thing
    |CHOICE| do the other thing
    |CHOICE| do the third thing
";
        let expected = Block {
            inner: vec![
                LineOrBlock::line(Line::command(
                    Command::parse("|CHOICE| do the thing").unwrap(),
                )),
                LineOrBlock::line(Line::command(
                    Command::parse("|CHOICE| do the other thing").unwrap(),
                )),
                LineOrBlock::line(Line::command(
                    Command::parse("|CHOICE| do the third thing").unwrap(),
                )),
            ],
        };
        let actual = Block::parse(input).expect("block is valid");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_nested_blocks() {
        let input = "    |SAY| First level\n        |SAY| Second level\n";
        let expected = Block {
            inner: vec![
                LineOrBlock::line(Line::command(Command::parse("|SAY| First level").unwrap())),
                LineOrBlock::block(Block {
                    inner: vec![LineOrBlock::line(Line::command(
                        Command::parse("|SAY| Second level").unwrap(),
                    ))],
                }),
            ],
        };
        let actual = Block::parse(input).expect("block is valid");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_round_trip() {
        let input = "    |CHOICE| do the thing\n";
        let block = Block::parse(input).expect("block is valid");
        let output = block.to_string();

        assert_eq!(input, output);
    }

    #[test]
    fn test_multiline_round_trip() {
        let input = "    |CHOICE| do the thing
    |CHOICE| do the other thing
    |CHOICE| do the third thing
";
        let block = Block::parse(input).expect("block is valid");
        let output = block.to_string();

        assert_eq!(input, output);
    }

    #[test]
    fn test_nested_round_trip() {
        let input = "    |SAY| First level
        |SAY| Second level
            |SAY| Third level
";
        let block = Block::parse(input).expect("block is valid");
        print!("{:#?}", block);
        let output = block.to_string();

        assert_eq!(input, output);
    }
}
