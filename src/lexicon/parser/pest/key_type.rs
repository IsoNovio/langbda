use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::str::FromStr;

pub trait KeyType:
    Clone + Copy + Debug + Display + FromStr + PartialEq + Eq + PartialOrd + Ord + Hash
{
}

impl<K: Clone + Copy + Debug + Display + FromStr + PartialEq + Eq + PartialOrd + Ord + Hash> KeyType
    for K
{
}
