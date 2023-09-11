use crate::{
    script::{element::TopLevelElement, line::Line},
    Script, StateMachine,
};
use anyhow::{bail, Context};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Builder<'s> {
    script: Option<&'s Script>,
}

impl<'s> Builder<'s> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(&self) -> Result<StateMachine<'s>, anyhow::Error> {
        let script = self
            .script
            .context("A script is required when building a state machine")?;
        let marker_map = build_marker_map(script)?;

        Ok(StateMachine {
            script,
            current_line: 0,
            marker_map,
        })
    }

    pub fn script(&mut self, script: &'s Script) -> &mut Self {
        self.script = Some(script);
        self
    }
}

fn build_marker_map(script: &Script) -> Result<HashMap<&str, usize>, anyhow::Error> {
    let mut marker_map = HashMap::new();

    for (line_number, el) in script.0.iter().enumerate() {
        if let TopLevelElement::Line(Line::Marker(marker)) = el {
            if marker_map.contains_key(marker.name()) {
                bail!(
                    "Duplicate marker \"{}\" on line {line_number}",
                    marker.name(),
                );
            }
            marker_map.insert(marker.name(), line_number);
        }
    }

    Ok(marker_map)
}
