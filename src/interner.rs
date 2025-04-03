use lasso::{Spur, ThreadedRodeo};
use std::sync::{LazyLock, Mutex};

static GLOBAL_INTERNER: LazyLock<Mutex<ThreadedRodeo>> =
    LazyLock::new(|| Mutex::new(ThreadedRodeo::new()));

#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct GlobalKey(Spur);

fn global_get(token: &str) -> GlobalKey {
    let interner = GLOBAL_INTERNER.lock().unwrap();
    GlobalKey(interner.get_or_intern(token))
}

fn global_resolve(token: &GlobalKey) -> Option<String> {
    let interner = GLOBAL_INTERNER.lock().unwrap();
    interner.try_resolve(&token.0).map(|s| s.to_string())
}

impl std::fmt::Display for GlobalKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", global_resolve(self).ok_or(std::fmt::Error)?)
    }
}

impl std::str::FromStr for GlobalKey {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(global_get(s))
    }
}

// impl KeyType for GlobalKey {}
