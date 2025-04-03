mod error;
mod node;
mod r#struct;

/// needs: Copy
pub type NodeID = usize;
pub use error::Error;
pub use r#struct::TreeModel;
