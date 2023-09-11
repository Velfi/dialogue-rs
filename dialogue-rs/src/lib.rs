#![deny(
    // missing_docs,
    rustdoc::missing_crate_level_docs,
    unreachable_pub,
    rust_2018_idioms
)]

//! # Dialogue Script - A language for writing interactable dialogue
//!
//! Dialogue Script is a language for writing dialogue that is both human-readable and easy to parse. The
//! dialogue can be thought of as a state machine, where each line of dialogue is a state and each indented
//! block is a transition to a sub-state.
//!
//! ## Example
//!
//! ```text
//! %START%
//!     ZELDA |SAY| "Hey there!"
//!     ZELDA |SAY| "How are you?"
//!     YOU |SAY| "I'm doing well, thanks!"
//!     ZELDA |SAY| "Glad to hear it!"
//! %END%
//! ```
//!
//! _More example scripts are located in the `example-scripts/` directory._
//!
//! ## Syntax
//!
//! The syntax of a script consists of lines, blocks, and comments. Lines are the main component of a script, and
//! blocks and comments are used to organize and annotate the script.
//!
//! Lines in a script are either [Markers](#Markers) or [Commands](#Commands). They can include any letters
//! or symbols, except for pipes _(&nbsp;|&nbsp;)_, as those are used to delimit commands.
//!
//! ### Markers
//!
//! A script must start with a `%START%` marker and end with a `%END%` marker which determine the start and
//! end of the script, respectively. Markers are written in ALL-CAPS-KEBAB-CASE and delimited by percent
//! symbols. By using the [|GOTO| command], the flow of dialogue can be
//! redirected to just after a marker.
//!
//! ### Commands
//!
//! Commands are written in ALL-CAPS-KEBAB-CASE and delimited by pipes. Commands can have 'prefixes' and
//! 'suffixes'. Several built-in commands are supported, and _(in most cases)_ it's easy to extend the language with custom
//! commands.
//!
//! ```text
//! PREFIX |COMMAND| SUFFIX
//! ```
//!
//! #### SAY
//!
//! The `|SAY|` command is used to display dialogue. When an optional prefix is supplied, it represents the
//! speaker. A suffix is required, and it represents the dialogue to be displayed.
//!
//! ```text
//! ZELDA |SAY| "Hey there!"
//! ```
//!
//! The above command would display the text `"Hey there!"` in a dialogue box with the speaker `ZELDA`.
//! Quotes around text are interpreted as text, and aren't required.
//!
//! ```text
//! |SAY| It was a dark and stormy night...
//! ```
//!
//! The above command would display the text `It was a dark and stormy night...` in a dialogue box with no speaker.
//!
//! #### CHOICE
//!
//! The `|CHOICE|` command is used to declare a list of choices. A suffix is required, and it represents a choice
//! that the player could make. A prefix is not allowed.
//!
//! ```text
//! |CHOICE| "Yes"
//!     PLAYER |SAY| "Yes"
//!     |GOTO| %CONTINUE%
//! |CHOICE| "No"
//!     PLAYER |SAY| "No"
//!     |GOTO| %GO-BACK%
//! ```
//!
//! The above command would display a list of choices with the text `"Yes"` and `"No"`. If the player chose
//! `"Yes"`, `PLAYER  - "Yes"` would be printed and the dialogue would continue at the `%CONTINUE%` marker. If the player chose `"No"`, `PLAYER - "No"` would be printed and the dialogue would continue at the `%GO-BACK%` marker.
//!
//! #### GOTO
//!
//! The `|GOTO|` command is used to redirect the flow of dialogue to a marker. A suffix is required, and it
//! must be a valid marker. A prefix is not allowed.
//!
//! ```text
//! |GOTO| %START%
//! ```
//!
//! The above command would redirect the flow of dialogue back to the `%START%` marker. Used in concert with the
//! `|CHOICE|` command, branching dialogue is possible. Below is an example of branching dialogue using `|GOTO|`.
//!
//! ```text
//! %START%
//! Zelda |SAY| "This is a test."
//! Zelda |SAY| "You got that?"
//!     |CHOICE| What?
//!         You |SAY| "Come again?"
//!         |GOTO| %START%
//!     |CHOICE| Yes
//!         You |SAY| "Ah, yes. Thank you."
//! Zelda |SAY| "You're welcome."
//! %END%
//! ```
//!
//! In the above dialogue, if the player chose `What?`, the dialogue would loop back to the start.
//! If the player chose `Yes`, the dialogue would continue to the end.
//!
//! ### Blocks
//!
//! Blocks are used to organize dialogue. They are indented by 4 spaces and can contain any number of lines or inner blocks. Blocks can be nested to any depth, though you should avoid nesting deeply, as it makes scripts difficult to read. The |CHOICE| and |GOTO| commands show examples of how blocks can be used. When a block is entered, dialogue will continue from the first line of the block. When a block is exited, dialogue will continue from the first line after the block.
//!
//! ```text
//! %START%
//! |SAY| 1
//!     |SAY| 2
//!         |SAY| 3
//!             |SAY| 4
//!                 |SAY| 5
//! %END%
//! ```
//!
//! is equivalent to
//!
//! ```text
//! %START%
//! |SAY| 1
//! |SAY| 2
//! |SAY| 3
//! |SAY| 4
//! |SAY| 5
//! %END%
//! ```
//!
//! Blocks should always be used to organize choices, and commands that result from a choice should be in a block after that choice.
//!
//! ### Comments
//!
//! Comments start with a `//` and continue until the end of the line. They can be used to annotate a script.
//!
//! ```text
//! // This is a comment
//! ```
//!
//! ## License
//!
//! This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

// pub mod state_machine;
pub mod script;
pub mod state_tree;
pub mod syntax;
pub mod tree;

// pub use state_machine::StateMachine;
pub use script::Script;
pub use syntax::commands::{
    CHOICE_COMMAND, GOTO_COMMAND, IF_COMMAND, SAY_COMMAND, SET_COMMAND, TRIGGER_COMMAND,
};
pub use syntax::{check_syntax, check_syntax_with_options, SyntaxCheckerOptions};
