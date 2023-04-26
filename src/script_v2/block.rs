use crate::script_v2::{
    line::Line,
    parser::{Parser, Rule},
    ScriptElement,
};
use pest::Parser as PestParser;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    inner: Vec<ScriptElement>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 1)
    }
}

impl Block {
    pub fn new(inner: Vec<ScriptElement>) -> Self {
        Self { inner }
    }

    pub fn empty() -> Self {
        Self::new(Vec::new())
    }

    pub fn elements(&self) -> &[ScriptElement] {
        &self.inner
    }

    pub fn parse(block_str: &str) -> Result<Self, anyhow::Error> {
        let mut pairs = Parser::parse(Rule::Block, block_str)?;
        let block = pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None);
        let inner = block
            .into_inner()
            .map(|pair| match pair.as_rule() {
                Rule::Block => Block::parse(pair.as_str()).map(ScriptElement::block),
                Rule::Line => Line::parse(pair.as_str()).map(ScriptElement::line),
                _ => unreachable!("Blocks can't contain anything other than inner blocks or lines"),
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { inner })
    }

    pub fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        for el in &self.inner {
            match el {
                ScriptElement::Block(block) => block.fmt_with_indent(f, indent + 1)?,
                ScriptElement::Line(line) => {
                    for _ in 0..indent {
                        write!(f, "    ")?;
                    }
                    write!(f, "{line}")?;
                }
                ScriptElement::Comment(comment) => {
                    for _ in 0..indent {
                        write!(f, "    ")?;
                    }
                    write!(f, "{comment}")?;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Block, ScriptElement};
    use crate::script_v2::{command::Command, line::Line};
    use pretty_assertions::assert_eq;

    #[test]
    fn test_single_line_parses_correctly() {
        let input = "    |CHOICE| do the thing\n";
        let expected = Block {
            inner: vec![ScriptElement::line(Line::command(
                Command::parse("|CHOICE| do the thing").unwrap(),
            ))],
        };
        let actual = Block::parse(input).expect("block is valid");
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_multiple_lines_parse_correctly() {
        let input = "    |CHOICE| do the thing
    |CHOICE| do the other thing
    |CHOICE| do the third thing
";
        let expected = Block {
            inner: vec![
                ScriptElement::line(Line::command(
                    Command::parse("|CHOICE| do the thing").unwrap(),
                )),
                ScriptElement::line(Line::command(
                    Command::parse("|CHOICE| do the other thing").unwrap(),
                )),
                ScriptElement::line(Line::command(
                    Command::parse("|CHOICE| do the third thing").unwrap(),
                )),
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
        let input = "    |TEST| A
        |TEST| B
        |TEST| C
";
        let block = Block::parse(input).unwrap();
        print!("{:#?}", block);

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
