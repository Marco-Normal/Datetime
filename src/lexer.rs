use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(PartialEq, Debug)]
pub enum Token {
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

#[derive(Debug)]
pub struct DateTimeLexer<'a> {
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
    type Item = Result<Token, DateTimeError>;

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
                assert!(!self.rest.is_empty());
                let ident = chars.next().expect("Checked above");
                self.rest = &self.rest[2..];
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
                    c if c.is_ascii_whitespace() => Some(Err(DateTimeError::InvalidWhitespace {
                        at: (
                            self.byte - next.len_utf8(),
                            next.len_utf8() + ident.len_utf8(),
                        )
                            .into(),
                        src: self.input.to_string(),
                    })),
                    c => Some(Err(DateTimeError::InvalidFormat {
                        src: self.input.to_string(),
                        at: (self.byte - next.len_utf8(), next.len_utf8() + c.len_utf8()).into(),
                    })),
                }
            }
            Started::Other(c) => {
                let mut len = 0;
                let mut pattern = String::from(c);
                loop {
                    if let Some(character) = chars.next() {
                        if c == character {
                            pattern.push(character);
                            len += 1;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                self.rest = &self.rest[1 + len..];
                self.byte += len + 1;
                Some(Ok(Token::Literal { pattern }))
            }
        }
    }
}

#[derive(Debug, Diagnostic, Error)]
pub enum DateTimeError {
    #[error("Invalid format of date given")]
    InvalidFormat {
        #[source_code]
        src: String,
        #[label("This input character")]
        at: SourceSpan,
    },
    #[error("Missing `%` from pattern definition")]
    MissingPercent,
    #[error("Invalid Whitespace")]
    InvalidWhitespace {
        #[label("This format here")]
        at: SourceSpan,
        #[source_code]
        src: String,
    },
}

#[cfg(test)]
mod tests {
    use miette::miette;

    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn basic_datetime() -> TestResult {
        let input = "%Y";
        let mut parser = DateTimeLexer::new(input);
        assert_eq!(
            parser.next().ok_or(DateTimeError::MissingPercent)??,
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
            parser.next().ok_or(DateTimeError::MissingPercent)??,
            Token::FullYear
        );
        assert_eq!(
            parser.next().ok_or(DateTimeError::MissingPercent)??,
            Token::FullMonth
        );
        assert_eq!(
            parser.next().ok_or(DateTimeError::MissingPercent)??,
            Token::Day
        );
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
                        pattern: String::from('h'),
                    },
                    Token::Literal {
                        pattern: String::from('e'),
                    },
                    Token::Literal {
                        pattern: String::from("ll"),
                    },
                    Token::Literal {
                        pattern: String::from('o'),
                    },
                    Token::Literal {
                        pattern: String::from(' '),
                    },
                    Token::FullYear,
                    Token::Literal {
                        pattern: String::from(' '),
                    },
                    Token::Literal {
                        pattern: String::from('w'),
                    },
                    Token::Literal {
                        pattern: String::from('o'),
                    },
                    Token::Literal {
                        pattern: String::from('r'),
                    },
                    Token::Literal {
                        pattern: String::from('l'),
                    },
                    Token::Literal {
                        pattern: String::from('d'),
                    },
                ],
            ),
            // Add edge cases: empty string, invalid format, etc.
        ];

        for (input, expected_tokens) in test_cases {
            let mut parser = DateTimeLexer::new(input);
            let mut actual_tokens = Vec::new();
            while let Some(token) = parser.next() {
                actual_tokens.push(token?);
            }
            dbg!(&input);
            assert_eq!(actual_tokens, expected_tokens, "Failed on input: {}", input);
        }
        Ok(())
    }
}
