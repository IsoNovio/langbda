use super::action::Action;
use super::error::{Error, Result};
use crate::cognitive::CognitiveModel;
use crate::syntax::FeatureSet;
use log::debug;
use std::fmt::Display;
use std::str::FromStr;

pub fn follow<K: Clone + FromStr + Ord + Display, C: CognitiveModel<K> + Display>(
    target: &str,
    actions: Vec<Action<K>>,
) -> Result<C> {
    let target = K::from_str(target).map_err(|_| Error::FromStr)?;
    let target = FeatureSet::from_category(target);
    let mut cogmodel = C::init(target);

    for action in actions {
        debug!("{action}");
        match action {
            Action::AddToken(token) => cogmodel.receive(token)?,
            Action::ApplyEntry(entry) => cogmodel.decide(entry)?,
        }
        debug!("{cogmodel}");
    }

    Ok(cogmodel)
}
