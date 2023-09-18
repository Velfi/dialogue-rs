//! # Markers
//!
//! A script must start with a `%START%` marker and end with a `%END%` marker which determine the start and
//! end of the script, respectively. Markers are written in ALL-CAPS-KEBAB-CASE and delimited by percent
//! symbols. By using the [|GOTO| command], the flow of dialogue can be
//! redirected to just after a marker.

use crate::script::parser::Rule;
use anyhow::bail;
use pest::iterators::Pair;
use std::hash::{Hash, Hasher};
use std::{borrow::Cow, fmt};

/// A marker that can be used as a destination for `GOTO` commands.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Marker(Cow<'static, str>);

impl Hash for Marker {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}%", self.0)
    }
}

impl Marker {
    /// Get the name of the marker.
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl Marker {
    /// Create a new `Marker` from a string.
    pub fn new<T: Into<Cow<'static, str>>>(name: T) -> Self {
        Self(name.into())
    }

    /// Parse a `Marker` from a string.
    pub fn parse(marker_str: &str) -> Result<Self, anyhow::Error> {
        if !marker_str.starts_with('%') || !marker_str.ends_with('%') {
            bail!(
                "Marker must be delimited by percent symbols: {}",
                marker_str
            );
        }

        let marker = marker_str.trim_start_matches('%').trim_end_matches('%');
        Ok(Self(marker.to_owned().into()))
    }
}

pub fn is_valid_marker_name(name: &str) -> bool {
    name.chars().all(|c| c.is_ascii_uppercase() || c == '-')
}

impl TryFrom<Pair<'_, Rule>> for Marker {
    type Error = anyhow::Error;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        match pair.as_rule() {
            Rule::Marker => Marker::parse(pair.as_str()),
            _ => bail!("Pair is not a marker: {:#?}", pair),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Marker;

    #[test]
    fn test_marker_parse() {
        let marker = Marker::parse("%START%").unwrap();
        assert_eq!(marker.name(), "START");
    }

    #[test]
    fn test_round_trip() {
        let marker = Marker::parse("%START%").unwrap();
        assert_eq!(marker.to_string(), "%START%");
    }
}
