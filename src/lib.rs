pub mod lexer;
pub mod interpreter;
pub use run::run;

mod run {
    use std::io;
    use crate::interpreter::Interpreter;
    use crate::lexer::{Lexer};
    use crate::lexer;
    
    pub fn run<'a>(program: &str, stdout: &'a mut dyn io::Write) {
        let mut commands = Lexer::new(&lexer::pre_lex(program)).parse();
        let mut interpreter = Interpreter::new(&mut commands, stdout);
        interpreter.run();
    }
}