use super::INDENT;
use super::{choice::Choice, goto::Goto, marker::Marker, say::Say};
use crate::error::Error;
use crate::script::indented_block::IndentedBlock;
use crate::script::parser::Rule;
use pest::iterators::Pair;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    Say(Say),
    Choice(Choice),
    Goto(Goto),
    Marker(Marker),
}

impl Line {
    pub fn expect_say(self) -> Say {
        match self {
            Self::Say(say) => say,
            _ => panic!("expected a SAY but got {self}"),
        }
    }

    pub fn expect_choice(self) -> Choice {
        match self {
            Self::Choice(choice) => choice,
            _ => panic!("expected a CHOICE but got {self}"),
        }
    }

    pub fn expect_goto(self) -> Goto {
        match self {
            Self::Goto(goto) => goto,
            _ => panic!("expected a GOTO but got {self}"),
        }
    }

    pub fn expect_marker(self) -> Marker {
        match self {
            Self::Marker(marker) => marker,
            _ => panic!("expected a MARKER but got {self}"),
        }
    }

    pub fn is_choice(&self) -> bool {
        matches!(self, Self::Choice(..))
    }

    pub fn is_say(&self) -> bool {
        matches!(self, Self::Say(..))
    }

    pub fn is_goto(&self) -> bool {
        matches!(self, Self::Goto(..))
    }

    pub fn is_marker(&self) -> bool {
        matches!(self, Self::Marker(..))
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_with_indent(f, 0)
    }
}

impl Line {
    fn fmt_with_indent(&self, f: &mut fmt::Formatter<'_>, indent: usize) -> fmt::Result {
        match self {
            Self::Say(say) => {
                writeln!(f, "{}{}", INDENT.repeat(indent), say)?;
                for sub_statement in &say.sub_statements {
                    sub_statement.fmt_with_indent(f, indent + 1)?;
                }

                Ok(())
            }
            Self::Choice(choice) => {
                writeln!(f, "{}{}", INDENT.repeat(indent), choice)?;
                for sub_statement in &choice.sub_statements {
                    sub_statement.fmt_with_indent(f, indent + 1)?;
                }

                Ok(())
            }
            Self::Goto(goto) => writeln!(f, "{}{}", INDENT.repeat(indent), goto),
            Self::Marker(marker) => writeln!(f, "{}{}", INDENT.repeat(indent), marker),
        }
    }
}

impl<'a, 'b> TryFrom<&'a Pair<'b, Rule>> for Line {
    type Error = Error;

    fn try_from(pair: &'a Pair<'b, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::Line => {
                let mut pairs = pair.clone().into_inner();
                let pair = &pairs.next().expect("a pair exists");

                let mut new_line = match pair.as_rule() {
                    Rule::SayCommand => pair.try_into().map(Self::Say),
                    Rule::ChoiceCommand => pair.try_into().map(Self::Choice),
                    Rule::GotoCommand => pair.try_into().map(Self::Goto),
                    Rule::Marker
                    | Rule::StartOfScript
                    | Rule::EndOfScript
                    | Rule::StartMarker
                    | Rule::EndMarker => pair.try_into().map(Self::Marker),
                    _ => Err(Error::unexpected_pair(
                        "Line (inner)",
                        pair.as_str().to_owned(),
                    )),
                }?;

                if let Some(sub_statements) = pairs.next().as_ref().map(IndentedBlock::try_from) {
                    let sub_statements = sub_statements?;

                    match &mut new_line {
                        Self::Choice(choice) => {
                            choice.sub_statements = sub_statements.0;
                        }
                        Self::Say(say) => {
                            say.sub_statements = sub_statements.0;
                        }
                        _ => {
                            return Err(Error::substatements_not_allowed(
                                pair.as_rule().to_string(),
                            ))
                        }
                    }
                }
                assert_eq!(pairs.next(), None, "no other pairs exist");

                Ok(new_line)
            }
            _ => Err(Error::unexpected_pair(
                "Line (outer)",
                pair.as_str().to_owned(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Line;
    use crate::{
        error::Error,
        script::parser::{Rule, ScriptParser},
    };
    use pest::Parser;

    #[test]
    fn test_parse_choice_line() {
        let input = "CHOICE This is no choice at all\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        let choice = line.expect_choice();
        assert_eq!(choice.text, "This is no choice at all");
    }

    #[test]
    fn test_parse_postfix_say_line() {
        let input = "SAY Is it any wonder?\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        let say = line.expect_say();
        assert_eq!(say.text, "Is it any wonder?");
        assert_eq!(say.subject, None);
    }

    #[test]
    fn test_parse_infix_say_line() {
        let input = "G-MAN SAY \"It's time to choose.\"\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        let say = line.expect_say();
        assert_eq!(say.text, r#""It's time to choose.""#);
        assert_eq!(say.subject, Some("G-MAN".to_owned()));
    }

    #[test]
    fn test_parse_marker_line() {
        let input = "%THE-PLACE-TO-BE%\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        let marker = line.expect_marker();
        assert_eq!(marker.name, "THE-PLACE-TO-BE");
    }

    #[test]
    fn test_start_marker_line() {
        let input = "%START%\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        let marker = line.expect_marker();
        assert_eq!(marker.name, "START");
    }

    #[test]
    fn test_parse_goto_line() {
        let input = "GOTO %THE-PLACE-TO-BE%\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        let goto = line.expect_goto();
        assert_eq!(goto.marker.name, "THE-PLACE-TO-BE");
    }

    #[test]
    fn test_lines_may_be_followed_by_multiple_newlines() {
        let input = r#"SAY "Hello, world!"




"#;

        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        let say = line.expect_say();
        assert_eq!(say.text, r#""Hello, world!""#);
    }

    #[test]
    fn test_choice_line_may_be_followed_by_indented_blocks() {
        let input = r#"CHOICE Do the thing
    SAY You proceed to do the thing
    GOTO %END%
"#;

        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        let choice = line.expect_choice();
        assert_eq!(choice.text, "Do the thing");
        assert_eq!(choice.sub_statements.len(), 2);
        let mut substatement_iter = choice.sub_statements.into_iter();

        let say = substatement_iter
            .next()
            .expect("line 0 is a SAY command")
            .expect_say();
        assert_eq!(say.text, "You proceed to do the thing");

        let goto_end = substatement_iter
            .next()
            .expect("line 1 is a GOTO command")
            .expect_goto();
        assert_eq!(goto_end.marker.name, "END");
    }

    #[test]
    fn test_goto_line_may_not_be_followed_by_indented_blocks() {
        let input = r#"GOTO %START%
    SAY "This is not allowed"
"#;

        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let err: Error = TryInto::<Line>::try_into(pair).expect_err("This line cannot be parsed");
        assert_eq!(
            err,
            Error::substatements_not_allowed("GOTO command".to_string())
        );
    }

    #[test]
    fn test_marker_line_may_not_be_followed_by_indented_blocks() {
        let input = r#"%START%
        SAY "This is not allowed"
    "#;

        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let err: Error = TryInto::<Line>::try_into(pair).expect_err("This line cannot be parsed");
        assert_eq!(err, Error::substatements_not_allowed("Marker".to_string()));
    }

    #[test]
    fn test_say_line_to_string() {
        let input = "SAY \"Do the thing\"\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        assert_eq!(line.to_string(), input);
    }

    #[test]
    fn test_choice_line_to_string() {
        let input = "CHOICE Do the thing\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        assert_eq!(line.to_string(), input);
    }

    #[test]
    fn test_goto_line_to_string() {
        let input = "GOTO %THE-PLACE-TO-BE%\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        assert_eq!(line.to_string(), input);
    }

    #[test]
    fn test_marker_line_to_string() {
        let input = "%THE-PLACE-TO-BE%\n";
        let mut pairs = ScriptParser::parse(Rule::Line, input).expect("input is a valid Line");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let line: Line = pair.try_into().expect("a line can be parsed");
        assert_eq!(line.to_string(), input);
    }
}
