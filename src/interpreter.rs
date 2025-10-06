use super::Datetime;
use crate::lexer::{DateTimeLexer, Token};
use miette::{Diagnostic, Error, IntoDiagnostic};
use thiserror::Error;

pub struct Interpreter;

#[derive(Debug, Error, Diagnostic)]
enum InterpreterError {
    #[error("Unexpect sequence. Expected `{}`, got `{}`", expected, unexpected)]
    WrongSequence {
        expected: String,
        unexpected: String,
        #[source_code]
        src: String,
    },
}
fn parse_digits(input: &mut str, width: usize) -> Result<usize, miette::Report> {
    let (part, input) = input.split_at_mut(width);
    dbg!(&part, &input);
    part.parse::<usize>().into_diagnostic()
}
impl Interpreter {
    pub fn new() -> Self {
        Self
    }
    pub fn parse_datetime(input: &mut str, expected_format: String) -> Result<Datetime, Error> {
        let lexer = DateTimeLexer::new(&expected_format);
        let mut input = input;
        let mut datetime = Datetime::default();
        for token in lexer {
            let token = token?;
            match token {
                Token::FullYear => datetime.ano = parse_digits(&mut input, 4)?,
                Token::HalfYear => {
                    let y = parse_digits(&mut input, 2)?;
                    datetime.ano = if y < 25 { y + 2000 } else { y + 1900 };
                }
                Token::FullMonth => datetime.mes = parse_digits(&mut input, 2)?,
                Token::Day => datetime.dia = parse_digits(&mut input, 2)?,
                Token::TwelveHourDay | Token::TwentyFourHourDay => {
                    datetime.hora = parse_digits(&mut input, 2)?
                }
                Token::Minute => datetime.minuto = parse_digits(&mut input, 2)?,
                Token::Second => datetime.segundo = parse_digits(&mut input, 2)?,
                token => {
                    todo!("{token:?} not yet implemented")
                }
            }
        }
        Ok(datetime)
    }
}
