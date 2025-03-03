use crate::syntax_node::SyntaxNode;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TreeNode<'a> {
    value: SyntaxNode<'a>,
    idx: usize,
    parent: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

impl<'a> TreeNode<'a> {
    pub fn new(
        value: SyntaxNode<'a>,
        idx: usize,
        parent: Option<usize>,
        left: Option<usize>,
        right: Option<usize>,
    ) -> TreeNode<'a> {
        TreeNode {
            value,
            idx,
            parent,
            left,
            right,
        }
    }

    pub fn get_value(&self) -> &SyntaxNode<'a> {
        &self.value
    }

    pub fn get_value_mut(&mut self) -> &mut SyntaxNode<'a> {
        &mut self.value
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }

    pub fn set_idx(&mut self, idx: usize) {
        self.idx = idx;
    }

    pub fn get_parent(&self) -> Option<usize> {
        self.parent
    }

    pub fn set_parent(&mut self, parent: Option<usize>) {
        self.parent = parent;
    }

    pub fn get_left(&self) -> Option<usize> {
        self.left
    }

    pub fn set_left(&mut self, left: Option<usize>) {
        self.left = left;
    }

    pub fn get_right(&self) -> Option<usize> {
        self.right
    }

    pub fn set_right(&mut self, right: Option<usize>) {
        self.right = right;
    }
}

impl<'a> std::fmt::Display for TreeNode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.idx, self.value)
    }
}