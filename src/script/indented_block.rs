use super::{line::Line, parser::Rule};
use crate::error::Error;
use pest::iterators::Pair;

#[derive(Debug)]
pub struct IndentedBlock(pub Vec<Line>);

impl IndentedBlock {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for IndentedBlock {
    fn default() -> Self {
        // Indented blocks are always at least one line long
        Self(Vec::with_capacity(1))
    }
}

impl<'a, 'b> TryFrom<&'a Pair<'b, Rule>> for IndentedBlock {
    type Error = Error;

    fn try_from(pair: &'a Pair<'b, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::IndentedBlock => {
                let pairs = pair.clone().into_inner();
                let mut block = Self::new();

                for pair in pairs {
                    match pair.as_rule() {
                        Rule::Line => {
                            let line = Line::try_from(&pair)?;
                            block.0.push(line);
                        }
                        _ => return Err(Error::unexpected_pair("Line", pair.as_str().to_owned())),
                    }
                }

                Ok(block)
            }
            _ => Err(Error::unexpected_pair(
                "IndentedBlock",
                pair.as_str().to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::IndentedBlock;
    use crate::script::parser::{Rule, ScriptParser};
    use pest::Parser;

    const SAY_COMMANDS: &str = "    SAY Hello, world!
    SAY Goodbye, world!
";

    #[test]
    fn test_parse_block_of_say_commands() {
        let mut pairs =
            ScriptParser::parse(Rule::Block, SAY_COMMANDS).expect("input is a valid script");

        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let block: IndentedBlock = pair.try_into().expect("a new block can be parsed");
        assert_eq!(block.0.len(), 2);
        let mut block_iter = block.0.into_iter();

        let say = block_iter
            .next()
            .expect("line 0 is a SAY command")
            .expect_say();
        assert_eq!(say.text, "Hello, world!");

        let say = block_iter
            .next()
            .expect("line 1 is a SAY command")
            .expect_say();
        assert_eq!(say.text, "Goodbye, world!");

        assert_eq!(block_iter.next(), None, "no more lines exist");
    }

    // const CHOICE_COMMANDS: &str = "    CHOICE This one
    //         SAY You've chosen this one.

    //     CHOICE That one
    //         SAY You've chosen that one.
    // ";

    // #[test]
    // fn test_parse_choices() {
    //     let mut pairs = ScriptParser::parse(Rule::Block, CHOICE_COMMANDS)
    //         .expect("input is a valid script");

    //     let pair = &pairs.next().expect("a pair exists");

    //     let block: IndentedBlock = pair.try_into().expect("a new block can be parsed");

    //     todo!()
    // }
}
