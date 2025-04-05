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

    let sentence = "the child ate an apple in the room.";
    let target = "Sentence";
    let name = dialect.name();
    println!("Interpreting \"{sentence}\" as {target} in {name}");
    let result = interpret::<_, LambdaModel<_>>(&dialect, sentence, target)?;

    println!("LANGBDA found {} interpretations.", result.len());
    for actions in result {
        let mut tree = follow::<_, TreeModel<_>>(target, actions)?;
        tree.prune().unwrap();
        println!("Tree:\n{}", tree);
    }

    Ok(())
}
