use super::action::Action;
use super::error::{Error, Result};
use crate::cognitive::CognitiveModel;
use crate::dialect::Dialect;
use crate::lexicon::Lexicon;
use crate::syntax::FeatureSet;
use crate::tokenizer::Tokenizer;
use log::debug;
use std::fmt::Display;
use std::str::FromStr;

pub type Actions<K> = Vec<Action<K>>;

pub fn interpret<K: FromStr + Clone + Ord + Display, C: CognitiveModel<K> + Display>(
    dialect: &impl Dialect<K>,
    text: &str,
    target: &str,
) -> Result<Vec<Actions<K>>> {
    let target = K::from_str(target).map_err(|_| Error::FromStr)?;
    let target = FeatureSet::from_category(target);
    let cogmodel = C::init(target);

    let mut res = Vec::new();
    let mut actions = Vec::new();

    step(dialect, &mut res, &mut actions, cogmodel, text)?;
    fn step<K: FromStr + Clone + Display, C: CognitiveModel<K> + Display>(
        dialect: &impl Dialect<K>,
        res: &mut Vec<Actions<K>>,
        actions: &mut Actions<K>,
        cogmodel: C,
        text: &str,
    ) -> Result<()> {
        debug!("remaining: {}", text);
        debug!("model: {}", cogmodel);

        if text.is_empty() && cogmodel.understood() {
            res.push(actions.clone());
            return Ok(());
        }

        if cogmodel.demand() {
            for (newtoken, remainder) in dialect.tokenizer().tokenize(text) {
                actions.push(Action::AddToken(newtoken.clone()));
                let mut cogmodel = cogmodel.clone();
                if let Ok(()) = cogmodel.receive(newtoken) {
                    step(dialect, res, actions, cogmodel, remainder)?;
                }
                actions.pop();
            }
        }

        if let Some(value) = cogmodel.wonder() {
            for entry in dialect.lexicon().get_entries(value) {
                actions.push(Action::ApplyEntry(entry.clone()));
                let mut cogmodel = cogmodel.clone();
                if let Ok(()) = cogmodel.decide(entry) {
                    step(dialect, res, actions, cogmodel, text)?;
                }
                actions.pop();
            }
        }

        Ok(())
    }

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cognitive::NaiveModel;
    use crate::dialect::English;
    use crate::interner::GlobalKey;

    #[test]
    fn test_cogmodel() {
        let dialect = English::default();
        let res = interpret::<_, NaiveModel>(&dialect, "Hello, world!", "S").unwrap();
        let shouldbe = vec![
            ["Hello", ",", "world", "!"]
                .map(|s| Action::AddToken(GlobalKey::from_str(s).unwrap()))
                .into_iter()
                .collect::<Vec<_>>(),
        ];
        assert_eq!(res, shouldbe);
    }
}
