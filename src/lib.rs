use std::str::FromStr;
use thiserror::Error;
use miette::Diagnostic;

#[derive(Debug)]
struct Datetime {
    Ano: usize,
    Mes: usize,
    Dia: usize,
    Hora: usize,
    Minuto: usize,
    Segundo: usize,
    Milissegundo: usize,
}

enum Token {
    FullYear,
    HalfYear,
    FullMonth,
    WrittenMonth,
    FullDay,
    Hour,
    Minute,
    Second,
}


#[derive(Error, Diagnostic, Debug)]
enum DateTimeError;

impl FromStr for Token {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Datetime {
    fn parse_isotime(date: &str) -> Self {
        let string_parts: Vec<_> = date.split('-').collect();
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
