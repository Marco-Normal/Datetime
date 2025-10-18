use crate::datetime::{Datetime, DatetimeBuilder, DatetimeError};
use crate::lexer::{DateTimeLexer, Token};
use miette::{Diagnostic, Error, IntoDiagnostic};
use thiserror::Error;

#[derive(Default)]
pub(crate) struct Interpreter;
#[derive(Debug, Error, Diagnostic)]
pub(crate) enum InterpreterError {
    #[error("Unexpect sequence. Expected `{}`, got `{}`", expected, unexpected)]
    WrongSequence {
        expected: String,
        unexpected: String,
        #[source_code]
        src: String,
    },
    #[error(
        "Input sequence too short for token. Expected at least `{}` tokens but got `{}`",
        expected,
        unexpected
    )]
    InputTooShort {
        expected: usize,
        unexpected: usize,
        #[source_code]
        src: String,
    },
}
fn parse_digits(input: &str, width: usize) -> Result<(usize, &str), miette::Report> {
    if input.len() < width {
        return Err(InterpreterError::InputTooShort {
            expected: width,
            unexpected: input.len(),
            src: input.to_string(),
        }
        .into());
    }
    let (part, rest) = input.split_at(width);
    let number = part.parse::<usize>().into_diagnostic()?;
    Ok((number, rest))
}
impl Interpreter {
    pub(crate) fn parse_datetime(
        mut input: &str,
        expected_format: &str,
    ) -> Result<Datetime, Error> {
        let lexer = DateTimeLexer::new(expected_format);
        let original_input = input;
        let mut datetime = DatetimeBuilder::default();
        for token in lexer {
            let token = token?;
            match token {
                Token::FullYear => {
                    let year: usize;
                    (year, input) = parse_digits(input, 4)?;
                    datetime = datetime.year(year)
                }
                Token::HalfYear => {
                    let y: usize;
                    (y, input) = parse_digits(input, 2)?;
                    datetime = datetime.year(if y < 25 { y + 2000 } else { y + 1900 });
                }
                Token::FullMonth => {
                    let mes: usize;
                    (mes, input) = parse_digits(input, 2)?;
                    datetime = datetime.month(mes);
                }
                Token::Day => {
                    let day: usize;
                    (day, input) = parse_digits(input, 2)?;
                    datetime = datetime.day(day);
                }
                Token::TwelveHourDay | Token::TwentyFourHourDay => {
                    let hour: usize;
                    (hour, input) = parse_digits(input, 2)?;
                    datetime = datetime.hour(hour);
                }
                Token::AmOrPm => {
                    let hour = datetime.hour;
                    if input.starts_with("PM") {
                        input = &input[2..];
                        if hour < 12 {
                            datetime = datetime.hour(hour + 12);
                        }
                    } else if input.starts_with("AM") {
                        input = &input[2..];
                        if hour == 12 {
                            datetime = datetime.hour(0);
                        }
                    } else {
                        return Err(InterpreterError::WrongSequence {
                            expected: "AM or PM".to_string(),
                            unexpected: input.get(..2).unwrap_or(input).to_string(),
                            src: original_input.to_string(),
                        }
                        .into());
                    }
                }
                Token::Minute => {
                    let minute: usize;
                    (minute, input) = parse_digits(input, 2)?;
                    datetime = datetime.minute(minute)
                }
                Token::Second => {
                    let second: usize;
                    (second, input) = parse_digits(input, 2)?;
                    datetime = datetime.second(second)
                }
                Token::Literal { pattern } => {
                    if let Some(rest) = input.strip_prefix(&pattern) {
                        input = rest;
                    } else {
                        return Err(InterpreterError::WrongSequence {
                            unexpected: input.get(..pattern.len()).unwrap_or(input).to_string(),
                            expected: pattern,
                            src: original_input.to_string(),
                        }
                        .into());
                    }
                }
                token => {
                    todo!("{token:?} not yet implemented")
                }
            }
        }
        datetime.build()
    }
}

mod tests {
    use super::*;
    type TestResult = Result<(), miette::Error>;

    #[test]
    fn basic_str_to_datetime() -> TestResult {
        let mut input = String::from("04-02-2003");
        let result = Interpreter::parse_datetime(&mut input, "%d-%m-%Y")?;
        assert_eq!(
            result,
            Datetime {
                year: 2003,
                month: 2,
                day: 4,
                ..Default::default()
            }
        );
        Ok(())
    }
    #[test]
    fn expected_err() -> TestResult {
        let mut input = String::from("04-02?2003");
        let result = Interpreter::parse_datetime(&mut input, "%d-%m-%Y");
        assert!(result.is_err());
        Ok(())
    }
    #[test]
    fn test_all_token_types() -> TestResult {
        // Test year parsing
        let mut input = String::from("2023");
        let result = parse_digits(&mut input, 4)?;
        assert_eq!(result.0, 2023);

        // Test full datetime with all components
        let mut input = String::from("2023-05-15 14:30:25");
        let result = Interpreter::parse_datetime(&mut input, "%Y-%m-%d %H:%M:%S")?;
        assert_eq!(
            result,
            Datetime {
                year: 2023,
                month: 5,
                day: 15,
                hour: 14,
                minute: 30,
                second: 25,
            }
        );

        // Test AM/PM format
        let mut input = String::from("03:45:20 PM");
        let result = Interpreter::parse_datetime(&mut input, "%I:%M:%S %p")?;
        assert_eq!(
            result,
            Datetime {
                hour: 15, // 3 PM = 15 in 24-hour format
                minute: 45,
                second: 20,
                ..Default::default()
            }
        );

        Ok(())
    }

    #[test]
    fn test_error_handling() -> TestResult {
        // Test mismatched literals
        let mut input = String::from("2023/05/15");
        let result = Interpreter::parse_datetime(&mut input, "%Y-%m-%d");
        assert!(result.is_err());

        // Test insufficient digits
        let mut input = String::from("23-5-15");
        let result = Interpreter::parse_datetime(&mut input, "%Y-%m-%d");
        assert!(result.is_err());

        // Test invalid numbers
        let mut input = String::from("20a3-05-15");
        let result = Interpreter::parse_datetime(&mut input, "%Y-%m-%d");
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_edge_dates() -> TestResult {
        // Test minimum date
        let mut input = String::from("0001-01-01");
        let result = Interpreter::parse_datetime(&mut input, "%Y-%m-%d")?;
        assert_eq!(result.year, 1);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 1);

        // Test leap year date
        let mut input = String::from("2020-02-29");
        let result = Interpreter::parse_datetime(&mut input, "%Y-%m-%d")?;
        assert_eq!(result.year, 2020);
        assert_eq!(result.month, 2);
        assert_eq!(result.day, 29);

        Ok(())
    }
}
