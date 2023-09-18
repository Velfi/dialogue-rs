use std::{collections::HashMap, rc::Rc};

use crate::{
    script::{element::TopLevelElement, line::Line, marker::Marker},
    tree::{NodeId, Tree},
    Script,
};
use anyhow::anyhow;

pub struct StateTree {
    tree: Tree<Rc<TopLevelElement>>,
    current_branch: Option<NodeId>,
    marker_to_node_id: HashMap<Marker, NodeId>,
}

impl StateTree {
    pub fn new(script: Script) -> Self {
        let mut tree = Tree::new();
        let mut marker_to_node_id = HashMap::new();
        let mut current_branch = None;

        fn handle_line(
            line: TopLevelElement,
            tree: &mut Tree<Rc<TopLevelElement>>,
            current_branch: &mut Option<NodeId>,
            marker_to_node_id: &mut HashMap<Marker, NodeId>,
        ) {
            let node_id = tree.push(Rc::new(line));
            *current_branch = Some(node_id);
            let data = tree
                .get_by_id(node_id)
                .expect("branch must exist since it was just pushed");
            let line = data
                .as_ref()
                .as_line()
                .expect("we already matched to ensure this is a line");
            match line {
                Line::Command(_) => {
                    // We don't need to do anything for commands
                }
                Line::Marker(m) => {
                    marker_to_node_id.insert(m.clone(), node_id);
                }
            }
        }

        fn handle_block(
            block: TopLevelElement,
            tree: &mut Tree<Rc<TopLevelElement>>,
            current_branch: &mut Option<NodeId>,
            marker_to_node_id: &mut HashMap<Marker, NodeId>,
        ) {
            let block_elements = block.expect_block().into_iter();

            for tle in block_elements {
                // Because blocks may only contain lines, we can safely unwrap here
                handle_line(tle, tree, current_branch, marker_to_node_id)
            }
        }

        for element in script.0.into_iter() {
            match element {
                line @ TopLevelElement::Line(_) => {
                    handle_line(line, &mut tree, &mut current_branch, &mut marker_to_node_id)
                }
                block @ TopLevelElement::Block(_) => handle_block(
                    block,
                    &mut tree,
                    &mut current_branch,
                    &mut marker_to_node_id,
                ),
                _comment @ TopLevelElement::Comment(_) => {
                    todo!()
                }
            }
        }

        let current_branch = tree.first().map(|(id, _)| id);

        Self {
            tree,
            current_branch,
            marker_to_node_id,
        }
    }

    pub fn tick(&mut self) -> anyhow::Result<Option<Rc<TopLevelElement>>> {
        let current_branch = self
            .current_branch
            .as_ref()
            .ok_or(anyhow!("a state tree must have at least one branch"))?;

        todo!()
    }

    pub fn choose(&mut self, choice: NodeId) -> anyhow::Result<()> {
        todo!()
    }

    pub fn goto(&mut self, marker: &str) -> anyhow::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::StateTree;
    use crate::Script;

    #[test]
    fn test_script_to_string_matches_input_script_1() {
        let input = std::fs::read_to_string("../example-scripts/daisy-and-luigi.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        let state_graph = StateTree::new(script);
        todo!("write daisy-and-luigi test");
    }

    #[test]
    fn test_script_to_string_matches_input_script_2() {
        let input = std::fs::read_to_string("../example-scripts/capital-of-spain.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        let state_graph = StateTree::new(script);
        todo!("write capital-of-spain test");
    }

    #[test]
    fn test_script_to_string_matches_input_script_3() {
        let input = std::fs::read_to_string("../example-scripts/jimi.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        let state_graph = StateTree::new(script);
        todo!("write jimi test");
    }

    #[test]
    fn test_script_to_string_matches_input_script_4() {
        let input = std::fs::read_to_string("../example-scripts/three-line.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        let state_graph = StateTree::new(script);
        todo!("write three-line test");
    }

    #[test]
    fn test_script_to_string_matches_input_script_5() {
        let input = std::fs::read_to_string("../example-scripts/two-line.script")
            .expect("example script exists");

        let script = Script::parse(&input).expect("a script can be parsed");
        let state_graph = StateTree::new(script);
        todo!("write two-line test");
    }
}
