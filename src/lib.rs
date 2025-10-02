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
