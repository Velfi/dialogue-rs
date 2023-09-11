mod builder;
// pub mod command_callbacks;
pub mod events;

use std::collections::HashMap;

use anyhow::{bail, Context};
use tracing::trace;

use self::builder::Builder;
use crate::{
    script::{command::Command, element::TopLevelElement, line::Line, Script},
    syntax::END_MARKER_NAME,
    CHOICE_COMMAND,
};

pub struct StateMachine<'s> {
    current_line: usize,
    marker_map: HashMap<&'s str, usize>,
    script: &'s Script,
}

impl<'s> StateMachine<'s> {
    pub fn builder() -> Builder<'s> {
        Default::default()
    }

    pub fn get_tle(&self, line_number: usize) -> Option<&TopLevelElement> {
        self.script.0.get(line_number)
    }

    pub fn current_tle(&self) -> Option<&TopLevelElement> {
        self.get_tle(self.current_line)
    }

    pub fn current_line_number(&self) -> usize {
        self.current_line
    }

    pub fn set_current_line_number(&mut self, line_number: usize) {
        self.current_line = line_number;
    }

    pub fn has_ended(&self) -> bool {
        self.current_tle()
            .and_then(|tle| tle.as_line())
            .and_then(|line| line.as_marker())
            .map(|marker| marker.name() == END_MARKER_NAME)
            .unwrap_or_default()
    }

    pub fn follow_goto(&mut self) -> Result<(), anyhow::Error> {
        let marker = self
            .current_tle()
            .context("expected there to be a current top-level element")?
            .as_line()
            .context("expected top-level element to be a line")?
            .as_marker()
            .context("expected line to be a marker")?;

        if let Some(line) = self.marker_map.get(marker.name()) {
            self.current_line = *line;
            return Ok(());
        }

        bail!("No marker named {marker} exists in this script.");
    }

    pub fn enter_choice_block(&mut self) -> Result<(), anyhow::Error> {
        let block_elements = self
            .current_tle()
            .context("expected there to be a current top-level element")?
            .as_block()
            .context("expected top-level element to be a block")?
            .elements();

        for el in block_elements {
            match el {
                TopLevelElement::Line(line) => {
                    let choice = line.as_command().context("matching a CHOICE command")?;
                    assert_eq!(choice.name(), CHOICE_COMMAND);
                    let choice_text = choice.suffix().context("getting CHOICE text")?;
                }
                TopLevelElement::Block(block) => {
                    todo!()
                }
                TopLevelElement::Comment(_) => continue,
            }
        }

        Ok(())
    }

    pub fn next_command(&mut self) -> Result<Option<&Command>, anyhow::Error> {
        loop {
            let line = self.script.0.get(self.current_line);
            self.current_line += 1;

            match line {
                None => return Ok(None),
                Some(tle) => match tle {
                    TopLevelElement::Block(_block) => {
                        // Blocks will be entered when the user makes a choice.
                        trace!("Skipping block at line #{}", self.current_line);
                        continue;
                    }
                    TopLevelElement::Comment(comment) => {
                        // Comments are ignored by the state machine
                        trace!("Skipping comment \"{}\"", comment);
                        continue;
                    }
                    TopLevelElement::Line(line) => match line {
                        Line::Command(command) => break Ok(Some(command)),
                        Line::Marker(marker) => {
                            // Markers are ignored by the state machine
                            trace!("Skipping marker \"{}\"", marker);
                            continue;
                        }
                    },
                },
            }
        }
    }
}
