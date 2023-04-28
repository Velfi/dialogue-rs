use crate::script::{
    command::Command,
    comment::Comment,
    marker::Marker,
    parser::{Parser, Rule},
};
use anyhow::bail;
use pest::{iterators::Pair, Parser as PestParser};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Line {
    Command(Command),
    Comment(Comment),
    Marker(Marker),
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Command(command) => writeln!(f, "{command}"),
            Self::Comment(comment) => writeln!(f, "{comment}"),
            Self::Marker(marker) => writeln!(f, "{marker}"),
        }
    }
}

impl From<Command> for Line {
    fn from(command: Command) -> Self {
        Self::Command(command)
    }
}

impl From<Comment> for Line {
    fn from(comment: Comment) -> Self {
        Self::Comment(comment)
    }
}

impl From<Marker> for Line {
    fn from(marker: Marker) -> Self {
        Self::Marker(marker)
    }
}

impl Line {
    pub fn command(command: Command) -> Self {
        Self::Command(command)
    }

    pub fn parse(line_str: &str) -> Result<Self, anyhow::Error> {
        let mut pairs = Parser::parse(Rule::Line, line_str)?;
        let pair = pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None);

        pair.try_into()
    }
}

impl TryFrom<Pair<'_, Rule>> for Line {
    type Error = anyhow::Error;

    fn try_from(pair: Pair<Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::Line => {
                let mut pairs = pair.into_inner();
                let pair = pairs.next().expect("a pair exists");
                assert_eq!(pairs.next(), None);

                match pair.as_rule() {
                    Rule::Command => Command::parse(pair.as_str()).map(Self::Command),
                    Rule::Comment => Comment::parse(pair.as_str()).map(Self::Comment),
                    Rule::Marker => Marker::parse(pair.as_str()).map(Self::Marker),
                    _ => unreachable!("Lines can't contain anything other than commands, comments, markers, or blank lines"),
                }
            }
            _ => bail!("Pair is not a line: {:#?}", pair),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::script::command::Command;

    use super::Line;

    #[test]
    fn test_parse_line() {
        let expected = Line::Command(Command::new(
            "SAY",
            Some("Zelda"),
            Some("\"I don't actually have anything to say\""),
        ));
        let actual = Line::parse("Zelda |SAY| \"I don't actually have anything to say\"\n\n")
            .expect("command is valid");

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_round_trip() {
        let input = "|SAY| Does this work?\n";
        let line = Line::parse(input).expect("line is valid");
        let output = line.to_string();

        assert_eq!(input, output);
    }
}
