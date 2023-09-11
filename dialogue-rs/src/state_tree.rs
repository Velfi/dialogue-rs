use std::rc::Rc;

use crate::{
    script::{element::TopLevelElement, line::Line},
    tree::{NodeId, Tree},
    Script,
};
use anyhow::anyhow;

pub struct StateTree {
    tree: Tree<Rc<TopLevelElement>>,
    current_branch: Option<NodeId>,
}

impl StateTree {
    pub fn new(script: Script) -> Self {
        let mut tree = Tree::new();

        tree.add_branches(script.0.into_iter().map(Rc::new));

        // TODO can I make this into a more general method of `Tree`?
        fn grow_branches(branch: &mut Branch<Rc<TopLevelElement>>) {
            match branch.data().as_ref() {
                TopLevelElement::Block(block) => {
                    // branch.add_branches(block.iter().map(Rc::new));
                }
                _ => {}
            }

            for sub_branch in branch.sub_branches_mut() {
                grow_branches(sub_branch);
            }
        }

        for branch in tree.branches_mut() {
            grow_branches(branch);
        }

        let current_branch = tree.first().cloned();

        Self {
            tree,
            current_branch,
        }
    }

    pub fn tick(&mut self) -> anyhow::Result<Option<Rc<TopLevelElement>>> {
        let current_branch = self
            .current_branch
            .as_ref()
            .ok_or(anyhow!("a state tree must have at least one branch"))?;

        todo!()
    }

    pub fn go_to_marker(&mut self, marker: &str) -> anyhow::Result<()> {
        let branch = self
            .tree
            .find_branch_by(|tle| match tle.as_ref() {
                TopLevelElement::Line(Line::Marker(m)) => m.name() == marker,
                _ => false,
            })
            .ok_or(anyhow!(
                "state tree contained no marker with name '{marker}'"
            ))?;
        self.current_branch = Some(branch.clone());

        Ok(())
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
