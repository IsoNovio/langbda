use core::fmt;

use super::todo_task::TodoTask;
use super::tree_node::TreeNode;

use crate::direction::{self, Direction};
use crate::lexicon::Interpretation;
use crate::sentence::Sentence;
use crate::syntax_node::SyntaxNode;
use crate::feature::FeatureSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SyntaxTree<'a> {
    nodes: Vec<TreeNode<'a>>,
    tree_root: Option<usize>,
    todo_stack: Vec<TodoTask<'a>>,
    token_stack: Vec<&'a str>,
}

impl<'a> SyntaxTree<'a> {
    pub fn new() -> SyntaxTree<'a> {
        SyntaxTree {
            nodes: Vec::new(),
            tree_root: None,
            todo_stack: Vec::new(),
            token_stack: Vec::new(),
        }
    }

    pub fn job_done(&self) -> bool {
        self.todo_stack.is_empty() && self.token_stack.is_empty()
    }

    pub fn get_node(&self, idx: usize) -> Option<&TreeNode<'a>> {
        self.nodes.get(idx)
    }

    pub fn get_node_mut(&mut self, idx: usize) -> Option<&mut TreeNode<'a>> {
        self.nodes.get_mut(idx)
    }

    pub fn get_root(&self) -> Option<&TreeNode<'a>> {
        self.tree_root.and_then(|idx| self.get_node(idx))
    }

    pub fn get_left(&self, node: &TreeNode<'a>) -> Option<&TreeNode<'a>> {
        node.get_left().and_then(|idx| self.get_node(idx))
    }

    pub fn get_right(&self, node: &TreeNode<'a>) -> Option<&TreeNode<'a>> {
        node.get_right().and_then(|idx| self.get_node(idx))
    }

    pub fn get_size(&self) -> usize {
        self.nodes.len()
    }

    pub fn todo_push(&mut self, todo: TodoTask<'a>) {
        self.todo_stack.push(todo);
    }

    pub fn todo_pop(&mut self) -> Option<TodoTask<'a>> {
        self.todo_stack.pop()
    }

    pub fn todo_peek(&self) -> Option<&TodoTask<'a>> {
        self.todo_stack.last()
    }

    fn token_pop(&mut self) -> Option<&'a str> {
        self.token_stack.pop()
    }

    fn calc_depth(&self, node: &TreeNode<'a>) -> Result<usize, String> {
        let mut depth = 0;
        let mut this = node;
        while let Some(idx_parent) = this.get_parent() {
            match self.get_node(idx_parent) {
                Some(node_parent) => {
                    this = node_parent;
                    depth += 1;
                }
                None => {
                    return Err(format!("Parent {} of node {} does not exist", idx_parent, node.get_idx()));
                }
            }
        }
        Ok(depth)
    }

    fn add_parent_node(&mut self, value: SyntaxNode<'a>, child: Option<usize>) -> Result<usize, String> {
        let idx = self.get_size();
        let mut node = TreeNode::new(value, self.get_size(), None, child, None);

        let parent = match child {
            Some(idx_child) => match self.get_node(idx_child) {
                Some(node_child) => node_child.get_parent(),
                None => {
                    return Err(format!("Child {} of node {} does not exist", idx_child, idx));
                }
            },
            None => None,
        };

        match child {
            Some(idx_child) => match self.get_node_mut(idx_child) {
                Some(node_child) => {
                    node_child.set_parent(Some(idx));
                    node.set_left(Some(idx_child));
                }
                None => {
                    return Err(format!("Child {} of node {} does not exist", idx_child, idx));
                }
            },
            None => (),
        }

        match parent {
            Some(idx_parent) => match self.get_node_mut(idx_parent) {
                Some(node_parent) => {
                    if node_parent.get_left() == child {
                        node_parent.set_left(Some(idx))
                    } else {
                        node_parent.set_right(Some(idx))
                    }
                    node.set_parent(Some(idx_parent));
                }
                None => {
                    return Err(format!("Parent {} of node {} does not exist", idx_parent, idx));
                }
            },
            None => (),
        }

        self.nodes.push(node);
        Ok(idx)
    }

    fn add_child_node(&mut self, value: SyntaxNode<'a>, parent: Option<usize>, direction: Direction) -> Result<usize, String> {
        let idx = self.get_size();
        let mut node = TreeNode::new(value, idx, parent, None, None);

        let child = match parent {
            Some(idx_parent) => match self.get_node(idx_parent) {
                Some(node_parent) => match direction {
                    direction::Direction::Left => node_parent.get_left(),
                    direction::Direction::Right => node_parent.get_right(),
                },
                None => {
                    return Err(format!("Parent {} of node {} does not exist", idx_parent, idx));
                }
            },
            None => None,
        };

        match parent {
            Some(idx_parent) => match self.get_node_mut(idx_parent) {
                Some(node_parent) => {
                    match direction {
                        direction::Direction::Left => node_parent.set_left(Some(idx)),
                        direction::Direction::Right => node_parent.set_right(Some(idx)),
                    }
                    node.set_parent(Some(idx_parent));
                }
                None => {
                    return Err(format!("Parent {} of node {} does not exist", idx_parent, idx));
                }
            },
            None => {}
        }

        match child {
            Some(idx_child) => match self.get_node_mut(idx_child) {
                Some(node_child) => {
                    node_child.set_parent(Some(idx));
                    node.set_left(Some(idx_child));
                }
                None => {
                    return Err(format!("Child {} of node {} does not exist", idx_child, idx));
                }
            },
            None => {}
        }

        self.nodes.push(node);
        Ok(idx)
    }

    pub fn init(sentence: &'a Sentence, target: &'a str) -> Result<SyntaxTree<'a>, String> {
        let mut tree = SyntaxTree::new();

        tree.add_child_node(SyntaxNode::from_feature_str(target), None, direction::Direction::Left)?;

        tree.tree_root = Some(0);
        tree.todo_push(TodoTask::Val(0));

        tree.token_stack = sentence.as_str();
        tree.token_stack.reverse();

        Ok(tree)
    }

    pub fn add_token(&mut self) -> Result<(), String> {
        let token = self.token_pop().ok_or("token stack is empty")?;
        let todo = self.todo_pop().ok_or("todo stack is empty")?;
        match todo {
            TodoTask::Val(parent_idx) => match self.get_node_mut(parent_idx) {
                Some(_) => {
                    let idx = self.add_child_node(SyntaxNode::from_lexical_str(token), Some(parent_idx), direction::Direction::Left)?;
                    self.todo_push(TodoTask::Lambda { from: idx, to: parent_idx });
                }
                None => return Err(format!("Node {} does not exist", parent_idx)),
            },
            todo => {
                return Err(format!("When adding token, todo stack does not pop a Val, but {:?}", todo));
            }
        }
        Ok(())
    }

    pub fn apply_entry(&mut self, input_val: &'a SyntaxNode<'a>, interp_val: &'a Interpretation<'a>) -> Result<(), String> {
        let todo = self.todo_pop().ok_or("todo stack is empty")?;
        match todo {
            TodoTask::Lambda { from: popped_from, to: popped_to } => {
                // use input = interp to replace from
                let from_node = self.get_node_mut(popped_from).ok_or(format!("Node {} does not exist", popped_from))?;
                let from_val = from_node.get_value_mut();
                match (from_val, input_val, interp_val) {
                    (SyntaxNode::Lexi(_), SyntaxNode::Lexi(_), Interpretation::Val(interp_fset)) => {
                        let idx = self.add_parent_node(SyntaxNode::from_feature_set(interp_fset.clone()), Some(popped_from))?;
                        self.todo_push(TodoTask::Lambda { from: idx, to: popped_to })
                    }
                    (SyntaxNode::Feat(_), SyntaxNode::Feat(input_fset), Interpretation::Val(interp_fset)) => {
                        let idx = self.add_parent_node(SyntaxNode::from_feature_set(interp_fset.clone()), Some(popped_from))?;
                        self.todo_push(TodoTask::Lambda { from: idx, to: popped_to });
                        self.todo_push(TodoTask::Project {from: popped_from, to: idx, ignore: input_fset});
                    }
                    (
                        SyntaxNode::Feat(_),
                        SyntaxNode::Feat(input_fset),
                        Interpretation::Lambda {
                            from: interp_from,
                            to: interp_to,
                            projection: interp_proj,
                        },
                    ) => {
                        let interp_to_idx = self.add_parent_node(SyntaxNode::from_feature_set(interp_to.clone()), Some(popped_from))?;
                        let interp_from_idx = self.add_child_node(SyntaxNode::from_feature_set(interp_from.clone()), Some(interp_to_idx), Direction::Right)?;
                        self.todo_push(TodoTask::Lambda { from: interp_to_idx, to: popped_to });
                        match interp_proj {
                            Direction::Left => {
                                self.todo_push(TodoTask::Project {from: popped_from, to: interp_to_idx, ignore: input_fset})
                            }
                            Direction::Right => {
                                self.todo_push(TodoTask::Project {from: interp_from_idx, to: interp_to_idx, ignore: interp_from})
                            }
                        }
                        self.todo_push(TodoTask::Val(interp_from_idx))
                    }
                    (from_val, input_val, interp_val) => {
                        return Err(format!("Not a valid entry case: apply {:?} = {:?} on {:?}", input_val, interp_val, from_val));
                    }
                }
            }
            _ => return Err(format!("When applying entry, todo stack does not pop a Lambda")),
        }

        Ok(())
    }

    pub fn do_projection(&mut self) -> Result<(), String> {
        match self.todo_pop().ok_or("todo stack is empty")? {
            TodoTask::Project { from, to, ignore } => {
                let from_fset = match self.get_node(from).ok_or(format!("Node {} does not exist", from))?.get_value() {
                    SyntaxNode::Feat(fset) => fset.clone(),
                    _ => return Err(format!("Node {} is not a feature set", from)),
                };
                let to_sn = self.get_node_mut(to).ok_or(format!("Node {} does not exist", to))?.get_value_mut();
                let to_fset = match to_sn {
                    SyntaxNode::Feat(fset) => fset,
                    _ => return Err(format!("Node {} is not a feature set", to)),
                };
                FeatureSet::project(&from_fset, to_fset, ignore)?
            }
            _ => return Err(format!("When doing projection, todo stack does not pop a Project")),
        }
        Ok(())
    }

    pub fn delete_identity_lambda(&mut self) -> Result<(), String> {
        match self.todo_pop().ok_or("todo stack is empty")? {
            TodoTask::Lambda { from, to } => {
                let from_fset = match self.get_node(from).ok_or(format!("Node {} does not exist", from))?.get_value() {
                    SyntaxNode::Feat(fset) => fset.clone(),
                    _ => return Err(format!("Node {} is not a feature set", from)),
                };
                let to_sn = self.get_node_mut(to).ok_or(format!("Node {} does not exist", to))?.get_value_mut();
                let to_fset = match to_sn {
                    SyntaxNode::Feat(fset) => fset,
                    _ => return Err(format!("Node {} is not a feature set", to)),
                };
                if to_fset.is_subset(&from_fset) {
                    let ignore = to_fset.clone();
                    FeatureSet::project(&from_fset, to_fset, &ignore)?
                    
                } else {
                    return Err(format!("Node {} is not a subset of node {}", to, from));
                }
            }
            _ => return Err(format!("When deleting identity lambda, todo stack does not pop a Lambda")),
        }
        Ok(())
    }
}


impl<'a> std::fmt::Display for SyntaxTree<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_token_stack = self.token_stack.join(", ");
        let fmt_todo_stack = self.todo_stack.iter().map(|t| format!("{}", t)).collect::<Vec<String>>().join(", ");

        fn fmt_subtree(tree: &SyntaxTree, idx: usize, depth: usize) -> Result<String, String> {
            let this = tree.get_node(idx).ok_or(format!("Node {} does not exist", idx))?;
            let left = match this.get_left() {
                Some(left) => fmt_subtree(tree, left, depth + 1)?,
                None => String::new(),
            };
            let right = match this.get_right() {
                Some(right) => fmt_subtree(tree, right, depth + 1)?,
                None => String::new(),
            };
            let this_value = format!("{}", this);
            let indent = " ".repeat(depth * 4);
            Ok(format!("\n{}[{}]{}{} ", indent, this_value, left, right))
        }

        let root_idx = self.tree_root.ok_or("Tree root does not exist").unwrap_or(0);
        let fmt_tree = fmt_subtree(self,root_idx, 0 ).unwrap_or("NO TREE".to_string());
        write!(f, "Token Stack: {}\nTodo Stack: {}\n\nTree:\n{}\n", fmt_token_stack, fmt_todo_stack, fmt_tree)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::FeatureSet;

    #[test]
    fn new_syntax_tree() {
        let sentence = "Whose naughty child ate the apple in the room?";
        let sentence = Sentence::from_str(sentence).unwrap();
        let tree = SyntaxTree::init(&sentence, "S");
        println!("{:#?}", tree);
    }

    #[test]
    fn syntax_tree_add_token() {
        let sentence = "Whose naughty child ate the apple in the room?";
        let sentence = Sentence::from_str(sentence).unwrap();
        let mut tree = SyntaxTree::init(&sentence, "S").unwrap();
        tree.add_token().unwrap();
        println!("{:#?}", tree);
    }

    #[test]
    fn syntax_tree_apply_entry() {
        let sentence = "Whose naughty child ate the apple in the room?";
        let sentence = Sentence::from_str(sentence).unwrap();
        let mut tree = SyntaxTree::init(&sentence, "S").unwrap();
        tree.add_token().unwrap();

        let sn1 = SyntaxNode::from_lexical_str("whose");
        let mut fs1 = FeatureSet::new();
        fs1.insert("Det", None);
        let interp1 = Interpretation::Val(fs1);
        tree.apply_entry(&sn1, &interp1).unwrap();

        let sn2 = SyntaxNode::from_feature_str("Det");
        let mut fs2 = FeatureSet::new();
        fs2.insert("Det", None);
        fs2.insert("clause", Some("question"));
        let interp2 = Interpretation::Val(fs2);
        tree.apply_entry(&sn2, &interp2).unwrap();

        let mut fs3 = FeatureSet::new();
        fs3.insert("Det", None);
        fs3.insert("clause", Some("question"));
        let sn3 = SyntaxNode::from_feature_set(fs3);
        let mut fs4 = FeatureSet::new();
        fs4.insert("N", None);
        let mut fs5 = FeatureSet::new();
        fs5.insert("Det", None);
        fs5.insert("N", None);
        fs5.insert("clause", Some("question"));
        fs5.insert("case", Some("subj"));
        let interp3 = Interpretation::Lambda {
            from: fs4,
            to: fs5,
            projection: Direction::Left,
        };
        tree.todo_pop();
        tree.apply_entry(&sn3, &interp3).unwrap();
        println!("{:#?}", tree);
        
    }
}
