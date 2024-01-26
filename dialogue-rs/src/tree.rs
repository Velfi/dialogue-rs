use core::fmt;
use std::sync::atomic::{AtomicU16, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(u16);

static ID_GENERATOR: AtomicU16 = AtomicU16::new(0);

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Opaque Node ID")
    }
}

fn get_next_id() -> NodeId {
    NodeId(ID_GENERATOR.fetch_add(1, Ordering::SeqCst))
}

#[derive(Debug)]
pub struct Tree<T> {
    branches: Vec<Branch<T>>,
    // A is parent of B
    parent_child_relationships: Vec<(NodeId, NodeId)>,
}

impl<T> Default for Tree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            branches: Vec::new(),
            parent_child_relationships: Vec::new(),
        }
    }

    /// Returns ID and data of the first branch in the tree.
    ///
    /// If the tree is empty, returns `None`.
    pub fn first(&self) -> Option<(NodeId, &T)> {
        self.branches
            .first()
            .map(|branch| (branch.id, &branch.data))
    }

    /// Given an ID, return the ID and data of the next branch in the tree.
    ///
    /// Search for the next ID is depth first. If the given ID has no siblings,
    /// then the next sibling of the parent is returned.
    pub fn next(&self, id: NodeId) -> Option<(NodeId, &T)> {
        self.next_sibling_of(id).or_else(|| {
            self.parent_of(id)
                .and_then(|(parent_id, _)| self.next(parent_id))
        })
    }

    /// Adds a branch to the tree.
    ///
    /// Returns the ID of the newly added branch.
    pub fn push(&mut self, data: T) -> NodeId {
        let id = get_next_id();
        self.branches.push(Branch::new(id, data));

        id
    }

    /// Adds a branch to the tree with a parent.
    ///
    /// Returns the ID of the newly added branch. Panics if the parent does not exist.
    pub fn push_with_parent(&mut self, data: T, parent: NodeId) -> NodeId {
        let id = get_next_id();
        self.branches.push(Branch::new(id, data));

        debug_assert!(
            self.branches.iter().any(|branch| branch.id == parent),
            "parent node must already exist in tree"
        );
        self.parent_child_relationships.push((parent, id));

        id
    }

    /// Given a `NodeId`, return the data held by that node (assuming that the node exists.)
    pub fn get_by_id(&self, id: NodeId) -> Option<&T> {
        self.branches
            .iter()
            .find(|branch| branch.id == id)
            .map(|branch| &branch.data)
    }

    /// Given a parent node's `NodeId`, return the IDs and data of all that parent's child nodes.
    pub fn children_of(&self, id: NodeId) -> impl DoubleEndedIterator<Item = (NodeId, &T)> + '_ {
        self.parent_child_relationships
            .iter()
            .filter(move |(parent_id, _)| *parent_id == id)
            .filter_map(move |(_, child_id)| {
                self.get_by_id(*child_id).map(|data| (*child_id, data))
            })
    }

    pub fn parent_of(&self, id: NodeId) -> Option<(NodeId, &T)> {
        self.parent_child_relationships
            .iter()
            .find(|(_, child_id)| *child_id == id)
            .and_then(|(parent_id, _)| self.get_by_id(*parent_id).map(|data| (*parent_id, data)))
    }

    pub fn siblings_of(&self, id: NodeId) -> Option<impl DoubleEndedIterator<Item = (NodeId, &T)>> {
        self.parent_of(id).map(|(parent_id, _)| {
            self.children_of(parent_id)
                .filter(move |(child_id, _)| *child_id != id)
        })
    }

    pub fn next_sibling_of(&self, id: NodeId) -> Option<(NodeId, &T)> {
        // No parent? No way to know the next sibling.
        // TODO: I think the root can have siblings, but it has no parent. How do we handle this?
        let (parent_id, _) = self.parent_of(id)?;
        let mut children = self.children_of(parent_id);
        let mut found_index = false;

        loop {
            match children.next() {
                Some((child_id, _)) if found_index => {
                    break Some((child_id, self.get_by_id(child_id)?))
                }
                Some((child_id, _)) if child_id == id => found_index = true,
                None => break None,
                _ => continue,
            }
        }
    }

    pub fn previous_sibling_of(&self, id: NodeId) -> Option<(NodeId, &T)> {
        // No parent? No way to know the previous sibling.
        // TODO: I think the root can have siblings, but it has no parent. How do we handle this?
        let (parent_id, _) = self.parent_of(id)?;
        let mut children = self.children_of(parent_id).rev();
        let mut previous_sibling = None;

        loop {
            match children.next() {
                Some((child_id, _)) if child_id == id => break previous_sibling,
                Some((child_id, _)) => {
                    previous_sibling = Some((child_id, self.get_by_id(child_id)?))
                }
                None => break None,
            }
        }
    }

    /// Returns the first branch in the tree that matches the given predicate.
    ///
    /// If no branch matches the predicate, returns `None`.
    pub fn find_by<F>(&self, f: F) -> Option<(NodeId, &T)>
    where
        F: Fn(&T) -> bool,
    {
        self.branches
            .iter()
            .find(|branch| f(&branch.data))
            .map(|branch| (branch.id, &branch.data))
    }
}

#[derive(Clone, Debug)]
pub struct Branch<T> {
    id: NodeId,
    data: T,
}

impl<T> PartialEq for Branch<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Branch<T> {}

impl<T> PartialOrd for Branch<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Branch<T> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T> Branch<T> {
    fn new(id: NodeId, data: T) -> Self {
        Self { id, data }
    }
}

#[cfg(test)]
mod test {
    use super::Tree;
    const DATA_A: &str = "A";
    const DATA_B: &str = "B";
    const DATA_C: &str = "C";
    const DATA_D: &str = "D";
    const DATA_E: &str = "E";

    #[test]
    fn test_tree_method_first() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let (first_id, first_data) = {
            let first = tree.first().unwrap();
            let first_id = first.0.clone();
            let first_data = first.1.to_owned();
            assert_eq!(DATA_A, first_data);
            assert_eq!(id_a, first_id);

            (first_id, first_data)
        };

        let _id_b = tree.push(DATA_B);
        assert_eq!(DATA_A, first_data);
        assert_eq!(id_a, first_id);
    }

    #[test]
    fn test_tree_method_next() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let id_b = tree.push(DATA_B);
        let id_c = tree.push(DATA_C);

        let (id, data) = tree.first().unwrap();
        assert_eq!(id_a, id);
        assert_eq!(&DATA_A, data);

        let (id, data) = tree.next(id).unwrap();
        assert_eq!(id_b, id);
        assert_eq!(&DATA_B, data);

        let (id, data) = tree.next(id).unwrap();
        assert_eq!(id_c, id);
        assert_eq!(&DATA_C, data);

        assert!(tree.next(id).is_none());
    }

    #[test]
    fn test_tree_method_push() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let id_b = tree.push(DATA_B);
        let id_c = tree.push(DATA_C);

        assert_eq!(id_a, tree.branches[0].id);
        assert_eq!(id_b, tree.branches[1].id);
        assert_eq!(id_c, tree.branches[2].id);
    }

    #[test]
    fn test_tree_method_push_with_parent() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let id_b = tree.push_with_parent(DATA_B, id_a);
        let id_c = tree.push_with_parent(DATA_C, id_b);

        assert_eq!(id_a, tree.branches[0].id);
        assert_eq!(id_b, tree.branches[1].id);
        assert_eq!(id_c, tree.branches[2].id);

        assert_eq!((id_a, id_b), tree.parent_child_relationships[0]);
        assert_eq!((id_b, id_c), tree.parent_child_relationships[1]);
    }

    #[test]
    fn test_tree_method_get_by_id() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let id_b = tree.push(DATA_B);
        let id_c = tree.push(DATA_C);

        assert_eq!(Some(&DATA_B), tree.get_by_id(id_b));
        assert_eq!(Some(&DATA_C), tree.get_by_id(id_c));
        assert_eq!(Some(&DATA_A), tree.get_by_id(id_a));
    }

    #[test]
    fn test_tree_method_parent_of() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let id_b = tree.push_with_parent(DATA_B, id_a);
        let id_c = tree.push_with_parent(DATA_C, id_b);
        let id_d = tree.push_with_parent(DATA_D, id_c);
        let id_e = tree.push_with_parent(DATA_E, id_d);

        assert_eq!(None, tree.parent_of(id_a));
        assert_eq!(Some((id_a, &DATA_A)), tree.parent_of(id_b));
        assert_eq!(Some((id_b, &DATA_B)), tree.parent_of(id_c));
        assert_eq!(Some((id_c, &DATA_C)), tree.parent_of(id_d));
        assert_eq!(Some((id_d, &DATA_D)), tree.parent_of(id_e));
    }

    #[test]
    fn test_tree_method_siblings_of() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let id_b = tree.push_with_parent(DATA_B, id_a);
        let id_c = tree.push_with_parent(DATA_C, id_a);
        let id_d = tree.push_with_parent(DATA_D, id_b);
        let id_e = tree.push_with_parent(DATA_E, id_b);

        assert_eq!(
            vec![(id_c, &DATA_C)],
            tree.siblings_of(id_b).unwrap().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(id_b, &DATA_B)],
            tree.siblings_of(id_c).unwrap().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(id_e, &DATA_E)],
            tree.siblings_of(id_d).unwrap().collect::<Vec<_>>()
        );
        assert_eq!(
            vec![(id_d, &DATA_D)],
            tree.siblings_of(id_e).unwrap().collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_tree_method_next_sibling_of() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let id_b = tree.push_with_parent(DATA_B, id_a);
        let id_c = tree.push_with_parent(DATA_C, id_a);
        let id_d = tree.push_with_parent(DATA_D, id_b);
        let id_e = tree.push_with_parent(DATA_E, id_b);

        assert_eq!(None, tree.next_sibling_of(id_a));
        assert_eq!(Some((id_c, &DATA_C)), tree.next_sibling_of(id_b));
        assert_eq!(None, tree.next_sibling_of(id_c));
        assert_eq!(Some((id_e, &DATA_E)), tree.next_sibling_of(id_d));
        assert_eq!(None, tree.next_sibling_of(id_e));
    }

    #[test]
    fn test_tree_method_previous_sibling_of() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let id_b = tree.push_with_parent(DATA_B, id_a);
        let id_c = tree.push_with_parent(DATA_C, id_a);
        let id_d = tree.push_with_parent(DATA_D, id_b);
        let id_e = tree.push_with_parent(DATA_E, id_b);

        assert_eq!(None, tree.previous_sibling_of(id_a));
        assert_eq!(None, tree.previous_sibling_of(id_b));
        assert_eq!(Some((id_b, &DATA_B)), tree.previous_sibling_of(id_c));
        assert_eq!(None, tree.previous_sibling_of(id_d));
        assert_eq!(Some((id_d, &DATA_D)), tree.previous_sibling_of(id_e));
    }

    #[test]
    fn test_tree_method_find_by() {
        let mut tree = Tree::new();
        let id_a = tree.push(DATA_A);
        let id_b = tree.push_with_parent(DATA_B, id_a);
        let id_c = tree.push_with_parent(DATA_C, id_a);
        let id_d = tree.push_with_parent(DATA_D, id_b);
        let id_e = tree.push_with_parent(DATA_E, id_b);

        assert_eq!(Some((id_a, &DATA_A)), tree.find_by(|data| data == &DATA_A));
        assert_eq!(Some((id_b, &DATA_B)), tree.find_by(|data| data == &DATA_B));
        assert_eq!(Some((id_c, &DATA_C)), tree.find_by(|data| data == &DATA_C));
        assert_eq!(Some((id_d, &DATA_D)), tree.find_by(|data| data == &DATA_D));
        assert_eq!(Some((id_e, &DATA_E)), tree.find_by(|data| data == &DATA_E));
    }
}
