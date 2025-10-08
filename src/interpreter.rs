use super::Datetime;
use crate::lexer::{DateTimeLexer, Token};
use miette::{Diagnostic, Error, IntoDiagnostic};
use thiserror::Error;

#[derive(Default)]
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
fn parse_digits(mut input: &mut str, width: usize) -> Result<(usize, &mut str), miette::Report> {
    let part: &mut str;
    (part, input) = input.split_at_mut(width);
    dbg!(&part, &input);
    let number = part.parse::<usize>().into_diagnostic()?;
    Ok((number, input))
}
impl Interpreter {
    pub fn parse_datetime(input: &mut str, expected_format: String) -> Result<Datetime, Error> {
        let lexer = DateTimeLexer::new(&expected_format);
        let mut input = input;
        let mut datetime = Datetime::default();
        for token in lexer {
            let token = token?;
            match token {
                Token::FullYear => (datetime.ano, input) = parse_digits(input, 4)?,
                Token::HalfYear => {
                    let y: usize;
                    (y, input) = parse_digits(input, 2)?;
                    datetime.ano = if y < 25 { y + 2000 } else { y + 1900 };
                }
                Token::FullMonth => (datetime.mes, input) = parse_digits(input, 2)?,
                Token::Day => (datetime.dia, input) = parse_digits(input, 2)?,
                Token::TwelveHourDay | Token::TwentyFourHourDay => {
                    (datetime.hora, input) = parse_digits(input, 2)?
                }
                Token::Minute => (datetime.minuto, input) = parse_digits(input, 2)?,
                Token::Second => (datetime.segundo, input) = parse_digits(input, 2)?,
                Token::Literal { pattern } => {
                    if input.strip_prefix(&pattern).is_some() {
                        input = &mut input[pattern.len()..]
                    } else {
                        return Err(InterpreterError::WrongSequence {
                            unexpected: input[..pattern.len()].to_string(),
                            expected: pattern,
                            src: input.to_string(),
                        }
                        .into());
                    }
                }
                token => {
                    todo!("{token:?} not yet implemented")
                }
            }
        }
        Ok(datetime)
    }
}

mod tests {
    type TestResult = Result<(), miette::Error>;
    use super::*;
    #[test]
    fn basic_str_to_datetime() -> TestResult {
        let mut input = String::from("04-02-2003");
        let result = Interpreter::parse_datetime(&mut input, String::from("%d-%m-%Y"))?;
        assert_eq!(
            result,
            Datetime {
                ano: 2003,
                mes: 2,
                dia: 4,
                ..Default::default()
            }
        );
        Ok(())
    }
    #[test]
    fn expected_err() -> TestResult {
        let mut input = String::from("04-02?2003");
        let result = Interpreter::parse_datetime(&mut input, String::from("%d-%m-%Y"));
        assert!(result.is_err());
        Ok(())
    }
}
