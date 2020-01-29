use crate::scanner::Scanner;

pub struct Compiler;

impl Compiler {
    pub fn compile(&self, source: &str) {
        Scanner::new(source);
    }
}
