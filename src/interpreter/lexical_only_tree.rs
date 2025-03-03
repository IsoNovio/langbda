use super::syntax_tree::SyntaxTree;
use super::tree_node::TreeNode;
use crate::syntax_node::SyntaxNode;

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LOT<'a> {
    Node { left: Box<LOT<'a>>, right: Box<LOT<'a>> },
    Leaf { value: &'a str },
    Empty
}

impl<'a> LOT<'a> {
    fn new() -> Self {
        LOT::Node {
            left: Box::new(LOT::Empty),
            right: Box::new(LOT::Empty),
        }
    }

    fn empty() -> Self {
        LOT::Empty
    }

    fn new_node_left(&mut self) -> Result<(), String> {
        match self {
            LOT::Node { left, .. } => {
                *left = Box::new(LOT::new());
                Ok(())
            }
            _ => Err("Not a node".to_string()),
        }
    }

    fn new_node_right(&mut self) -> Result<(), String> {
        match self {
            LOT::Node { right, .. } => {
                *right = Box::new(LOT::new());
                Ok(())
            }
            _ => Err("Not a node".to_string()),
        }
    }

    fn new_leaf_left(&mut self, value: &'a str) -> Result<(), String> {
        match self {
            LOT::Node { left, .. } => {
                *left = Box::new(LOT::Leaf { value: value });
                Ok(())
            }
            _ => Err("Not a node".to_string()),
        }
    }

    fn new_leaf_right(&mut self, value: &'a str) -> Result<(), String> {
        match self {
            LOT::Node { right, .. } => {
                *right = Box::new(LOT::Leaf { value: value });
                Ok(())
            }
            _ => Err("Not a node".to_string()),
        }
    }
}

impl<'a> LOT<'a> {
    pub fn from_syntax_tree(tree: &'a SyntaxTree<'a>) -> LOT<'a> {
        fn from_subtree<'a>(tree: &'a SyntaxTree<'a>, node: &'a TreeNode) -> LOT<'a> {
            match (tree.get_left(node), tree.get_right(node)) {
                (Some(left), Some(right)) => {
                    LOT::Node {
                        left: Box::new(from_subtree(tree, left)),
                        right: Box::new(from_subtree(tree, right)),
                    }
                }
                (Some(left), None) => {
                    from_subtree(tree, left)
                }
                (None, Some(right)) => {
                    from_subtree(tree, right)
                }
                (None, None) => {
                    match node.get_value() {
                        SyntaxNode::Lexi(lexi) => LOT::Leaf { value: lexi },
                        _ => LOT::empty(),
                    }
                }
            }
        }
        
        match tree.get_root() {
            Some(root) => {
                from_subtree(tree, root)
            }
            None => LOT::empty(),
        }
    }

    pub fn from_syntax_trees(trees: &'a Vec<SyntaxTree<'a>>) -> Vec<LOT<'a>> {
        let mut results = HashSet::new();
        for tree in trees {
            results.insert(LOT::from_syntax_tree(tree));
        }
        results.into_iter().collect()
    }
}

impl<'a> std::fmt::Display for LOT<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn fmt_subtree(tree: &LOT, depth: usize) -> String {
            match tree {
                LOT::Node { left, right } => {
                    let left = fmt_subtree(left, depth + 1);
                    let right = fmt_subtree(right, depth + 1);
                    format!("\n{}[{} {}]", " ".repeat(depth * 4), left, right)
                }
                LOT::Leaf { value } => {
                    format!("\n{}[{}]", " ".repeat(depth * 4), value)
                }
                LOT::Empty => String::new(),
            }
        }

        let fmt_tree = fmt_subtree(self, 0);
        write!(f, "{}", fmt_tree)

        // match self {
        //     this @ LOT::Node { left, right } => {
        //         let fmt_tree = fmt_subtree(this, 0).unwrap_or("NO TREE".to_string());
        //         write!(f, "{}", fmt_tree)
        //     }
        //     LOT::Leaf { value } => write!(f, "{}", value)
        // }
    }
}
