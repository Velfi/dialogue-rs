//! Syntax checking for [Script]s.

pub(crate) mod commands;
mod options;

pub use options::{RuleSeverity, SyntaxCheckerOptions};

use crate::script::{block::Block, element::TopLevelElement, line::Line, marker::Marker, Script};
use anyhow::bail;
use commands::check_command;
use itertools::Itertools;
use once_cell::sync::Lazy;

pub const START_MARKER_NAME: &str = "START";
pub const END_MARKER_NAME: &str = "END";
pub const START_MARKER: Lazy<Marker> = Lazy::new(|| Marker::new(START_MARKER_NAME));
pub const END_MARKER: Lazy<Marker> = Lazy::new(|| Marker::new(END_MARKER_NAME));

pub fn check_syntax(script: &Script) -> Result<(), anyhow::Error> {
    check_syntax_with_options(script, &SyntaxCheckerOptions::default())
}

/// Check the syntax of a [Script].
pub fn check_syntax_with_options(
    script: &Script,
    options: &SyntaxCheckerOptions,
) -> Result<(), anyhow::Error> {
    // Allocate space for the expected START and END markers.
    let mut markers_seen = Vec::with_capacity(2);
    let mut script_iter = script.iter().peekable();

    // Skip any comments at the start of the script. Once a non-comment is found, check that it's a START marker.
    // Otherwise, return an error.
    loop {
        match script_iter.next() {
            Some(TopLevelElement::Comment(_)) => {
                continue;
            }
            Some(TopLevelElement::Line(Line::Marker(marker))) if marker.name() == START_MARKER_NAME => {
                // Script starts with expected START marker
                markers_seen.push(marker.clone());
                break;
            }
            Some(TopLevelElement::Line(Line::Marker(marker))) => {
                let marker_name = marker.name();
                bail!("Marker {marker_name} shouldn't be declared before the {START_MARKER_NAME} marker.");
            }
            Some(other) => bail!("Script must start with a {START_MARKER_NAME} marker but found {other:#?}"),
            None => bail!("Script is empty. Valid scripts must have {START_MARKER_NAME} and {END_MARKER_NAME} markers, and at least one command."),
        }
    }

    // Skip any comments after the START marker. Once a non-comment is found, check that it's a line containing a command.
    loop {
        match script_iter.peek() {
            Some(TopLevelElement::Comment(_)) => {
                continue;
            }
            Some(TopLevelElement::Line(Line::Marker(marker))) if marker.name() == END_MARKER_NAME => {
                bail!(
                    "Scripts must contain at least one command between the {START_MARKER_NAME} and {END_MARKER_NAME} markers."
                );
            }
            Some(TopLevelElement::Line(Line::Marker(marker))) => {
                let marker_name = marker.name();
                bail!("Marker {marker_name} shouldn't immediately follow the {START_MARKER_NAME} marker.");
            }
            None => bail!("The {START_MARKER_NAME} marker should be followed by at least one command"),
            _ => break,
        }
    }

    let pairwise_iter = script_iter.tuples::<(_, _)>();
    check_syntax_of_pairwise_elements(pairwise_iter, &mut markers_seen, options)
}

fn check_block(
    block: &Block,
    markers_seen: &mut Vec<Marker>,
    options: &SyntaxCheckerOptions,
) -> Result<(), anyhow::Error> {
    let block_iter = block.iter().peekable();

    let pairwise_iter = block_iter.tuples::<(_, _)>();
    check_syntax_of_pairwise_elements(pairwise_iter, markers_seen, options)
}

fn check_syntax_of_pairwise_elements<'a, 'b>(
    pairwise_iter: impl Iterator<Item = (&'a TopLevelElement, &'b TopLevelElement)>,
    markers_seen: &mut Vec<Marker>,
    options: &SyntaxCheckerOptions,
) -> Result<(), anyhow::Error> {
    for (current, next) in pairwise_iter {
        match current {
            TopLevelElement::Line(Line::Command(command)) => {
                check_command(
                    &command,
                    match next {
                        TopLevelElement::Block(block) => Some(&block),
                        TopLevelElement::Line(_) | TopLevelElement::Comment(_) => None,
                    },
                    options,
                )?;
            }
            TopLevelElement::Line(Line::Marker(marker)) if markers_seen.contains(marker) => {
                bail!("Marker {marker} shouldn't be declared more than once.");
            }
            TopLevelElement::Line(Line::Marker(marker)) => {
                markers_seen.push(marker.clone());
            }
            TopLevelElement::Block(block) => {
                check_block(&block, markers_seen, options)?;
            }
            TopLevelElement::Comment(_) => {
                // Comments are allowed anywhere
                continue;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{check_syntax, Script};

    #[test]
    #[should_panic = "Script is empty. Valid scripts must have START and END markers, and at least one command."]
    fn test_syntax_of_empty_script_is_invalid() {
        let script = Script(vec![]);
        let _ = check_syntax(&script).unwrap();
    }

    #[test]
    fn test_syntax_of_example_script_is_valid_1() {
        let input = std::fs::read_to_string("../example-
        scripts/capital-of-spain.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        check_syntax(&script).expect("syntax of example script is correct");
    }

    #[test]
    fn test_syntax_of_example_script_is_valid_2() {
        let input = std::fs::read_to_string("../example-
        scripts/daisy-and-luigi.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        check_syntax(&script).expect("syntax of example script is correct");
    }

    #[test]
    fn test_syntax_of_example_script_is_valid_3() {
        let input = std::fs::read_to_string("../example-
        scripts/jimi.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        check_syntax(&script).expect("syntax of example script is correct");
    }

    #[test]
    fn test_syntax_of_example_script_is_valid_4() {
        let input = std::fs::read_to_string("../example-
        scripts/three-line.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        check_syntax(&script).expect("syntax of example script is correct");
    }

    #[test]
    fn test_syntax_of_example_script_is_valid_5() {
        let input = std::fs::read_to_string("../example-
        scripts/two-line.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        check_syntax(&script).expect("syntax of example script is correct");
    }

    #[test]
    fn top_level_choices_are_ok() {
        let input = std::fs::read_to_string("../test-scripts/top-level-choices.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        check_syntax(&script).expect("syntax of example script is correct");
    }
}
