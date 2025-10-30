use core::fmt;

use log::{info, warn};
use miette::{Diagnostic, Error};
use thiserror::Error;

use crate::{interpreter::Interpreter, lexer::Token};
/// A datetime Structure that contains only the most important parts
/// Every Field is public to mimic how datetime in python works.
/// But, if you decide to build directly, there will be no guarantees
/// that the date will be valid. So, it's recommended that you use the
/// proper builder
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd)]
pub struct Datetime {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
    pub second: usize,
}

/// A datetime builder that contains only the most important parts.
/// When calling `.build()`, it performs all the necessary checks
/// to ensure that the date is correct. If, for some reason, some
/// field is with a wrong value, it'll throw an error.
/// # Examples
/// ```
/// use doc::DatetimeBuilder
/// let new_date = DatetimeBuilder::new()
///     .year(2024)
///     .month(2)
///     .day(4);
/// let date: Result<Datetime, _> = new_date.build();
/// assert!(date.is_ok());
/// ```
pub struct DatetimeBuilder {
    year: usize,
    month: usize,
    day: usize,
    pub(crate) hour: usize,
    minute: usize,
    second: usize,
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
            hour: 0,
            minute: 00,
            second: 00,
        }
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}/{:02}/{:02} {:02}:{:02}:{:02}",
            self.day, self.month, self.year, self.hour, self.minute, self.second
        )
    }
}

impl Default for DatetimeBuilder {
    fn default() -> Self {
        Self {
            year: 1900,
            month: 1,
            day: 1,
            hour: 0,
            minute: 00,
            second: 00,
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
        Self { month, ..self }
    }
    pub fn day(self, day: usize) -> Self {
        Self { day, ..self }
    }

    pub fn hour(self, hour: usize) -> Self {
        Self { hour, ..self }
    }

    pub fn minute(self, minute: usize) -> Self {
        Self { minute, ..self }
    }

    pub fn second(self, second: usize) -> Self {
        Self { second, ..self }
    }
    /// Returns an error if some field for the date is invalid, e.g.: month(14)
    pub fn build(self) -> Result<Datetime, Error> {
        let max_days = match days_in_month(self.year, self.month) {
            Some(days) => days,
            None => {
                return Err(DatetimeError::InvalidValue {
                    expected: "A month between 1-12".to_string(),
                    field: Token::FullMonth, // Or another appropriate token
                    got: self.month.to_string(),
                    src: None,
                }
                .into());
            }
        };
        if self.month > 12 {
            return Err(DatetimeError::InvalidValue {
                expected: "1-12".to_string(),
                field: Token::FullMonth,
                got: self.month.to_string(),
                src: None,
            }
            .into());
        }

        if self.day == 0 || self.day > max_days {
            return Err(DatetimeError::InvalidValue {
                expected: format!("A day between 1-{}", max_days),
                field: Token::Day,
                got: self.day.to_string(),
                src: None,
            }
            .into());
        }
        if self.hour > 23 {
            return Err(DatetimeError::InvalidValue {
                expected: "0-23".to_string(),
                field: Token::Hour,
                got: self.hour.to_string(),
                src: None,
            }
            .into());
        }

        if self.minute > 59 {
            return Err(DatetimeError::InvalidValue {
                expected: "0-60".to_string(),
                field: Token::Day,
                got: self.minute.to_string(),
                src: None,
            }
            .into());
        }
        if self.second > 59 {
            return Err(DatetimeError::InvalidValue {
                expected: "0-60".to_string(),
                field: Token::Second,
                got: self.second.to_string(),
                src: None,
            }
            .into());
        }
        Ok(Datetime {
            year: self.year,
            month: self.month,
            day: self.day,
            hour: self.hour,
            minute: self.minute,
            second: self.second,
        })
    }
}
fn is_leap_year(year: usize) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
fn days_in_month(year: usize, month: usize) -> Option<usize> {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31), // Months with 31 days
        4 | 6 | 9 | 11 => Some(30),              // Months with 30 days
        2 => Some(if is_leap_year(year) { 29 } else { 28 }), // February
        _ => None,                               // Invalid month
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
        assert_eq!(date.hour, 0);
        assert_eq!(date.minute, 0);
        assert_eq!(date.second, 0);
    }

    #[test]
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
