use self::{
    automata::{Automata, Dfa},
    debug::AutomataPrinter,
    parser::parse_regex,
};

mod automata;
mod debug;
mod helper;
mod lexer;
mod parser;
mod tests;

pub use parser::Error;

#[derive(Debug)]
pub struct Regex {
    pub automaton: Dfa,
}

impl Regex {
    pub fn new(re: &str) -> Result<Self, Error> {
        Ok(Self {
            automaton: Automata::from_regex_expr(parse_regex(re)?),
        })
    }

    pub fn is_match(&self, text: &str) -> bool {
        self.automaton.validate_str(text)
    }

    pub fn debug_save_automata_to_file(&self, filename: &str) {
        let printer = AutomataPrinter::new(&self.automaton);
        printer.save_to_file(filename);
    }
}
