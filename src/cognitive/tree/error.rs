use super::NodeID;
use derive_more::{Display, From};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Display, From)]
pub enum Error {
    #[from(ignore)]
    NodeNotFound(NodeID),

    #[from(ignore)]
    NodeHasNoParent(NodeID),

    #[display("Node {} has no child with ID {}", _0, _1)]
    #[from(ignore)]
    NodeHasNoChildWithID(NodeID, NodeID),

    #[display("Node {} has no parent with ID {}", _0, _1)]
    #[from(ignore)]
    NodeHasNoParentWithID(NodeID, NodeID),

    #[from(ignore)]
    NodeIsNotFeatures(NodeID),

    Syntax(crate::syntax::Error),

    LambdaToIsNestedLambda,

    StdFmt(std::fmt::Error),
    StdIO(std::io::Error),

    GraphvizDotNotInstalled,
}

impl std::error::Error for Error {}
