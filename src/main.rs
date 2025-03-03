mod feature;
mod direction;
mod bintree;
mod lexicon;
mod syntax_node;
mod sentence;
mod interpreter;

use crate::lexicon::Lexicon;
use crate::interpreter::{Interpreter, LOT};

fn main() {
    let sentence = "Whose naughty child ate the apple in the room?";
    let target = "S";
    let lexicon_file = "src/lexicons/en.lexicon";
    let lexicon_input = std::fs::read_to_string(lexicon_file).unwrap();

    let lexicon = Lexicon::from_str(&lexicon_input).unwrap();

    let mut interp = Interpreter::new(sentence, target, &lexicon);
    let results = interp.run().unwrap_or(Vec::new());
    let results = LOT::from_syntax_trees(&results);
    println!("There are {} parsing results:", results.len());
    for result in results.iter() {
        println!("{}", result);
    }
}
