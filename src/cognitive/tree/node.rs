use super::NodeID;
use crate::syntax::{FeatureSet, SyntaxValue};

#[derive(Debug, Clone)]
pub struct Node<K> {
    id: NodeID,
    value: SyntaxValue<K>,
    done: bool,
    parent: Option<NodeID>,
    project: Option<FeatureSet<K>>,
    left: Option<NodeID>,
    right: Option<NodeID>,
    moved: Option<NodeID>,
}

impl<K> Node<K> {
    pub fn new(id: NodeID, value: SyntaxValue<K>) -> Self {
        Node {
            id,
            value,
            done: false,
            parent: None,
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
    pub fn add_parent(&mut self, parent: NodeID) -> bool {
        if self.parent.is_none() {
            self.parent = Some(parent);
        } else {
            return false;
        }
        true
    }
    pub fn add_child(&mut self, child: NodeID, left_first: bool) -> bool {
        if left_first && self.left.is_none() {
            self.left = Some(child);
        } else if !left_first && self.right.is_none() {
            self.right = Some(child);
        } else if self.left.is_none() {
            self.left = Some(child);
        } else if self.right.is_none() {
            self.right = Some(child);
        } else {
            return false;
        }

        true
    }
}
