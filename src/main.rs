mod cognitive;
mod dialect;
mod error;
mod interner;
mod interpreter;
mod lexicon;
mod logger;
mod syntax;
mod tokenizer;
mod trie;

use self::cognitive::{LambdaModel, TreeModel};
use self::dialect::{Dialect, English};
use self::error::Result;
use self::interpreter::{follow, interpret};
use self::logger::init_logger;

fn main() -> Result<()> {
    init_logger();

    let dialect = English::init();
    println!("{}", dialect);

    let sentence = "the child ate an apple.";
    let target = "Sentence";
    println!(
        "Interpreting \"{sentence}\" as {target} in {}",
        dialect.name()
    );
    let result = interpret::<_, LambdaModel<_>>(&dialect, sentence, target)?;
    for actions in result {
        let tree = follow::<_, TreeModel<_>>(target, actions)?;
        println!("Tree:\n{}", tree);
    }

    Ok(())
}
