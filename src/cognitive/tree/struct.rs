use super::super::CognitiveModel;
use super::NodeID;
use super::error::{Error, Result};
use super::node::Node;
use crate::lexicon::LexiconEntry;
use crate::syntax::{FeatureSet, SyntaxValue};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct TreeModel<K> {
    nodes: Vec<Node<K>>,
    root: Option<NodeID>,
    working: Option<NodeID>,
}

// tree-level methods
impl<K> TreeModel<K> {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            root: None,
            working: None,
        }
    }
    fn get_working(&self) -> Result<NodeID> {
        self.working.ok_or(Error::NoWorkingNode)
    }
    fn set_working(&mut self, id: NodeID) {
        self.working = Some(id);
    }
    fn size(&self) -> usize {
        self.nodes.len()
    }
}

// node-level methods
impl<K> TreeModel<K> {
    /// parent = Option<(parent_id, project, left_first)>
    fn new_node(
        &mut self,
        value: SyntaxValue<K>,
        parent: Option<(NodeID, Option<FeatureSet<K>>, bool)>,
    ) -> Result<NodeID> {
        let id = self.size();
        self.nodes.push(Node::new(id, value));
        if let Some((parent_id, project, left_first)) = parent {
            self.set_relation(parent_id, id, project, left_first)?;
        }
        Ok(id)
    }

    fn get_node(&self, id: NodeID) -> Result<&Node<K>> {
        self.nodes.get(id).ok_or(Error::NodeNotFound(id))
    }

    fn get_node_mut(&mut self, id: NodeID) -> Result<&mut Node<K>> {
        self.nodes.get_mut(id).ok_or(Error::NodeNotFound(id))
    }

    fn get_value(&self, id: NodeID) -> Result<&SyntaxValue<K>> {
        Ok(self.get_node(id)?.get_value())
    }

    fn if_done(&self, id: NodeID) -> Result<bool> {
        Ok(self.get_node(id)?.if_done())
    }

    fn set_done(&mut self, id: NodeID) -> Result<()> {
        self.get_node_mut(id)?.set_done();
        Ok(())
    }

    fn get_parent(&self, id: NodeID) -> Result<NodeID> {
        match self.get_node(id)?.get_parent() {
            Some(parent_id) => Ok(parent_id),
            None => Err(Error::NodeHasNoParent(id)),
        }
    }

    fn get_project(&self, id: NodeID) -> Result<Option<&FeatureSet<K>>> {
        Ok(self.get_node(id)?.get_project())
    }
    fn set_project(&mut self, id: NodeID, project: Option<FeatureSet<K>>) -> Result<()> {
        self.get_node_mut(id)?.set_project(project);
        Ok(())
    }

    fn set_moved(&mut self, from: NodeID, to: NodeID) -> Result<()> {
        self.get_node_mut(from)?.set_moved(to);
        Ok(())
    }

    fn set_relation(
        &mut self,
        parent_id: NodeID,
        child_id: NodeID,
        project: Option<FeatureSet<K>>,
        left_first: bool,
    ) -> Result<()> {
        let parent = self.get_node_mut(parent_id)?;
        if !parent.add_child(child_id, left_first) {
            return Err(Error::NodeAlreadyHasTwoChildren(parent_id));
        }
        let child = self.get_node_mut(child_id)?;
        if !child.add_parent(parent_id) {
            return Err(Error::NodeAlreadyHasParent(child_id));
        }
        child.set_project(project);
        Ok(())
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
            with_project: Option<&FeatureSet<K>>,
        ) -> Result<NodeID> {
            let cur_id = self.get_working()?;
            self.set_done(cur_id)?;

            let parent_id = self.get_parent(cur_id)?;
            self.set_working(parent_id);
            self.delete_relation(parent_id, cur_id)?;

            let new_parent_id =
                self.append_child(value, self.get_project(cur_id)?.cloned(), cur_id)?;
            self.set_relation(new_parent_id, cur_id, with_project.cloned(), true)?;
            if let Some(ignore) = with_project {
                self.project(cur_id, ignore)?;
            }
            Ok(new_parent_id)
        }

        fn append_child(
            &mut self,
            value: LexiconNode<K>,
            project: Option<FeatureSet<K>>,
            trigger: NodeID,
        ) -> Result<NodeID> {
            let cur_id = self.get_working()?;

            match value {
                LexiconNode::Value { value } => {
                    let child_id = self.new_node(value, Some((cur_id, project, false)))?;
                    self.set_working(child_id);
                    Ok(child_id)
                }
                LexiconNode::Lambda {
                    from,
                    to,
                    project: from_project,
                } => {
                    let value = SyntaxValue::Features(to);
                    let child_id = self.new_node(value, Some((cur_id, project, false)))?;
                    self.set_working(child_id);

                    let child_child_id = self.append_child(*from, None, trigger)?;
                    if from_project {
                        let ignore_fs = self.get_features(child_child_id)?;
                        self.set_project(child_child_id, Some(ignore_fs.clone()))?;
                    }
                    Ok(child_id)
                }
                LexiconNode::Moved { from } => {
                    let value = SyntaxValue::Features(from);
                    let child_id = self.new_node(value, Some((cur_id, project, false)))?;
                    self.set_working(child_id);
                    self.set_moved(child_id, trigger)?;
                    Ok(child_id)
                }
            }
        }

        pub fn try_project(&mut self) -> Result<()> {
            while let Some(cur_id) = self.working {
                let parent_id = if let Ok(parent_id) = self.get_parent(cur_id) {
                    parent_id
                } else {
                    self.working = None;
                    break;
                };
                if let Some(ignore_fs) = self.get_project(cur_id)? {
                    let cur_fs = self.get_features(cur_id)?;
                    let parent_fs = self.get_features(parent_id)?;
                    if parent_fs.is_subset(cur_fs) {
                        self.project(cur_id, &ignore_fs.clone()).ok();
                        self.set_done(cur_id)?;
                        self.set_done(parent_id)?;
                    }
                }
                if self.if_done(cur_id)? {
                    self.set_working(parent_id);
                } else {
                    break;
                }
            }
            Ok(())
        }

        //
        // fn apply_entry aka decide
        // - insert_parent
        // - from the working position, project upward, if possible
    }
}

impl<K: Ord + Clone> CognitiveModel<K> for TreeModel<K> {
    fn init(target: FeatureSet<K>) -> Self {
        let mut model = Self::new();
        let root_id = model
            .new_node(SyntaxValue::Features(target), None)
            .expect("Cannot fail");
        model.root = Some(root_id);
        model.working = Some(root_id);
        model
    }
    fn understood(&self) -> bool {
        self.working.is_none()
    }
    fn receive(&mut self, token: K) -> super::super::error::Result<()> {
        let value = SyntaxValue::Item(token);
        let cur_id = self.get_working()?;
        let child_id = self.new_node(value, Some((cur_id, Some(FeatureSet::new()), false)))?;
        self.set_working(child_id);
        Ok(())
    }
    fn wonder(&self) -> Option<&SyntaxValue<K>> {
        match self.get_working() {
            Ok(cur_id) => self.get_value(cur_id).ok(),
            Err(_) => None,
        }
    }
    fn decide(&mut self, entry: LexiconEntry<K>) -> super::super::error::Result<()> {
        match entry {
            LexiconEntry::Lexical(value) => self.insert_parent(value, None)?,
            LexiconEntry::Functional { from, to, project } => {
                let with_project = if project { Some(&from) } else { None };
                self.insert_parent(to, with_project)?
            }
        };
        self.try_project()?;
        Ok(())
    }
}

impl<K: Display> TreeModel<K> {
    fn fmt_node(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        id: NodeID,
        mut indent: usize,
        may_skip: bool,
        debug_mode: bool,
    ) -> std::fmt::Result {
        let node = match self.get_node(id) {
            Ok(node) => node,
            Err(_) => return Err(std::fmt::Error),
        };
        let is_features = matches!(node.get_value(), SyntaxValue::Features(_));

        if !(may_skip && is_features) || debug_mode {
            write!(f, "{}[", " ".repeat(indent))?;
            if node.get_project().is_some() {
                write!(f, "★ ")?;
            }
            write!(f, "{}]", node.get_id())?;
            if let Some(moved_id) = node.get_moved() {
                write!(f, " --> [{}]", moved_id)?;
            }
            /* debug */
            write!(f, "[parent: {}]", node.get_parent().unwrap_or(0))?;

            writeln!(f, " {}", node.get_value())?;
            indent += 4;
        }

        let may_skip = is_features && node.number_of_children() == 1;
        if let Some(left_id) = node.get_left() {
            self.fmt_node(f, left_id, indent, may_skip, debug_mode)?;
        }
        if let Some(right_id) = node.get_right() {
            self.fmt_node(f, right_id, indent, may_skip, debug_mode)?;
        }

        Ok(())
    }
}

impl<K: Display> Display for TreeModel<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(root_id) = self.root {
            self.fmt_node(f, root_id, 0, false, false)?;
        }
        Ok(())
    }
}
