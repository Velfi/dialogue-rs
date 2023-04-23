use super::{line::Line, parser::Rule};
use crate::error::Error;
use pest::iterators::Pair;
use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Marker {
    pub name: Cow<'static, str>,
}

impl Marker {
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self { name: name.into() }
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}%", self.name)
    }
}

impl From<Marker> for Line {
    fn from(marker: Marker) -> Self {
        Self::Marker(marker)
    }
}

pub const START_MARKER: Marker = Marker {
    name: Cow::Borrowed("START"),
};

pub const END_MARKER: Marker = Marker {
    name: Cow::Borrowed("END"),
};

impl<'a, 'b> TryFrom<&'a Pair<'b, Rule>> for Marker {
    type Error = Error;

    fn try_from(pair: &'a Pair<'b, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::Marker => {
                let mut inner = pair.clone().into_inner();
                let name = inner.next().expect("a name exists");
                let name = name.as_str().trim_start_matches('%').trim_end_matches('%');
                let name = Cow::Owned(name.to_owned());
                Ok(Self::new(name))
            }
            Rule::StartMarker | Rule::StartOfScript => Ok(START_MARKER),
            Rule::EndMarker | Rule::EndOfScript => Ok(END_MARKER),
            _ => Err(Error::unexpected_pair("MARKER", pair.as_str().to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Marker;
    use crate::script::parser::{Rule, ScriptParser};
    use pest::Parser;

    #[test]
    fn test_parse_marker_into_struct() {
        let input = "%START%";
        let mut pairs = ScriptParser::parse(Rule::Marker, input).expect("input is a valid MARKER");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let marker: Marker = pair.try_into().expect("a marker can be parsed");
        assert_eq!(marker.name, "START");
    }

    #[test]
    fn test_parse_start_marker_into_struct() {
        let input = "%START%";
        let mut pairs =
            ScriptParser::parse(Rule::StartMarker, input).expect("input is a valid START_MARKER");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let marker: Marker = pair.try_into().expect("a marker can be parsed");
        assert_eq!(marker.name, "START");
    }

    #[test]
    fn test_parse_start_of_script_into_struct() {
        let input = "%START%\n";
        let mut pairs =
            ScriptParser::parse(Rule::StartOfScript, input).expect("input is a valid START_MARKER");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let marker: Marker = pair.try_into().expect("a marker can be parsed");
        assert_eq!(marker.name, "START");
    }

    #[test]
    fn test_parse_end_marker_into_struct() {
        let input = "%END%";
        let mut pairs =
            ScriptParser::parse(Rule::EndMarker, input).expect("input is a valid END_MARKER");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let marker: Marker = pair.try_into().expect("a marker can be parsed");
        assert_eq!(marker.name, "END");
    }

    #[test]
    fn test_parse_end_of_script_into_struct() {
        let input = "%END%";
        let mut pairs =
            ScriptParser::parse(Rule::EndOfScript, input).expect("input is a valid END_MARKER");
        let pair = &pairs.next().expect("a pair exists");
        assert_eq!(pairs.next(), None, "no other pairs exist");

        let marker: Marker = pair.try_into().expect("a marker can be parsed");
        assert_eq!(marker.name, "END");
    }
}
