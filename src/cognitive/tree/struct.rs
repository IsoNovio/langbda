use super::super::CognitiveModel;
use super::NodeID;
use super::error::{Error, Result};
use super::node::Node;
use crate::lexicon::LexiconEntry;
use crate::syntax::{FeatureSet, SyntaxValue};
use std::fmt::{Debug, Display};

#[derive(Clone)]
pub struct TreeModel<K> {
    nodes: Vec<Node<K>>,
    root: NodeID,
    upper_cursor: NodeID,
    lower_cursor: NodeID,
    unattached: Vec<NodeID>,
}

// tree-level methods
impl<K> TreeModel<K> {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: 0,
            upper_cursor: 0,
            lower_cursor: 0,
            unattached: Vec::new(),
        }
    }
    fn size(&self) -> usize {
        self.nodes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
    pub fn get_root(&self) -> NodeID {
        self.root
    }
}

// node-level methods
impl<K> TreeModel<K> {
    // node
    fn new_node(&mut self, value: SyntaxValue<K>) -> NodeID {
        let id = self.size();
        self.nodes.push(Node::new(id, value));
        id
    }
    fn get_node(&self, id: NodeID) -> Result<&Node<K>> {
        self.nodes.get(id).ok_or(Error::NodeNotFound(id))
    }
    fn get_node_mut(&mut self, id: NodeID) -> Result<&mut Node<K>> {
        self.nodes.get_mut(id).ok_or(Error::NodeNotFound(id))
    }

    // node self
    pub fn get_value(&self, id: NodeID) -> Result<&SyntaxValue<K>> {
        Ok(self.get_node(id)?.get_value())
    }
    fn if_done(&self, id: NodeID) -> Result<bool> {
        Ok(self.get_node(id)?.if_done())
    }
    fn set_done(&mut self, id: NodeID) -> Result<()> {
        self.get_node_mut(id)?.set_done();
        Ok(())
    }

    // node parent
    fn get_parent(&self, id: NodeID) -> Result<NodeID> {
        match self.get_node(id)?.get_parent() {
            Some(parent_id) => Ok(parent_id),
            None => Err(Error::NodeHasNoParent(id)),
        }
    }
    fn get_is_left(&self, id: NodeID) -> Result<bool> {
        Ok(self.get_node(id)?.get_is_left())
    }
    fn take_project(&mut self, id: NodeID) -> Result<Option<FeatureSet<K>>> {
        Ok(self.get_node_mut(id)?.take_project())
    }
    fn set_project(&mut self, id: NodeID, project: Option<FeatureSet<K>>) -> Result<()> {
        self.get_node_mut(id)?.set_project(project);
        Ok(())
    }

    // node children
    pub fn get_left(&self, id: NodeID) -> Result<Option<NodeID>> {
        Ok(self.get_node(id)?.get_left())
    }
    pub fn get_right(&self, id: NodeID) -> Result<Option<NodeID>> {
        Ok(self.get_node(id)?.get_right())
    }

    // node other
    pub fn get_moved(&self, id: NodeID) -> Result<Option<NodeID>> {
        Ok(self.get_node(id)?.get_moved())
    }
    fn set_moved(&mut self, from: NodeID, to: NodeID) -> Result<()> {
        self.get_node_mut(from)?.set_moved(to);
        Ok(())
    }

    fn set_relation_left(&mut self, parent_id: NodeID, child_id: NodeID) -> Result<()> {
        let parent = self.get_node_mut(parent_id)?;
        parent.set_left(Some(child_id));
        let child = self.get_node_mut(child_id)?;
        child.set_parent(Some(parent_id));
        child.set_as_left();
        Ok(())
    }

    fn set_relation_right(&mut self, parent_id: NodeID, child_id: NodeID) -> Result<()> {
        let parent = self.get_node_mut(parent_id)?;
        parent.set_right(Some(child_id));
        let child = self.get_node_mut(child_id)?;
        child.set_parent(Some(parent_id));
        child.set_as_right();
        Ok(())
    }

    fn set_relation(
        &mut self,
        parent_id: NodeID,
        child_id: NodeID,
        child_is_left: bool,
    ) -> Result<()> {
        match child_is_left {
            true => self.set_relation_left(parent_id, child_id),
            false => self.set_relation_right(parent_id, child_id),
        }
    }

    fn delete_relation(&mut self, parent_id: NodeID, child_id: NodeID) -> Result<()> {
        let parent = self.get_node_mut(parent_id)?;
        if parent.get_left() == Some(child_id) {
            parent.set_left(None);
        } else if parent.get_right() == Some(child_id) {
            parent.set_right(None);
        } else {
            return Err(Error::NodeHasNoChildWithID(parent_id, child_id));
        }

        let child = self.get_node_mut(child_id)?;
        if child.get_parent() == Some(parent_id) {
            child.set_parent(None);
        } else {
            return Err(Error::NodeHasNoParentWithID(child_id, parent_id));
        }

        Ok(())
    }

    /// add node as left child of upper cursor
    /// put existing left child to unattached
    /// move lower cursor to child
    fn add_left(&mut self, child_id: NodeID) -> Result<()> {
        let parent_id = self.upper_cursor;
        if let Some(old_left_id) = self.get_left(parent_id)? {
            self.delete_relation(parent_id, old_left_id)?;
            self.unattached.push(old_left_id);
        }
        self.set_relation_left(parent_id, child_id)?;
        self.lower_cursor = child_id;
        Ok(())
    }

    /// add node as right child of lower cursor
    /// move lower cursor to child
    /// move upper cursor to child
    fn add_right(&mut self, child_id: NodeID) -> Result<()> {
        let parent_id = self.lower_cursor;
        self.set_relation_right(parent_id, child_id)?;
        self.lower_cursor = child_id;
        self.upper_cursor = child_id;
        Ok(())
    }

    fn add_child(&mut self, child_id: NodeID, is_left: bool) -> Result<()> {
        match is_left {
            true => self.add_left(child_id),
            false => self.add_right(child_id),
        }
    }
}

// interface with FeatureSet
impl<K: Ord + Clone> TreeModel<K> {
    fn get_features(&self, id: NodeID) -> Result<&FeatureSet<K>> {
        match self.get_node(id)?.get_value() {
            SyntaxValue::Features(features) => Ok(features),
            _ => Err(Error::NodeIsNotFeatures(id)),
        }
    }

    fn get_features_mut(&mut self, id: NodeID) -> Result<&mut FeatureSet<K>> {
        match self.get_node_mut(id)?.get_value_mut() {
            SyntaxValue::Features(features) => Ok(features),
            _ => Err(Error::NodeIsNotFeatures(id)),
        }
    }

    fn project(&mut self, from_id: NodeID, ignore: &FeatureSet<K>) -> Result<()> {
        let parent_id = self.get_parent(from_id)?;
        let from_fs = self.get_features(from_id)?.clone();
        let onto_fs = self.get_features_mut(parent_id)?;

        FeatureSet::project(&from_fs, onto_fs, ignore)?;
        Ok(())
    }
}

// interface with Lexicon
mod lexicon {
    use super::*;
    use crate::lexicon::LexiconNode;

    impl<K: Ord + Clone> TreeModel<K> {
        pub fn insert_parent(
            &mut self,
            value: LexiconNode<K>,
            with_project: Option<FeatureSet<K>>,
        ) -> Result<()> {
            let cur_id = self.lower_cursor;
            let cur_is_left = self.get_is_left(cur_id)?;
            let old_upper_cursor = self.upper_cursor;

            // detach cur from its parent
            let parent_id = self.get_parent(cur_id)?;
            self.delete_relation(parent_id, cur_id)?;

            // mark cur as done
            self.set_done(cur_id)?;
            self.lower_cursor = parent_id;

            // append new parent as child of old parent
            let new_parent_id = self.append_child(value, cur_id, cur_is_left)?;

            // link cur and new parent
            self.set_relation(new_parent_id, cur_id, cur_is_left)?;
            if let cur_project @ Some(_) = self.take_project(cur_id)? {
                self.set_project(new_parent_id, cur_project)?
            }
            if let Some(ignore) = with_project {
                self.project(cur_id, &ignore)?;
            }

            // re-add unattached nodes after the tree has been adjusted
            if old_upper_cursor != self.upper_cursor {
                if let Some(unattached_id) = self.unattached.pop() {
                    self.add_left(unattached_id)?
                }
            }

            // start from lower_cursor
            // if satisfies parent's features, go upward
            self.try_project()
        }

        fn append_child(
            &mut self,
            value: LexiconNode<K>,
            trigger: NodeID,
            child_is_left: bool,
        ) -> Result<NodeID> {
            match value {
                LexiconNode::Value { value } => {
                    let child_id = self.new_node(value);
                    self.add_child(child_id, child_is_left)?;
                    Ok(child_id)
                }
                LexiconNode::Lambda {
                    from,
                    to,
                    project: from_project,
                } => {
                    let to = match *to {
                        LexiconNode::Value { value } => Ok(value),
                        _ => Err(Error::LambdaToIsNestedLambda),
                    }?;
                    let child_id = self.new_node(to);
                    self.add_child(child_id, child_is_left)?;

                    let child_child_id = self.append_child(*from, trigger, !child_is_left)?;
                    if from_project {
                        let ignore_fs = self.get_features(child_child_id)?;
                        self.set_project(child_child_id, Some(ignore_fs.clone()))?;
                    }
                    Ok(child_id)
                }
                LexiconNode::Moved { from } => {
                    let from = SyntaxValue::from(from);
                    let child_id = self.new_node(from);
                    self.add_child(child_id, child_is_left)?;
                    self.set_moved(child_id, trigger)?;
                    Ok(child_id)
                }
            }
        }

        fn try_project(&mut self) -> Result<()> {
            let cur_id = self.lower_cursor;

            let parent_id = match self.get_parent(cur_id) {
                Ok(id) => id,
                Err(_) => return Ok(()),
            };

            if !self.if_done(cur_id)? && self.get_is_left(cur_id)? {
                let cur_fs = self.get_features(cur_id)?;
                let parent_fs = self.get_features(parent_id)?;
                if parent_fs.is_subset(cur_fs) {
                    self.project(cur_id, &parent_fs.clone())?;
                    self.set_done(cur_id)?;
                    self.set_done(parent_id)?;
                }
            }

            if !self.if_done(cur_id)? {
                return Ok(());
            }

            if let Some(ignore_fs) = self.take_project(cur_id)? {
                self.project(cur_id, &ignore_fs)?
            }

            self.lower_cursor = parent_id;
            self.upper_cursor = self.lower_cursor;
            while self.get_is_left(self.upper_cursor)? {
                self.upper_cursor = self.get_parent(self.upper_cursor)?
            }
            self.try_project()
        }
    }
}

impl<K: Ord + Clone> CognitiveModel<K> for TreeModel<K> {
    fn init(target: FeatureSet<K>) -> Self {
        let mut model = Self::new();
        model.new_node(SyntaxValue::Features(target));
        model
    }
    fn understood(&self) -> bool {
        self.if_done(self.lower_cursor).unwrap_or(false)
    }
    fn demand(&self) -> bool {
        true
    }
    fn receive(&mut self, token: K) -> super::super::error::Result<()> {
        let new_node = SyntaxValue::from(token);
        let new_node = self.new_node(new_node);
        self.add_left(new_node)?;
        Ok(())
    }
    fn wonder(&self) -> Option<&SyntaxValue<K>> {
        self.get_value(self.lower_cursor).ok()
    }
    fn decide(&mut self, entry: LexiconEntry<K>) -> super::super::error::Result<()> {
        match entry {
            LexiconEntry::Lexical(value) => self.insert_parent(value, None)?,
            LexiconEntry::Functional { to, project } => self.insert_parent(to, project)?,
        };
        Ok(())
    }
}

impl<K: Display> TreeModel<K> {
    fn fmt_node_debug(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        id: NodeID,
        mut indent: usize,
    ) -> std::fmt::Result {
        let node = self.get_node(id).map_err(|_| std::fmt::Error)?;

        // print this node
        write!(f, "{}[", " ".repeat(indent))?;
        if node.if_done() {
            write!(f, "✔  ")?
        }
        if node.get_project().is_some() {
            write!(f, "★ ")?;
        }
        write!(f, "{}]", node.get_id())?;
        if let Some(moved_id) = node.get_moved() {
            write!(f, " --> [{}]", moved_id)?;
        }
        write!(f, " {}", node.get_value())?;
        if id == self.upper_cursor {
            write!(f, " <--UPPER_CURSOR|")?
        }
        if id == self.lower_cursor {
            write!(f, " <--LOWER_CURSOR|")?
        }
        write!(f, "({} children)", node.number_of_children())?;

        writeln!(f)?;

        // print children
        indent += 4;
        if let Some(left_id) = node.get_left() {
            self.fmt_node_debug(f, left_id, indent)?;
        }
        if let Some(right_id) = node.get_right() {
            self.fmt_node_debug(f, right_id, indent)?;
        }

        Ok(())
    }
}

impl<K: Display> Debug for TreeModel<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.size() > 0 {
            self.fmt_node_debug(f, self.root, 0)?;
        }

        if !self.unattached.is_empty() {
            writeln!(f, "Unattached subtrees:")?;
            for id in self.unattached.iter() {
                self.fmt_node_debug(f, *id, 0)?;
            }
        }

        Ok(())
    }
}

impl<K: Display + Clone + Ord> Display for TreeModel<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nodes = Vec::new();
        if self.size() > 0 {
            nodes.push((0, 0))
        }

        while let Some((id, indent)) = nodes.pop() {
            let node = self.get_node(id).map_err(|_| std::fmt::Error)?;

            // print this node
            write!(f, "{}[", " ".repeat(indent))?;
            if node.get_project().is_some() {
                write!(f, "★ ")?
            }
            write!(f, "{}]", node.get_id())?;
            if let Some(moved_id) = node.get_moved() {
                write!(f, " --> [{}]", moved_id)?;
            }
            write!(f, " {}", node.get_value())?;
            writeln!(f)?;

            // print children
            // push right first because nodes is a stack
            if let Some(right_id) = node.get_right() {
                nodes.push((right_id, indent + 4))
            }
            if let Some(left_id) = node.get_left() {
                nodes.push((left_id, indent + 4))
            }
        }

        Ok(())
    }
}

impl<K: Clone + Ord> TreeModel<K> {
    pub fn prune(&mut self) -> Result<()> {
        let mut nodes = Vec::new();
        if self.size() > 0 {
            nodes.push(0)
        }

        let mut parent_features = None;
        let mut parent_child_cnt = 0;
        let mut parent_id = 0;
        while let Some(id) = nodes.pop() {
            let node = self.get_node(id)?;
            let features: Option<FeatureSet<K>> = node.get_value().clone().try_into().ok();
            let child_cnt = node.number_of_children();

            let delete = match (&features, &parent_features) {
                (Some(fs), Some(parent_fs)) => fs.is_subset(parent_fs) && parent_child_cnt == 1,
                _ => false,
            };

            let left_id = node.get_left();
            let right_id = node.get_right();
            if delete {
                self.delete_relation(parent_id, id)?;
                if let Some(left_id) = left_id {
                    self.delete_relation(id, left_id)?;
                    self.set_relation_left(parent_id, left_id)?;
                    nodes.push(left_id)
                }
                if let Some(right_id) = right_id {
                    self.delete_relation(id, right_id)?;
                    self.set_relation_right(parent_id, right_id)?;
                    nodes.push(right_id)
                }
                parent_child_cnt = child_cnt;
            } else {
                if let Some(left_id) = left_id {
                    nodes.push(left_id)
                }
                if let Some(right_id) = right_id {
                    nodes.push(right_id)
                }
                parent_features = features;
                parent_child_cnt = child_cnt;
                parent_id = id;
            }
        }

        Ok(())
    }
}
