use core::fmt;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(PartialEq, Debug)]
pub(crate) enum Token {
    FullYear,
    HalfYear,
    FullMonth,
    WrittenMonth,
    Day,
    TwentyFourHourDay,
    TwelveHourDay,
    Hour,
    Minute,
    Second,
    Literal { pattern: String },
    AmOrPm,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FullYear | Self::HalfYear => write!(f, "Year"),
            Self::FullMonth | Self::WrittenMonth => write!(f, "Month"),
            Self::Day => write!(f, "Day"),
            Self::TwentyFourHourDay | Self::TwelveHourDay | Self::Hour => write!(f, "Hour"),
            Self::Minute => write!(f, "Minute"),
            Self::Second => write!(f, "Second"),
            Self::Literal { pattern: _ } => write!(f, "Literal"),
            Self::AmOrPm => write!(f, "Am or Pm"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct DateTimeLexer<'a> {
    input: &'a str,
    rest: &'a str,
    byte: usize,
}

impl<'a> DateTimeLexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            input: src,
            rest: src,
            byte: 0,
        }
    }
}

impl Iterator for DateTimeLexer<'_> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.rest.chars();
        let next = chars.next()?;
        dbg!(self.rest);
        self.byte += next.len_utf8();
        enum Started {
            Percent,
            Other(char),
        }
        let started = match next {
            '%' => Started::Percent,
            c => Started::Other(c),
        };
        match started {
            Started::Percent => {
                self.rest = &self.rest[1..];
                if self.rest.is_empty() {
                    return Some(Err(LexerError::UnexpectedEOF));
                }
                assert!(!self.rest.is_empty());
                let ident = chars.next().expect("Checked above");
                self.rest = &self.rest[1..];
                match ident {
                    'Y' => Some(Ok(Token::FullYear)),
                    'y' => Some(Ok(Token::HalfYear)),
                    'm' => Some(Ok(Token::FullMonth)),
                    'B' => Some(Ok(Token::WrittenMonth)),
                    'd' => Some(Ok(Token::Day)),
                    'H' => Some(Ok(Token::TwentyFourHourDay)),
                    'I' => Some(Ok(Token::TwelveHourDay)),
                    'M' => Some(Ok(Token::Minute)),
                    'S' => Some(Ok(Token::Second)),
                    'p' => Some(Ok(Token::AmOrPm)),
                    c if c.is_ascii_whitespace() => Some(Err(LexerError::InvalidWhitespace {
                        at: (
                            self.byte - next.len_utf8(),
                            next.len_utf8() + ident.len_utf8(),
                        )
                            .into(),
                        src: self.input.to_string(),
                    })),
                    c => Some(Err(LexerError::InvalidFormat {
                        src: self.input.to_string(),
                        at: (self.byte - next.len_utf8(), next.len_utf8() + c.len_utf8()).into(),
                    })),
                }
            }
            Started::Other(c) => {
                let mut pattern = String::from(c);
                for next_char in chars {
                    if next_char == '%' {
                        break;
                    }
                    pattern.push(next_char);
                }
                self.rest = &self.rest[pattern.len()..];
                self.byte += pattern.len();
                Some(Ok(Token::Literal { pattern }))
            }
        }
    }
}

#[derive(Debug, Diagnostic, Error)]
pub enum LexerError {
    #[error("Invalid format of date given")]
    InvalidFormat {
        #[source_code]
        src: String,
        #[label("This input character")]
        at: SourceSpan,
    },
    #[error("Invalid Whitespace")]
    InvalidWhitespace {
        #[label("This format here")]
        at: SourceSpan,
        #[source_code]
        src: String,
    },
    #[error("Unexpected EOF")]
    UnexpectedEOF,
}

#[cfg(test)]
mod tests {

    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn basic_datetime() -> TestResult {
        let input = "%Y";
        let mut parser = DateTimeLexer::new(input);
        assert_eq!(
            parser.next().ok_or(LexerError::UnexpectedEOF)??,
            Token::FullYear
        );
        assert!(parser.next().is_none()); // Ensure end of input
        Ok(())
    }

    #[test]
    fn full_datetime() -> TestResult {
        let input = "%Y%m%d";
        let mut parser = DateTimeLexer::new(input);
        assert_eq!(
            parser.next().ok_or(LexerError::UnexpectedEOF)??,
            Token::FullYear
        );
        assert_eq!(
            parser.next().ok_or(LexerError::UnexpectedEOF)??,
            Token::FullMonth
        );
        assert_eq!(parser.next().ok_or(LexerError::UnexpectedEOF)??, Token::Day);
        assert!(parser.next().is_none());
        Ok(())
    }
    #[test]
    fn lexer_tokenization() -> TestResult {
        let test_cases = vec![
            ("%Y", vec![Token::FullYear]),
            ("%m", vec![Token::FullMonth]),
            ("%d", vec![Token::Day]),
            (
                "%Y%m%d",
                vec![Token::FullYear, Token::FullMonth, Token::Day],
            ),
            (
                "hello %Y world",
                vec![
                    Token::Literal {
                        pattern: String::from("hello "),
                    },
                    Token::FullYear,
                    Token::Literal {
                        pattern: String::from(" world"),
                    },
                ],
            ),
            // Add edge cases: empty string, invalid format, etc.
        ];

        for (input, expected_tokens) in test_cases {
            let parser = DateTimeLexer::new(input);
            let mut actual_tokens = Vec::new();
            for token in parser {
                actual_tokens.push(token?);
            }
            dbg!(&actual_tokens);
            assert_eq!(actual_tokens, expected_tokens, "Failed on input: {}", input);
        }
        Ok(())
    }
    #[test]
    fn test_error_conditions() -> TestResult {
        let input = "%Z"; // Invalid format specifier
        let mut lexer = DateTimeLexer::new(input);
        let result = lexer.next().ok_or(LexerError::UnexpectedEOF)?;
        // Should return an error for invalid format
        assert!(result.is_err());

        // Test for invalid whitespace
        let input = "% ";
        let mut lexer = DateTimeLexer::new(input);
        let result = lexer.next().ok_or(LexerError::UnexpectedEOF)?;
        assert!(matches!(result, Err(LexerError::InvalidWhitespace { .. })));

        Ok(())
    }

    #[test]
    fn test_complex_patterns() -> TestResult {
        let input = "Date: %Y-%m-%d Time: %H:%M:%S";
        let mut lexer = DateTimeLexer::new(input);
        let mut tokens = Vec::new();

        for token in lexer {
            tokens.push(token?);
        }

        // Should have pattern literals intermixed with format tokens
        dbg!(&tokens);
        assert_eq!(tokens.len(), 12);
        assert!(matches!(tokens[0], Token::Literal { .. }));
        assert!(matches!(tokens[3], Token::FullMonth));

        Ok(())
    }

    #[test]
    fn test_edge_cases() -> TestResult {
        // Empty input
        let input = "";
        let mut lexer = DateTimeLexer::new(input);
        assert!(lexer.next().is_none());

        // Single % at the end
        let input = "%";
        let mut lexer = DateTimeLexer::new(input);
        let result = lexer.next();
        assert!(result.is_some());
        assert!(result.unwrap().is_err());

        Ok(())
    }

    #[test]
    fn test_consecutive_literals_merged() -> TestResult {
        let input = "hello world";
        let mut lexer = DateTimeLexer::new(input);
        let token = lexer.next().ok_or(LexerError::UnexpectedEOF)??;

        // Should merge all literals into a single token
        assert_eq!(
            token,
            Token::Literal {
                pattern: "hello world".to_string()
            }
        );
        assert!(lexer.next().is_none());

        Ok(())
    }
}
