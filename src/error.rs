use std::borrow::Cow;

use crate::script::parser::Rule;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Parsing has failed")]
    Parse(#[source] Box<ParseError>),

    #[error("Can't create a '{0}' from a '{1}' pair")]
    UnexpectedPair(Cow<'static, str>, Cow<'static, str>),

    #[error("Can't have an indented block after a '{0}'")]
    SubstatementsNotAllowed(Cow<'static, str>),
}

type PestError = pest::error::Error<Rule>;

impl Error {
    pub fn parse_error(parsing_error: PestError) -> Self {
        Self::Parse(Box::new(ParseError::Invalid(parsing_error)))
    }

    pub fn unexpected_pair(expected: &'static str, got: String) -> Self {
        Self::UnexpectedPair(expected.into(), got.into())
    }

    pub fn substatements_not_allowed(got: String) -> Self {
        Self::SubstatementsNotAllowed(got.into())
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum ParseError {
    #[error("It's invalid IDK man, try harder next time")]
    Invalid(#[source] PestError),
}
