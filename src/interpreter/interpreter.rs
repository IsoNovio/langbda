use super::syntax_tree::SyntaxTree;
use super::todo_task::TodoTask;
use crate::lexicon::Lexicon;
use crate::sentence::Sentence;

pub struct Interpreter<'a> {
    sentence: Sentence,
    target: &'a str,
    lexicon: &'a Lexicon<'a>,
}

pub struct InterpreterState<'a> {
    states: Vec<SyntaxTree<'a>>,
}

impl<'a> InterpreterState<'a> {
    pub fn new() -> InterpreterState<'a> {
        InterpreterState { states: Vec::new() }
    }

    pub fn push(&mut self, tree: SyntaxTree<'a>) {
        self.states.push(tree);
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }
}

pub struct InterpreterResult<'a> {
    results: Vec<SyntaxTree<'a>>,
}

impl<'a> InterpreterResult<'a> {
    pub fn new() -> InterpreterResult<'a> {
        InterpreterResult { results: Vec::new() }
    }

    pub fn push(&mut self, tree: SyntaxTree<'a>) {
        self.results.push(tree);
    }
}

impl<'a> Interpreter<'a> {
    pub fn new(sentence: &'a str, target: &'a str, lexicon: &'a Lexicon<'a>) -> Interpreter<'a> {
        Interpreter {
            sentence: Sentence::from_str(sentence).unwrap(),
            target,
            lexicon,
        }
    }

    pub fn run(&'a mut self) -> Result<Vec<SyntaxTree<'a>>, String> {
        let syntax_tree = SyntaxTree::init(&self.sentence, self.target)?;
        let mut interpreter_state = InterpreterState::new();
        interpreter_state.states.push(syntax_tree);
        let mut interpreter_result = InterpreterResult::new();

        while !interpreter_state.is_empty() {
            let states = std::mem::take(&mut interpreter_state.states);
            for state in states {
                self.run_state(&mut interpreter_state, &mut interpreter_result, state)?;
            }
        }

        Ok(interpreter_result.results)
    }

    fn run_state(&self, states: &mut InterpreterState<'a>, results: &mut InterpreterResult<'a>, tree: SyntaxTree<'a>) -> Result<(), String> {
        if tree.job_done() {
            results.push(tree);
            return Ok(());
        }

        match tree.todo_peek().ok_or("todo stack is empty")? {
            TodoTask::Project { from: _, to: _, ignore: _ } => {
                let mut new_tree = tree.clone();
                match new_tree.do_projection() {
                    Ok(()) => states.push(new_tree),
                    Err(_) => {}
                }
            }
            TodoTask::Val(_) => {
                let mut new_tree = tree.clone();
                match new_tree.add_token() {
                    Ok(()) => states.push(new_tree),
                    Err(_) => {}
                }
            }
            TodoTask::Lambda { from: input_idx, to: output_idx } => {
                let input_idx = *input_idx;
                let input_sn = tree.get_node(input_idx).ok_or(format!("Node {} does not exist", input_idx))?.get_value();
                let output_idx = *output_idx;
                let output_sn = tree.get_node(output_idx).ok_or(format!("Node {} does not exist", output_idx))?.get_value();
                
                if output_sn.is_subset(&input_sn) {
                    let mut new_tree = tree.clone();
                    match new_tree.delete_identity_lambda() {
                        Ok(()) => states.push(new_tree),
                        Err(_) => {}
                    };
                }
                for (from_sn, to_interp_vec) in self.lexicon.iter_all() {
                    if from_sn.is_subset(input_sn) {
                        for to_interp in to_interp_vec.iter() {
                            let mut new_tree = tree.clone();
                            match new_tree.apply_entry(from_sn, to_interp) {
                                Ok(()) => states.push(new_tree),
                                Err(_) => {}
                            };
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
