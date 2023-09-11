use core::fmt;
use std::sync::atomic::{AtomicU16, Ordering};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

pub struct Tree<T> {
    branches: Vec<Branch<T>>,
    // A is parent of B
    parent_child_relationships: Vec<(NodeId, NodeId)>,
    // A is child of B
    child_parent_relationships: Vec<(NodeId, NodeId)>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            branches: Vec::new(),
            parent_child_relationships: Vec::new(),
            child_parent_relationships: Vec::new(),
        }
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
        self.child_parent_relationships.push((id, parent));

        id
    }

    fn get_by_id(&self, id: NodeId) -> Option<&T> {
        self.branches
            .iter()
            .find(|branch| branch.id == id)
            .map(|branch| &branch.data)
    }

    fn parent_id_of(&self, id: NodeId) -> Option<NodeId> {
        self.child_parent_relationships
            .iter()
            .find(|(child, _)| *child == id)
            .map(|(_, parent)| *parent)
    }

    pub fn parent_of(&self, id: NodeId) -> Option<(NodeId, &T)> {
        self.parent_id_of(id)
            .and_then(|parent_id| self.get_by_id(parent_id).map(|data| (parent_id, data)))
    }

    pub fn next_sibling_of(&self, id: NodeId) -> Option<(NodeId, &T)> {
        self.parent_id_of(id)
            .and_then(|parent_id| {
                self.child_parent_relationships
                    .iter()
                    .find(|(_, parent)| *parent == parent_id)
                    .map(|(child, _)| *child)
            })
            .and_then(|sibling_id| self.get_by_id(sibling_id).map(|data| (sibling_id, data)))
    }

    pub fn previous_sibling_of(&self, id: NodeId) -> Option<(NodeId, &T)> {
        self.parent_id_of(id)
            .and_then(|parent_id| {
                self.child_parent_relationships
                    .iter()
                    .rev()
                    .find(|(_, parent)| *parent == parent_id)
                    .map(|(child, _)| *child)
            })
            .and_then(|sibling_id| self.get_by_id(sibling_id).map(|data| (sibling_id, data)))
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

struct Branch<T> {
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
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
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
