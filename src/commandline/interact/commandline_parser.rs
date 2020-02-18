use std::error;
use std::fmt;
use std::str::Chars;
use std::iter::Peekable;

pub fn to_args(input: &String) -> Result<Vec<String>, Error> {
    let mut chars = input.chars().peekable();
}

fn to_arg(chars: &mut Peekable<Chars>) -> Result<String, Error> {
    let mut res = String::new();

    let mut in_quote = false;
    loop {
        if let Some(next_char) = chars.peek() {
            let next_char = next_char.clone();
            match next_char {
                '"' => {
                    in_quote = !in_quote;
                    chars.next();
                },
                '\\' => {
                    chars.next();
                    if let Some(next_char) = chars.next() {
                        match next_char {
                            'n' => res.push('\n'),
                            't' => res.push('\t'),
                            '\\' => res.push('\\'),
                            '"' => res.push('"'),
                            _ => {
                                let illegal_escape_char = format!("\\{}", next_char);
                                return Err(Error::IllegalEscapeChar(illegal_escape_char));
                            }
                        }
                    } else {
                        return Err(Error::IllegalEscapeChar(String::from('\\')));
                    }
                }
                _ if next_char.is_whitespace() => {

                },
                _ => {
                    res.push(chars.next().unwrap().clone())
                }
            }
        } else {
            break;
        }
    }
    Ok(res)
}

#[derive(Debug)]
pub enum Error {
    IllegalEscapeChar(String)
}

impl error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        use Error::*;
        let message = match &self {
            IllegalEscapeChar(escape_char) => format!("Illegal escape char {}.", escape_char)
        };
        Ok(())
    }
}
