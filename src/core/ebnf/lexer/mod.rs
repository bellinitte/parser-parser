pub mod error;
#[cfg(test)]
mod tests;
pub mod token;

use super::scanner::Symbol;
use error::Error;
pub use token::{Token, TokenKind};

pub(super) fn lex<'a>(symbols: &[Symbol]) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut i = 0;

    'tokens: loop {
        'gap: loop {
            println!("{:?}", symbols.get(i));
            match symbols.get(i) {
                Some((_, c)) if c.is_whitespace() => {
                    i += 1;
                }
                Some((_, '(')) => match symbols.get(i + 1) {
                    Some((_, '*')) => {
                        match symbols.get(i + 2) {
                            Some((_, ')')) => return Err(Error::AmbiguousSymbol),
                            _ => {}
                        }
                        i += 2;
                        let mut nest_level = 1;
                        // comment
                        while nest_level != 0 {
                            match symbols.get(i) {
                                Some((_, '(')) => match symbols.get(i + 1) {
                                    Some((_, '*')) => {
                                        match symbols.get(i + 2) {
                                            Some((_, ')')) => return Err(Error::AmbiguousSymbol),
                                            _ => {}
                                        }
                                        i += 2;
                                        nest_level += 1;
                                    }
                                    _ => {}
                                },
                                Some((_, '*')) => match symbols.get(i + 1) {
                                    Some((_, ')')) => {
                                        i += 2;
                                        nest_level -= 1;
                                    }
                                    _ => {}
                                },
                                Some(_) => {
                                    i += 1;
                                }
                                None => return Err(Error::UnterminatedComment),
                            };
                        }
                    }
                    _ => break 'gap,
                },
                _ => break 'gap,
            };
        }
        println!("{:?}", symbols.get(i));
        match symbols.get(i) {
            Some((o, ',')) => {
                tokens.push(Token::new(TokenKind::Concatenation, *o..*o + 1));
                i += 1;
            }
            Some((o, '=')) => {
                tokens.push(Token::new(TokenKind::Definition, *o..*o + 1));
                i += 1;
            }
            Some((o, '|')) | Some((o, '!')) => {
                tokens.push(Token::new(TokenKind::DefinitionSeparator, *o..*o + 1));
                i += 1;
            }
            Some((o, ')')) => {
                tokens.push(Token::new(TokenKind::EndGroup, *o..*o + 1));
                i += 1;
            }
            Some((o, ']')) => {
                tokens.push(Token::new(TokenKind::EndOption, *o..*o + 1));
                i += 1;
            }
            Some((o, '/')) => match symbols.get(i + 1) {
                Some((_, ')')) => {
                    tokens.push(Token::new(TokenKind::EndOption, *o..*o + 2));
                    i += 2;
                }
                Some(_) => {
                    tokens.push(Token::new(TokenKind::DefinitionSeparator, *o..*o + 1));
                    i += 1;
                }
                _ => {}
            },
            Some((o, '}')) => {
                tokens.push(Token::new(TokenKind::EndRepeat, *o..*o + 1));
                i += 1;
            }
            Some((o, ':')) => match symbols.get(i + 1) {
                Some((_, ')')) => {
                    tokens.push(Token::new(TokenKind::EndRepeat, *o..*o + 2));
                    i += 2;
                }
                _ => {
                    return Err(Error::InvalidSymbol(':'));
                }
            },
            Some((o, '-')) => {
                tokens.push(Token::new(TokenKind::Exception, *o..*o + 1));
                i += 1;
            }
            Some((o, '*')) => {
                tokens.push(Token::new(TokenKind::Repetition, *o..*o + 1));
                i += 1;
            }
            Some((o, '(')) => match symbols.get(i + 1) {
                Some((_, '/')) => {
                    tokens.push(Token::new(TokenKind::StartOption, *o..*o + 2));
                    i += 2;
                }
                Some((_, ':')) => {
                    tokens.push(Token::new(TokenKind::StartRepeat, *o..*o + 2));
                    i += 2;
                }
                _ => {
                    tokens.push(Token::new(TokenKind::StartGroup, *o..*o + 2));
                    i += 1;
                }
            },
            Some((o, '[')) => {
                tokens.push(Token::new(TokenKind::StartOption, *o..*o + 1));
                i += 1;
            }
            Some((o, '{')) => {
                tokens.push(Token::new(TokenKind::StartRepeat, *o..*o + 1));
                i += 1;
            }
            Some((o, ';')) | Some((o, '.')) => {
                tokens.push(Token::new(TokenKind::Terminator, *o..*o + 1));
                i += 1;
            }
            Some((os, quote)) if *quote == '\'' || *quote == '"' => {
                let mut string = String::new();
                i += 1;
                'terminal: loop {
                    match symbols.get(i) {
                        Some((oe, c)) if c == quote => {
                            if *oe == *os + 1 {
                                return Err(Error::EmptyTerminal);
                            } else {
                                tokens.push(Token::new(TokenKind::Terminal(string), *os..*oe + 1));
                                i += 1;
                                break 'terminal;
                            }
                        }
                        Some((_, c)) => {
                            string.push(*c);
                            i += 1;
                        }
                        None => return Err(Error::UnterminatedTerminal),
                    }
                }
            }
            Some((os, '?')) => {
                let mut string = String::new();
                i += 1;
                'special: loop {
                    match symbols.get(i) {
                        Some((oe, '?')) => {
                            tokens.push(Token::new(TokenKind::Special(string), *os..*oe + 1));
                            i += 1;
                            break 'special;
                        }
                        Some((_, c)) => {
                            string.push(*c);
                            i += 1;
                        }
                        None => return Err(Error::UnterminatedSpecial),
                    }
                }
            }
            Some((os, c)) if c.is_digit(10) => {
                let mut oe = *os;
                let mut integer = c.to_digit(10).unwrap() as usize;
                i += 1;
                'integer: loop {
                    match symbols.get(i) {
                        Some((o, c)) if c.is_digit(10) => {
                            integer = integer * 10 + c.to_digit(10).unwrap() as usize;
                            oe = *o;
                            i += 1;
                        }
                        Some((_, c)) if c.is_whitespace() => {
                            i += 1;
                        }
                        _ => {
                            tokens.push(Token::new(TokenKind::Integer(integer), *os..oe + 1));
                            i += 1;
                            break 'integer;
                        }
                    }
                }
            }
            Some((os, c)) if c.is_alphabetic() => {
                let mut oe = *os;
                let mut string = c.to_string();
                i += 1;
                'nonterminal: loop {
                    match symbols.get(i) {
                        Some((o, c)) if c.is_alphanumeric() => {
                            string.push(*c);
                            oe = *o;
                            i += 1;
                        }
                        Some((_, c)) if c.is_whitespace() => {
                            i += 1;
                        }
                        _ => {
                            tokens.push(Token::new(TokenKind::Nonterminal(string), *os..oe + 1));
                            i += 1;
                            break 'nonterminal;
                        }
                    }
                }
            }
            Some((_, c)) => {
                return Err(Error::InvalidSymbol(*c));
            }
            None => break 'tokens,
        }
    }

    return Ok(tokens);
}
