use super::error::Result;
use crate::lexicon::LexiconEntry;
use crate::syntax::{FeatureSet, SyntaxValue};

pub trait CognitiveModel<K>: Clone {
    fn init(target: FeatureSet<K>) -> Self;
    fn understood(&self) -> bool;
    fn receive(&mut self, token: K) -> Result<()>;
    fn wonder(&self) -> Option<&SyntaxValue<K>>;
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
