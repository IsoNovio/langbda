use super::NodeID;
use crate::syntax::{FeatureSet, SyntaxValue};

#[derive(Debug, Clone)]
pub struct Node<K> {
    // self
    id: NodeID,
    value: SyntaxValue<K>,
    done: bool,

    // parent
    parent: Option<NodeID>,
    is_left: bool,
    project: Option<FeatureSet<K>>,

    // children
    left: Option<NodeID>,
    right: Option<NodeID>,

    // other
    moved: Option<NodeID>,
}

impl<K> Node<K> {
    pub fn new(id: NodeID, value: SyntaxValue<K>) -> Self {
        Node {
            id,
            value,
            done: false,
            parent: None,
            is_left: false,
            project: None,
            left: None,
            right: None,
            moved: None,
        }
    }
    pub fn get_id(&self) -> NodeID {
        self.id
    }
    pub fn get_value(&self) -> &SyntaxValue<K> {
        &self.value
    }
    pub fn if_done(&self) -> bool {
        self.done
    }
    pub fn set_done(&mut self) {
        self.done = true;
    }
    pub fn get_value_mut(&mut self) -> &mut SyntaxValue<K> {
        &mut self.value
    }
    pub fn take_project(&mut self) -> Option<FeatureSet<K>> {
        self.project.take()
    }
    pub fn get_project(&self) -> Option<&FeatureSet<K>> {
        self.project.as_ref()
    }
    pub fn set_project(&mut self, project: Option<FeatureSet<K>>) {
        self.project = project;
    }
    pub fn get_parent(&self) -> Option<NodeID> {
        self.parent
    }
    pub fn set_parent(&mut self, parent: Option<NodeID>) {
        self.parent = parent;
    }
    pub fn get_is_left(&self) -> bool {
        self.is_left
    }
    pub fn set_as_left(&mut self) {
        self.is_left = true;
    }
    pub fn set_as_right(&mut self) {
        self.is_left = false;
    }
    pub fn get_left(&self) -> Option<NodeID> {
        self.left
    }
    pub fn set_left(&mut self, left: Option<NodeID>) {
        self.left = left;
    }
    pub fn get_right(&self) -> Option<NodeID> {
        self.right
    }
    pub fn set_right(&mut self, right: Option<NodeID>) {
        self.right = right;
    }
    pub fn get_moved(&self) -> Option<NodeID> {
        self.moved
    }
    pub fn set_moved(&mut self, moved: NodeID) {
        self.moved = Some(moved);
    }
    pub fn number_of_children(&self) -> usize {
        self.left.is_some() as usize + self.right.is_some() as usize
    }
}
