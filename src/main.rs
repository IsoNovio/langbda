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
    let name = dialect.name();

    let examples = vec![
        ("the child ate an apple in the room.", "Sentence"),
        // ("the child did eat an apple.", "Sentence"), // still unsupported: see roadmap
        // ("did the child eat an apple?", "Sentence"), // still unsupported: see roadmap
        // ("whose apple did the child eat?", "Sentence"), // still unsupported: see roadmap
    ];

    for (sentence, target) in examples {
        println!("Interpreting \"{sentence}\" as {target} in {name}");
        let result = interpret::<_, LambdaModel<_>>(&dialect, sentence, target)?;

        println!("LANGBDA found {} interpretations.", result.len());
        let filename_sentence = sentence
            .chars()
            .map(|c| if c.is_alphanumeric() { c } else { '-' })
            .collect::<String>();
        for (index, actions) in result.into_iter().enumerate() {
            let mut tree = follow::<_, TreeModel<_>>(target, actions)?;
            tree.prune().map_err(cognitive::Error::from)?;

            let filename = format!(
                "assets/examples/{}_tree-{}.png",
                filename_sentence,
                index + 1
            );
            tree.to_png(filename).map_err(cognitive::Error::from)?;
        }
    }

    Ok(())
}
