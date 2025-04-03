use super::super::super::{Lexicon, LexiconNode};
use super::super::LexiconParser;
use super::feature_topology::FeatureTopology;
use super::key_type::KeyType;
use crate::syntax::{FeatureSet, SyntaxValue};

use pest::Parser;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "lexicon/parser/pest/lexicon.pest"]
pub struct PestLexiconParser;

impl<K: KeyType> LexiconParser<K> for PestLexiconParser {
    fn parse_str(lexicon: &mut impl Lexicon<K>, input: &str) -> Result<(), String> {
        parse_lexicon(lexicon, input)
    }
}

fn parse_lexicon<K: KeyType>(lexicon: &mut impl Lexicon<K>, input: &str) -> Result<(), String> {
    let pairs = PestLexiconParser::parse(Rule::lexicon, input).map_err(|e| e.to_string())?;
    parse_sections(lexicon, pairs)
}

fn parse_sections<K: KeyType>(
    lexicon: &mut impl Lexicon<K>,
    pairs: Pairs<Rule>,
) -> Result<(), String> {
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
            r => unreachable!("Parsing sections: unexpected rule: {r:?}"),
        }
    }

    // parse feature entries into feature topology
    let mut feature_topology: FeatureTopology<K> = FeatureTopology::new();
    parse_feature_entries(&mut feature_topology, feature_entries)?;

    // parse functional entries
    parse_functional_entries(lexicon, functional_entries, &feature_topology)?;

    // parse lexical entries
    parse_lexical_entries(lexicon, lexical_entries, &feature_topology)?;

    Ok(())
}

type SubstitutionTable<K> = HashMap<K, (K, Option<K>)>;
type ParserState<T, K> = Vec<(T, SubstitutionTable<K>)>;

fn parse_feature_entries<K: KeyType>(
    ft: &mut FeatureTopology<K>,
    pairs: Vec<Pair<Rule>>,
) -> Result<(), String> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::feature_entry => {
                let mut inner = pair.into_inner();

                let pair_cat = inner.next().expect("Parsing feature entry: no category");
                let category = parse_word(pair_cat)?;

                let pair_vals = inner.next().expect("Parsing feature entry: no values");
                for pair_val in pair_vals.into_inner() {
                    let val = parse_word(pair_val)?;
                    ft.insert(category, val);
                }
            }
            r => unreachable!("Parsing feature entries: unexpected rule: {r:?}"),
        }
    }
    Ok(())
}

fn parse_functional_entries<K: KeyType>(
    lexicon: &mut impl Lexicon<K>,
    pairs: Vec<Pair<Rule>>,
    ft: &FeatureTopology<K>,
) -> Result<(), String> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::functional_entry => {
                let mut inner = pair.into_inner();
                let pair_fset = inner.next().expect("Parsing functional entry: no from");
                let pair_lnode = inner.next().expect("Parsing functional entry: no to");

                let subst = SubstitutionTable::new();
                for (from, subst) in parse_feature_set(pair_fset, ft, subst)? {
                    for (to, _) in parse_syntax_node(pair_lnode.clone(), ft, subst)? {
                        lexicon.add_entry(SyntaxValue::Features(from.clone()), to);
                    }
                }
            }
            r => unreachable!("Parsing functional entries: unexpected rule: {r:?}"),
        }
    }
    Ok(())
}

fn parse_syntax_node<K: KeyType>(
    pair: Pair<Rule>,
    ft: &FeatureTopology<K>,
    subst: SubstitutionTable<K>,
) -> Result<ParserState<LexiconNode<K>, K>, String> {
    match pair.as_rule() {
        Rule::lambda => parse_lambda(pair, ft, subst),
        Rule::moved => parse_moved(pair, ft, subst),
        Rule::feature_set => parse_feature_set(pair, ft, subst).map(|ps| {
            ps.into_iter()
                .map(|(fs, subst)| {
                    (
                        LexiconNode::Value {
                            value: SyntaxValue::Features(fs),
                        },
                        subst,
                    )
                })
                .collect()
        }),
        r => unreachable!("Parsing syntax node: unexpected rule: {r:?}"),
    }
}

fn parse_lambda<K: KeyType>(
    pair: Pair<Rule>,
    ft: &FeatureTopology<K>,
    subst: SubstitutionTable<K>,
) -> Result<ParserState<LexiconNode<K>, K>, String> {
    let mut inner = pair.into_inner();
    let pair_l = inner.next().expect("Parsing lambda: no left");
    let pair_dir = inner.next().expect("Parsing lambda: no direction");
    let pair_r = inner.next().expect("Parsing lambda: no right");

    let mut res = Vec::new();
    let project = parse_project(pair_dir)?;
    for (l_lnode, subst) in parse_syntax_node(pair_l, ft, subst)? {
        for (r_fset, subst) in parse_feature_set(pair_r.clone(), ft, subst)? {
            res.push((
                LexiconNode::Lambda {
                    from: Box::new(l_lnode.clone()),
                    to: r_fset,
                    project,
                },
                subst,
            ));
        }
    }
    Ok(res)
}

fn parse_moved<K: KeyType>(
    pair: Pair<Rule>,
    ft: &FeatureTopology<K>,
    subst: SubstitutionTable<K>,
) -> Result<ParserState<LexiconNode<K>, K>, String> {
    let pair_fset = pair
        .into_inner()
        .next()
        .expect("Parsing moved: no features");
    let res = parse_feature_set(pair_fset, ft, subst)?;
    Ok(res
        .into_iter()
        .map(|(fset, subst)| (LexiconNode::Moved { from: fset }, subst))
        .collect())
}

fn parse_feature_set<K: KeyType>(
    pair: Pair<Rule>,
    ft: &FeatureTopology<K>,
    subst: SubstitutionTable<K>,
) -> Result<ParserState<FeatureSet<K>, K>, String> {
    let fset = FeatureSet::new();
    let mut res = vec![(fset, subst)];
    for pair_f in pair.into_inner() {
        let mut new_res = Vec::new();
        while let Some((fset, subst)) = res.pop() {
            let feature_parses = parse_feature(pair_f.clone(), ft, subst)?;
            for ((feature_cat, feature_val), subst) in feature_parses {
                let mut fset = fset.clone();
                fset.insert(feature_cat, feature_val);
                new_res.push((fset, subst));
            }
        }
        res = new_res;
    }

    Ok(res)
}

fn parse_feature<K: KeyType>(
    pair: Pair<Rule>,
    ft: &FeatureTopology<K>,
    subst: SubstitutionTable<K>,
) -> Result<ParserState<(K, Option<K>), K>, String> {
    let k = parse_word(pair)?;
    match subst.get(&k) {
        Some(sub) => Ok(vec![(sub.to_owned(), subst)]),
        None => {
            if ft.is_category(&k) {
                let vals = ft
                    .get_from_category(&k)
                    .expect("Parsing feature: category {k} has no values");
                let mut res = Vec::new();
                for val in vals {
                    let mut subst = subst.clone();
                    let sub = (k, Some(val.to_owned()));
                    subst.insert(k, sub.to_owned());
                    res.push((sub, subst));
                }
                Ok(res)
            } else {
                let sub = match ft.get_from_value(&k) {
                    Some(cat) => (cat, Some(k)),
                    None => (k, None),
                };
                let mut subst = subst.clone();
                subst.insert(k, sub);
                Ok(vec![(sub, subst)])
            }
        }
    }
}

fn parse_project(pair: Pair<Rule>) -> Result<bool, String> {
    match pair.as_rule() {
        Rule::right_projection => Ok(true),
        Rule::left_projection => Ok(false),
        r => unreachable!("Parsing direction: unexpected rule: {r:?}"),
    }
}

fn parse_word<K: KeyType>(pair: Pair<Rule>) -> Result<K, String> {
    let s = pair.as_str().trim();
    K::from_str(s).map_err(|_| format!("FromStr: Cannot parse {s}"))
}

fn parse_lexical_entries<K: KeyType>(
    lexicon: &mut impl Lexicon<K>,
    pairs: Vec<Pair<Rule>>,
    topology: &FeatureTopology<K>,
) -> Result<(), String> {
    for pair in pairs {
        match pair.as_rule() {
            Rule::lexical_entry => {
                let mut inner = pair.into_inner();
                let pair_lexi = inner.next().expect("Parsing lexical entry: no from");
                let pair_lnode = inner.next().expect("Parsing lexical entry: no to");

                let from = SyntaxValue::Item(parse_word(pair_lexi)?);
                let subst = SubstitutionTable::new();
                for (to, _) in parse_syntax_node(pair_lnode, topology, subst)? {
                    lexicon.add_entry(from.clone(), to);
                }
            }
            r => unreachable!("Parsing lexical entries: unexpected rule: {r:?}"),
        }
    }
    Ok(())
}
