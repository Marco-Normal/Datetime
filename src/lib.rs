pub mod interpreter;
pub mod lexer;

#[derive(Debug, Default)]
pub struct Datetime {
    ano: usize,
    mes: usize,
    dia: usize,
    hora: usize,
    minuto: usize,
    segundo: usize,
    milissegundo: usize,
}
