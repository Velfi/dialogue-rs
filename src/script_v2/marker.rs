use std::{borrow::Cow, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Marker(Cow<'static, str>);

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "%{}%", self.0)
    }
}

impl Marker {
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl Marker {
    pub fn new<T: Into<Cow<'static, str>>>(name: T) -> Self {
        Self(name.into())
    }

    pub fn parse(marker_str: &str) -> Result<Self, anyhow::Error> {
        let marker = marker_str.trim_start_matches('%').trim_end_matches('%');
        Ok(Self(marker.to_owned().into()))
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
