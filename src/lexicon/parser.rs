use crate::feature::FeatureSet;
use crate::direction::Direction;

use super::feature_topology::FeatureTopology;
use super::interpretation::Interpretation;
use super::lexicon::Lexicon;

use std::collections::{HashSet, HashMap};
use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "lexicon/lexicon.pest"]
pub struct LexiconParser;

impl LexiconParser {
    pub fn parse_lexicon(input: &str) -> Result<Lexicon, String> {
        match LexiconParser::parse(Rule::lexicon, input) {
            Ok(pairs) => Ok(parse_lexicon(pairs)?),
            Err(e) => Err(format!("{}", e)),
        }        
    }
}

fn parse_lexicon<'a>(pairs: Pairs<'a, Rule>) -> Result<Lexicon<'a>, String> {
    let mut lexicon = Lexicon::new();
    let mut feature_entries = Vec::new();
    let mut functional_entries = Vec::new();
    let mut lexical_entries = Vec::new();

    // collect all entries
    for pair in pairs {
        match pair.as_rule() {
            Rule::feature_section => feature_entries.extend(pair.into_inner()),
            Rule::functional_section => functional_entries.extend(pair.into_inner()),
            Rule::lexical_section => lexical_entries.extend(pair.into_inner()),
            Rule::EOI => (),
            _ => return Err(format!("Unexpected rule in lexicon: {:?}", pair)),
        }
    }

    // parse feature entries into feature topology
    let mut feature_topology = FeatureTopology::new();
    parse_feature_topology(&mut feature_topology, feature_entries)?;

    // parse functional entries
    parse_functional_entries(&mut lexicon, functional_entries, &feature_topology)?;

    // parse lexical entries
    parse_lexical_entries(&mut lexicon, lexical_entries, &feature_topology)?;

    Ok(lexicon)
}

fn parse_feature_topology<'a>(ft: &mut FeatureTopology<'a>, pairs: Vec<Pair<'a, Rule>>) -> Result<(), String> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::feature_entry => {
                let mut inner_pairs = pair.into_inner();
                let pair_cat = inner_pairs.next().ok_or("feature_entry has no category")?;
                let pair_vals = inner_pairs.next().ok_or("feature_entry has no values")?;
                let category = pair_cat.as_str().trim();
                for pair_val in pair_vals.into_inner() {
                    let val = pair_val.as_str().trim();
                    ft.insert(category, val);
                }
            }
            _ => return Err(format!("Unexpected rule in features: {:#?}", pair)),
        }
    }
    Ok(())
}

fn parse_functional_entries<'a>(lexicon: &mut Lexicon<'a>, pairs: Vec<Pair<'a, Rule>>, topology: &FeatureTopology<'a>) -> Result<(), String> {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum FuncFeatLoc { Input, From, To }
    enum FuncProjection { Left, Right, None}
    type FuncEntry<'a> = HashMap<&'a str, HashSet<FuncFeatLoc>>;

    fn extract_features<'a>(func_entry: &mut FuncEntry<'a>, loc: FuncFeatLoc, pairs: Pairs<'a, Rule>) -> Result<(), String> {
        for pair in pairs {
            match pair.as_rule() {
                Rule::feature => {
                    func_entry.entry(pair.as_str().trim()).or_default().insert(loc.clone());
                },
                _ => return Err(format!("Unexpected rule in feature set: {:#?}", pair)),
            }
        }
        Ok(())
    }

    fn parse_entry<'a>(lexicon: &mut Lexicon<'a>, func_entry: FuncEntry<'a>, projection: &FuncProjection, topology: &FeatureTopology<'a>) -> Result<(), String> {
        for (feature, locs) in func_entry.iter() {
            if topology.is_category(feature) {
                for val in topology.get_from_category(feature).ok_or("category has not value")? {
                    let mut new_entry = func_entry.clone();
                    new_entry.remove(feature);
                    new_entry.insert(val, locs.clone());
                    parse_entry(lexicon, new_entry, projection, topology)?;
                }
                return Ok(());
            }
        }

        let mut fs_input = FeatureSet::new();
        let mut fs_from = FeatureSet::new();
        let mut fs_to = FeatureSet::new();
        for (feature, locs) in func_entry {
            for loc in locs {
                match loc {
                    FuncFeatLoc::Input => fs_input.insert_feature(topology.to_feature(feature))?,
                    FuncFeatLoc::From => fs_from.insert_feature(topology.to_feature(feature))?,
                    FuncFeatLoc::To => fs_to.insert_feature(topology.to_feature(feature))?,
                }
            }
        }

        let interp = match projection {
            FuncProjection::None => Interpretation::Val(fs_to),
            FuncProjection::Left => Interpretation::Lambda { from: fs_from, to: fs_to, projection: Direction::Left },
            FuncProjection::Right => Interpretation::Lambda { from: fs_from, to: fs_to, projection: Direction::Right },
        };

        lexicon.insert_functional(fs_input, interp);
        Ok(())
    }

    for pair in pairs {
        match pair.as_rule() {
            Rule::functional_entry => {
                let mut func_entry: FuncEntry<'a> = HashMap::new();
                let mut func_entry_projection = FuncProjection::None;
                
                let mut inner_pairs = pair.into_inner();
                let pair_input = inner_pairs.next().ok_or("functional_entry has no input features")?;
                let pair_interp: Pair<'a, Rule> = inner_pairs.next().ok_or("functional_entry has no interpretation")?;
                extract_features(&mut func_entry, FuncFeatLoc::Input, pair_input.into_inner())?;
                
                match pair_interp.as_rule() {
                    Rule::feature_set => extract_features(&mut func_entry, FuncFeatLoc::To, pair_interp.into_inner())?,
                    projection_direction @ (Rule::left_projection | Rule::right_projection) => {
                        let mut inner_pairs = pair_interp.into_inner();
                        let pair_from = inner_pairs.next().ok_or("interpretation has no from")?;
                        let pair_to = inner_pairs.next().ok_or("interpretation has no to")?;
                        extract_features(&mut func_entry, FuncFeatLoc::From, pair_from.into_inner())?;
                        extract_features(&mut func_entry, FuncFeatLoc::To, pair_to.into_inner())?;
                        func_entry_projection = match projection_direction {
                            Rule::left_projection => FuncProjection::Left,
                            Rule::right_projection => FuncProjection::Right,
                            _ => return Err("Impossible".to_string())
                        };
                    },
                    _ => return Err(format!("Unexpected rule in interpretation: {:#?}", pair_interp)),
                }

                parse_entry(lexicon, func_entry, &func_entry_projection, topology)?
            },
            _ => return Err(format!("Unexpected rule in functional entries: {:#?}", pair)),
        }
    }
    Ok(())
}

fn parse_lexical_entries<'a>(lexicon: &mut Lexicon<'a>, pairs: Vec<Pair<'a, Rule>>, topology: &FeatureTopology<'a>) -> Result<(), String> {
    fn parse_feature_set<'a>(pairs: Pairs<'a, Rule>, topology: &FeatureTopology<'a>) -> Result<FeatureSet<'a>, String> {
        let mut fs = FeatureSet::new();
        for pair in pairs {
            match pair.as_rule() {
                Rule::feature => {
                    let feature_str = pair.as_str().trim();
                    let feature = topology.to_feature(feature_str);
                    fs.insert_feature(feature)?
                },
                _ => return Err(format!("Unexpected rule in feature set: {:#?}", pair)),
            }
        }
        Ok(fs)
    }
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::lexical_entry => {
                let mut inner_pairs = pair.into_inner();
                let pair_lexi = inner_pairs.next().ok_or("lexical_entry has no lexical item")?;
                let pair_features = inner_pairs.next().ok_or("lexical_entry has no feature set")?;
                let lexi = pair_lexi.as_str().trim();
                let features = parse_feature_set(pair_features.into_inner(), topology)?;
                lexicon.insert_lexical(lexi, features);
            },
            _ => return Err(format!("Unexpected rule in lexical entries: {:#?}", pair)),
        }
    }
    Ok(())
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pest_lexicon() {
        let inputfile = "src/lexicons/en.lexicon";
        let input = std::fs::read_to_string(inputfile).unwrap();
        match LexiconParser::parse(Rule::lexicon, &input) {
            Ok(o) => println!("{:#?}", o),
            Err(e) => println!("{}", e),
        };
    }

    #[test]
    fn parse_lexicon() {
        let inputfile = "src/lexicons/en.lexicon";
        let input = std::fs::read_to_string(inputfile).unwrap();
        let lexicon = LexiconParser::parse_lexicon(&input).unwrap();
        println!("{}", lexicon);
    }
}
