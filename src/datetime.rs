use miette::Error;

use crate::interpreter::Interpreter;

#[derive(Debug, PartialEq, Eq)]
pub struct Datetime {
    pub(crate) ano: usize,
    pub(crate) mes: usize,
    pub(crate) dia: usize,
    pub(crate) hora: usize,
    pub(crate) minuto: usize,
    pub(crate) segundo: usize,
    pub(crate) milissegundo: usize,
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

impl Datetime {
    pub fn from_str(date: String, date_format: &str) -> Result<Self, Error> {
        let mut date = date;
        Interpreter::parse_datetime(&mut date, date_format)
    }
}
