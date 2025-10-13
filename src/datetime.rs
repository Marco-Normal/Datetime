use log::{info, warn};
use miette::{Diagnostic, Error};
use thiserror::Error;

use crate::{interpreter::Interpreter, lexer::Token};
/// A datetime Structure that contains only the most important parts
#[derive(Debug, PartialEq, Eq)]
pub struct Datetime {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
    pub(crate) twelve_hour_format: bool,
}

/// A datetime builder that contains only the most important parts
pub struct DatetimeBuilder {
    year: usize,
    month: usize,
    day: usize,
    pub(crate) hour: usize,
    minute: usize,
    second: usize,
    pub(crate) twelve_hour_format: bool,
}
#[derive(Debug, Error, Diagnostic)]
pub(crate) enum DatetimeError {
    #[error(
        "Invalid value. Expected `{}` for `{}` but got `{}`",
        expected,
        field,
        got
    )]
    InvalidValue {
        expected: String,
        field: Token,
        got: String,
        #[source_code]
        src: Option<String>,
    },
}

impl Default for Datetime {
    fn default() -> Self {
        Self {
            year: 1900,
            month: 1,
            day: 1,
            hour: 24,
            minute: 00,
            second: 00,
            twelve_hour_format: false,
        }
    }
}

impl Default for DatetimeBuilder {
    fn default() -> Self {
        Self {
            year: 1900,
            month: 1,
            day: 1,
            hour: 24,
            minute: 00,
            second: 00,
            twelve_hour_format: false,
        }
    }
}

impl DatetimeBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn year(self, year: usize) -> Self {
        Self { year, ..self }
    }
    pub fn month(self, month: usize) -> Self {
        if month > 12 {
            panic!(
                "{}",
                DatetimeError::InvalidValue {
                    expected: "1-12".to_string(),
                    field: Token::FullMonth,
                    got: month.to_string(),
                    src: None,
                }
            );
        }
        Self { month, ..self }
    }
    pub fn day(self, day: usize) -> Self {
        if day > 30 {
            panic!(
                "{}",
                DatetimeError::InvalidValue {
                    expected: "1-30".to_string(),
                    field: Token::Day,
                    got: day.to_string(),
                    src: None,
                }
            );
        }
        Self { day, ..self }
    }

    pub fn hour(self, hour: usize) -> Self {
        if hour > 24 {
            panic!(
                "{}",
                DatetimeError::InvalidValue {
                    expected: "0-24".to_string(),
                    field: Token::Hour,
                    got: hour.to_string(),
                    src: None,
                }
            );
        }
        Self { hour, ..self }
    }

    pub fn minute(self, minute: usize) -> Self {
        if minute > 60 {
            panic!(
                "{}",
                DatetimeError::InvalidValue {
                    expected: "0-60".to_string(),
                    field: Token::Day,
                    got: minute.to_string(),
                    src: None,
                }
            );
        }
        Self { minute, ..self }
    }

    pub fn second(self, second: usize) -> Self {
        if second > 60 {
            panic!(
                "{}",
                DatetimeError::InvalidValue {
                    expected: "0-60".to_string(),
                    field: Token::Second,
                    got: second.to_string(),
                    src: None,
                }
            );
        }
        Self { second, ..self }
    }
    pub fn change_format(self, twelve_hour_format: bool) -> Self {
        Self {
            twelve_hour_format,
            ..self
        }
    }
    pub fn build(self) -> Datetime {
        Datetime {
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
            twelve_hour_format: self.twelve_hour_format,
        }
    }
}

impl Datetime {
    pub fn from_str(date: &str, date_format: &str) -> Result<Self, Error> {
        Interpreter::parse_datetime(date, date_format)
    }
    pub fn try_guess(date: &str) -> Option<Self> {
        const COMMON_FORMATS: &[&str] = &[
            "%Y/%m/%d",
            "%Y-%m-%d",
            "%Y/%d/%m",
            "%Y/%d/%m",
            "%y/%m/%d",
            "%y-%m-%d",
            "%y/%d/%m",
            "%y/%d/%m",
            "%H:%M:%S",
            "%Hh:%Mm:%Ss",
            "%H %p:%M:%S",
            "%H %p:%M:%S",
            "%H:%M",
            "%Hh:%Mm",
            "%H:%M %p",
        ];
        for format in COMMON_FORMATS {
            info!("Trying to parse `{date}` as format `{format}`");
            match Interpreter::parse_datetime(date, format) {
                Ok(date) => return Some(date),
                Err(e) => warn!("Format `{format}` did not match `{date}`. Reason: {e}"),
            }
        }
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use miette::Error;

    type TestResult = Result<(), Error>;

    #[test]
    fn test_from_str() -> TestResult {
        let date = Datetime::from_str("2023-10-15", "%Y-%m-%d")?;
        assert_eq!(date.year, 2023);
        assert_eq!(date.month, 10);
        assert_eq!(date.day, 15);

        let date = Datetime::from_str("15/10/2023", "%d/%m/%Y")?;
        assert_eq!(date.year, 2023);
        assert_eq!(date.month, 10);
        assert_eq!(date.day, 15);

        Ok(())
    }

    #[test]
    fn test_default() {
        let date = Datetime::default();
        assert_eq!(date.year, 1900);
        assert_eq!(date.month, 1);
        assert_eq!(date.day, 1);
        assert_eq!(date.hour, 24);
        assert_eq!(date.minute, 0);
        assert_eq!(date.second, 0);
    }

    #[test]
    #[should_panic]
    fn test_invalid_formats() {
        let result = Datetime::from_str("2023-13-32", "%Y-%m-%d");
        assert!(result.is_err()); // Invalid month and day

        let _ = Datetime::from_str("25:70:99", "%H:%M:%S");
    }

    // Add this when you implement the try_guess feature
    #[test]
    fn test_try_guess() {
        let date = "2023-10-15";
        let result = Datetime::try_guess(date);
        assert!(result.is_some());
        if let Some(dt) = result {
            assert_eq!(dt.year, 2023);
            assert_eq!(dt.month, 10);
            assert_eq!(dt.day, 15);
        }

        let date = "15/10/2023";
        let result = Datetime::try_guess(date);
        assert!(result.is_some());

        let date = "not a date";
        let result = Datetime::try_guess(date);
        assert!(result.is_none());
    }
}
