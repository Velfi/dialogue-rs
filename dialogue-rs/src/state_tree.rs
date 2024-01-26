//! A state tree is a tree of [Command]s that can be traversed. It is used to allow running/navigation of a script.
//! State is driven by calling [StateTree::tick]. Calling [StateTree::tick] will return the next command to run, if any.
//! If the next command is multiple choice, the user must call [StateTree::choose] with the index of the choice they want
//! to make.

use crate::{
    script::{command::Command, element::TopLevelElement, line::Line, marker::Marker},
    tree::{NodeId, Tree},
    Script,
};
use anyhow::{anyhow, bail};
use std::{collections::HashMap, rc::Rc};

/// A state tree is a tree of [Command]s that can be traversed. It is used to allow running/navigation of a script.
///
/// State is driven by calling [StateTree::tick]. Calling [StateTree::tick] will return the next command(s) to run, if any.
/// If the next command is multiple choice, the user must call [StateTree::choose] with the index of the choice they want
/// to make.
#[derive(Debug)]
pub struct StateTree {
    tree: Tree<Rc<Command>>,
    /// A mapping of markers to node ids.
    marker_to_node_id: HashMap<String, NodeId>,
    /// When true, [choose] must be called before [tick] can be called again.
    state: TreeState,
    tick_count: usize,
}

#[derive(Debug)]
enum TreeState {
    /// The state tree is waiting for a choice to be made.
    AwaitingChoice(Vec<NodeId>),
    /// The state tree is waiting for a tick to be made.
    AwaitingTick(NodeId),
    /// The state tree has finished running.
    Done,
}

#[derive(Debug)]
pub struct Tick {
    number: usize,
    commands: Vec<Rc<Command>>,
}

impl Tick {
    pub fn number(&self) -> usize {
        self.number
    }

    pub fn commands(&self) -> &[Rc<Command>] {
        &self.commands
    }

    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn get(&self, index: usize) -> Option<&Rc<Command>> {
        self.commands.get(index)
    }
}

impl std::ops::Index<usize> for Tick {
    type Output = Rc<Command>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.commands[index]
    }
}

impl StateTree {
    pub fn new(script: Script) -> Self {
        let mut tree = Tree::new();
        let mut marker_to_node_id = HashMap::new();
        let mut last_id = None;
        let mut marker_awaiting_node_id: Option<Marker> = None;

        fn traverse(
            tree: &mut Tree<Rc<Command>>,
            marker_to_node_id: &mut HashMap<String, NodeId>,
            last_id: &mut Option<NodeId>,
            parent: Option<NodeId>,
            element: TopLevelElement,
            marker_awaiting_node_id: &mut Option<Marker>,
        ) {
            /*
             * A - no_previous line, no parent
             * B - previous_line, no parent
             *   C - previous_line, parent B
             *   D - previous_line, parent B
             * E - previous_line, no parent
             */

            match element {
                TopLevelElement::Line(line) => match line {
                    Line::Command(command) => {
                        let node_id = match parent {
                            Some(parent_id) => tree.push_with_parent(Rc::new(command), parent_id),
                            None => tree.push(Rc::new(command)),
                        };

                        if let Some(marker) = marker_awaiting_node_id.take() {
                            marker_to_node_id.insert(marker.name().to_owned(), node_id);
                        }

                        *last_id = Some(node_id);
                    }
                    Line::Marker(marker) => {
                        debug_assert!(
                            marker_awaiting_node_id.is_none(),
                            "Markers should never follow another marker in the script"
                        );

                        *marker_awaiting_node_id = Some(marker);
                    }
                },
                TopLevelElement::Block(block) => {
                    debug_assert!(last_id.is_some(), "blocks will always have a parent line");
                    for element in block.into_iter() {
                        traverse(
                            tree,
                            marker_to_node_id,
                            last_id,
                            parent,
                            element,
                            marker_awaiting_node_id,
                        );
                    }
                }
                TopLevelElement::Comment(_) => {
                    // Do nothing.
                }
            }
        }

        for element in script.into_iter() {
            traverse(
                &mut tree,
                &mut marker_to_node_id,
                &mut last_id,
                None,
                element,
                &mut marker_awaiting_node_id,
            );
        }

        let state = TreeState::AwaitingTick(tree.first().expect("a first node exists").0);

        Self {
            tree,
            marker_to_node_id,
            state,
            tick_count: 0,
        }
    }

    /// Advance the state tree by one tick, returning one or more commands. If
    /// the next command is a choice, the user must call [StateTree::choose]
    /// with the index of the choice they want to make. If an empty slice is
    /// returned, the script has ended.
    pub fn tick(&mut self) -> anyhow::Result<Tick> {
        /*
         * A - no_previous line, no parent
         * B - previous_line, no parent
         *   C - previous_line, parent B
         *   D - previous_line, parent B
         * E - previous_line, no parent
         */

        match self.state {
            TreeState::AwaitingChoice(_) => {
                bail!("a choice must be made before tick can be called again")
            }
            TreeState::AwaitingTick(current_id) => {
                let number = self.tick_count;
                self.tick_count += 1;

                let child_ids: Vec<_> = self
                    .tree
                    .children_of(current_id)
                    .map(|(child_id, _)| child_id)
                    .collect();
                let command = self
                    .tree
                    .get_by_id(current_id)
                    .ok_or_else(|| anyhow!("no tree branch with ID '{current_id}'"))?;
                let commands = vec![command.clone()];

                if child_ids.is_empty() {
                    if command.is_choice() {
                        self.state = TreeState::AwaitingChoice(child_ids);
                    } else {
                        // When the current node has no children, go to its next sibling.
                        // If it has no siblings, go to the next sibling of its parent.
                        // If it has no parent, then we're done ticking.
                        match self.tree.next(current_id) {
                            Some((next_sibling, _)) => {
                                self.state = TreeState::AwaitingTick(next_sibling);
                            }
                            None => {
                                self.state = TreeState::Done;
                            }
                        }
                    }
                } else {
                    let first_child_node_id = child_ids
                        .first()
                        .expect("collection contains at least one member");
                    let first_child = self
                        .tree
                        .get_by_id(*first_child_node_id)
                        .ok_or_else(|| anyhow!("no tree branch with ID '{current_id}'"))?;

                    if first_child.is_choice() {
                        self.state = TreeState::AwaitingChoice(child_ids);
                    } else {
                        self.state = TreeState::AwaitingTick(*first_child_node_id);
                    }
                }

                Ok(Tick { commands, number })
            }
            // If we're done, return an empty vec. Tick count is not advanced.
            TreeState::Done => Ok(Tick {
                commands: vec![],
                number: self.tick_count,
            }),
        }
    }

    pub fn choose(&mut self, choice: usize) -> anyhow::Result<()> {
        match self.state {
            TreeState::AwaitingTick(_) => {
                bail!("a choice may not be made until one is presented")
            }
            TreeState::AwaitingChoice(ref children) => {
                let child_id = children[choice];
                self.state = TreeState::AwaitingTick(child_id);
                Ok(())
            }
            TreeState::Done => bail!("a choice may not be made after the script has ended"),
        }
    }

    pub fn goto(&mut self, marker: &str) -> anyhow::Result<()> {
        let node_id = self
            .marker_to_node_id
            .get(marker)
            .copied()
            .ok_or_else(|| anyhow::anyhow!("marker does not exist"))?;

        self.state = TreeState::AwaitingTick(node_id);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::StateTree;
    use crate::{script::command::Command, Script};

    // #[test]
    // fn test_script_1_state_tree() {
    //     let input = std::fs::read_to_string("../example-scripts/daisy-and-luigi.script")
    //         .expect("example script exists");

    //     let script = Script::parse(&input).expect("a script can be parsed");
    //     let mut state_tree = StateTree::new(script);

    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("SAY", Some("DAISY"), Some(r#""This is a test.""#)).into()
    //     );
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("SAY", Some("DAISY"), Some(r#""You got that?""#)).into()
    //     );
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 2, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("CHOICE", None, Some("Come again?")).into()
    //     );
    //     assert_eq!(
    //         tick_res[1],
    //         Command::new("CHOICE", None, Some("Ah, yes. Thank you.")).into()
    //     );
    //     state_tree.choose(0).expect("choice succeeds");
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("SAY", Some("LUIGI"), Some("Come again?")).into()
    //     );
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("GOTO", None, Some("START")).into()
    //     );
    //     state_tree.goto("START").expect("goto succeeds");
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("SAY", Some("DAISY"), Some("This is a test.")).into()
    //     );
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("SAY", Some("DAISY"), Some("You got that?")).into()
    //     );
    //     let _ = state_tree.tick().expect("tick succeeds");
    //     state_tree.choose(1).expect("choice succeeds");
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("SAY", Some("DAISY"), Some("You're welcome.")).into()
    //     );
    //     let final_tick = state_tree.tick().expect("tick succeeds");
    //     assert!(final_tick.is_empty());
    // }

    // #[test]
    // fn test_script_2_state_tree() {
    //     let input = std::fs::read_to_string("../example-scripts/capital-of-spain.script")
    //         .expect("example script exists");

    //     let script = Script::parse(&input).expect("a script can be parsed");
    //     let mut state_tree = StateTree::new(script);

    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new(
    //             "SAY",
    //             Some("STANDARDIZED TEST"),
    //             Some("What is the capital of Spain?")
    //         )
    //         .into()
    //     );
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 4, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("CHOICE", None, Some("A: Barcelona")).into()
    //     );
    //     assert_eq!(
    //         tick_res[1],
    //         Command::new("CHOICE", None, Some("B: Madrid")).into()
    //     );
    //     assert_eq!(
    //         tick_res[2],
    //         Command::new("CHOICE", None, Some("C: Bilbao")).into()
    //     );
    //     assert_eq!(
    //         tick_res[3],
    //         Command::new("CHOICE", None, Some("D: Seville")).into()
    //     );
    //     state_tree.choose(1).expect("choice succeeds");
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("SAY", Some("YOU"), Some("The capital of Spain is Madrid")).into()
    //     );
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new("SAY", Some("STANDARDIZED TEST"), Some("Correct!")).into()
    //     );
    //     let tick_res = state_tree.tick().expect("tick succeeds");
    //     assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
    //     assert_eq!(
    //         tick_res[0],
    //         Command::new(
    //             "SAY",
    //             Some("STANDARDIZED TEST"),
    //             Some("Thank you for taking the test.")
    //         )
    //         .into()
    //     );
    //     let final_tick = state_tree.tick().expect("tick succeeds");
    //     assert!(final_tick.is_empty());
    // }
    #[test]
    fn test_script_3_state_tree() {
        let input = std::fs::read_to_string("../example-scripts/jimi.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        let mut state_tree = StateTree::new(script);

        let tick_res = state_tree.tick().expect("tick succeeds");
        assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
        assert_eq!(
            tick_res[0],
            Command::new(
                "SAY",
                Some("JIMI HENDRIX"),
                Some(r#""Excuse me while I kiss the sky.""#)
            )
            .into()
        );
        let tick_res = dbg!(state_tree.tick().expect("tick succeeds"));
        assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
        assert_eq!(
            tick_res[0],
            Command::new("CHOICE", None, Some("Wait patiently.")).into()
        );
        state_tree.choose(0).expect("choice succeeds");
        let tick_res = state_tree.tick().expect("tick succeeds");
        assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
        assert_eq!(
            tick_res[0],
            Command::new("SAY", None, Some("You wait patiently...")).into()
        );
        let tick_res = state_tree.tick().expect("tick succeeds");
        assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
        assert_eq!(
            tick_res[0],
            Command::new(
                "SAY",
                Some("JIMI HENDRIX"),
                Some(
                    r#"Jimi kisses the sky before turning to you and saying "Thank you for your patience.""#
                )
            ).into()
        );
        let final_tick = state_tree.tick().expect("tick succeeds");
        assert!(final_tick.is_empty());
    }

    #[test]
    fn test_script_4_state_tree() {
        let input = std::fs::read_to_string("../example-scripts/three-line.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        let mut state_tree = StateTree::new(script);

        let tick_res = state_tree.tick().expect("tick succeeds");
        assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
        assert_eq!(
            tick_res[0],
            Command::new("SAY", Some("ASHLEY"), Some(r#""Did it work?""#)).into()
        );
        let tick_res = state_tree.tick().expect("tick succeeds");
        assert_eq!(
            tick_res.len(),
            1,
            "Problem with tick {},\nStateTree = {:#?}",
            tick_res.number(),
            state_tree
        );
        assert_eq!(
            tick_res[0],
            Command::new(
                "SAY",
                Some("ZELDA"),
                Some(r#""Well of course it did! Do you think I'm some two-bit hack?""#)
            )
            .into()
        );
        let tick_res = state_tree.tick().expect("tick succeeds");
        assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
        assert_eq!(
            tick_res[0],
            Command::new(
                "SAY",
                Some("ASHLEY"),
                Some(r#""Well, no; But isn't this the first such system you've written?""#)
            )
            .into()
        );
        let final_tick = state_tree.tick().expect("tick succeeds");
        assert!(
            final_tick.is_empty(),
            "Problem with tick {}",
            final_tick.number()
        );
    }

    #[test]
    fn test_script_5_state_tree() {
        let input = std::fs::read_to_string("../example-scripts/two-line.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        let mut state_tree = StateTree::new(script);

        let tick_res = state_tree.tick().expect("tick succeeds");
        assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
        assert_eq!(
            tick_res[0],
            Command::new(
                "SAY",
                None,
                Some("Alone in her office, Zelda thinks aloud:")
            )
            .into()
        );
        let tick_res = state_tree.tick().expect("tick succeeds");
        assert_eq!(tick_res.len(), 1, "Problem with tick {}", tick_res.number());
        assert_eq!(
            tick_res[0],
            Command::new(
                "SAY",
                Some("ZELDA"),
                Some(r#""Sometimes I wonder if I spend too much time on the computer.""#)
            )
            .into()
        );
        let final_tick = state_tree.tick().expect("tick succeeds");
        assert!(final_tick.is_empty());
    }
}
