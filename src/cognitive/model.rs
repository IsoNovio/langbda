use super::error::Result;
use crate::lexicon::LexiconEntry;
use crate::syntax::{FeatureSet, SyntaxValue};

pub trait CognitiveModel<K>: Clone {
    /// create a new cognitive model for interpreting a sentence as a target feature
    fn init(target: FeatureSet<K>) -> Self;

    /// whether the cognitive process has completed
    fn understood(&self) -> bool;

    /// whether the cognitive model wants to take in a new token
    fn demand(&self) -> bool;

    /// adds a new token to the cognitive model
    fn receive(&mut self, token: K) -> Result<()>;

    /// what does the cognitive model wants interpretation for
    fn wonder(&self) -> Option<&SyntaxValue<K>>;

    /// apply an entry as an interpretation to the result of wonder()
    fn decide(&mut self, entry: LexiconEntry<K>) -> Result<()>;
}

#[cfg(test)]
pub mod naive_model {
    use super::*;
    use crate::interner::GlobalKey;
    use derive_more::Display;

    #[derive(Clone, Debug, Display)]
    pub struct NaiveModel {}
    impl CognitiveModel<GlobalKey> for NaiveModel {
        fn init(target: FeatureSet<GlobalKey>) -> Self {
            println!("NaiveInterpreter: Initializing for {target}");
            NaiveModel {}
        }
        fn understood(&self) -> bool {
            true
        }
        fn demand(&self) -> bool {
            true
        }
        fn receive(&mut self, token: GlobalKey) -> Result<()> {
            println!("NaiveInterpreter: Adding token {token}");
            Ok(())
        }
        fn wonder(&self) -> Option<&SyntaxValue<GlobalKey>> {
            None
        }
        fn decide(&mut self, entry: LexiconEntry<GlobalKey>) -> Result<()> {
            println!("NaiveInterpreter: Applying entry {entry}");
            Ok(())
        }
    }
}
