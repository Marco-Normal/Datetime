pub mod interpreter;
pub mod lexer;

#[derive(Debug, PartialEq, Eq)]
pub struct Datetime {
    ano: usize,
    mes: usize,
    dia: usize,
    hora: usize,
    minuto: usize,
    segundo: usize,
    milissegundo: usize,
}

impl Default for Datetime {
    fn default() -> Self {
        Self {
            ano: 1900,
            mes: 1,
            dia: 1,
            hora: 24,
            minuto: 00,
            segundo: 00,
            milissegundo: 00,
        }
    }
}
