use error::{Error, ErrorKind};
pub use token::{Symbol, Token, TokenKind};
use super::error::{Location, Span};
use unicode_segmentation::UnicodeSegmentation;

pub mod error;
#[cfg(test)]
mod tests;
pub mod token;

fn is_whitespace<'a>(string: &'a str) -> bool {
    match string {
        "\n" | "\r" | "\r\n" => return true,
        _ => {},
    }
    match string.chars().next() {
        Some(ch) if ch.is_whitespace() => true,
        _ => false,
    }
}

fn is_digit<'a>(string: &'a str) -> bool {
    match string.chars().next() {
        Some(ch) if ch.is_digit(10) => true,
        _ => false,
    }
}

fn to_digit<'a>(string: &'a str) -> Option<u32> {
    string.chars().next().and_then(|ch| ch.to_digit(10))
}

fn is_alphabetic<'a>(string: &'a str) -> bool {
    match string.chars().next() {
        Some(ch) if ch.is_alphabetic() => true,
        _ => false,
    }
}

fn is_alphanumeric<'a>(string: &'a str) -> bool {
    match string.chars().next() {
        Some(ch) if ch.is_alphanumeric() => true,
        _ => false,
    }
}

fn scan<'a>(string: &'a str) -> Result<Vec<Symbol>, Error> {
    string
        .graphemes(true)
        .scan(Location::new(), |location, grapheme| {
            let current_location = *location;
            (*location).column += grapheme.chars().count();
            match grapheme {
                "\n" | "\r" | "\r\n" => {
                    (*location).line += 1;
                    (*location).column = 0;
                },
                _ => {},
            }
            Some(Symbol {
                grapheme,
                span: Span {
                    from: current_location,
                    to: *location,
                }
            })
        })
        .map(|symbol| Ok(symbol))
        .collect::<Result<Vec<Symbol>, Error>>()
}

pub(super) fn lex<'a>(string: &'a str) -> Result<Vec<Token>, Error> {
    let symbols = scan(string)?;

    let mut tokens = Vec::new();
    let mut i = 0;

    'tokens: loop {
        'gap: loop {
            match symbols.get(i).as_ref() {
                Some(Symbol { grapheme: c, .. }) if is_whitespace(c) => {
                    i += 1;
                }
                Some(Symbol { grapheme: "(", span: os }) => match symbols.get(i + 1) {
                    Some(Symbol { grapheme: "*", .. }) => {
                        match symbols.get(i + 2) {
                            Some(Symbol { grapheme: ")", span: oe }) => {
                                return Err(Error {
                                    kind: ErrorKind::InvalidSymbol("(*)".to_owned()),
                                    span: Span::combine(os, oe),
                                })
                            }
                            _ => {}
                        }
                        i += 2;
                        let mut nest_level = 1;
                        // comment
                        while nest_level != 0 {
                            match symbols.get(i) {
                                Some(Symbol { grapheme: "(", span: os }) => match symbols.get(i + 1) {
                                    Some(Symbol { grapheme: "*", .. }) => {
                                        match symbols.get(i + 2) {
                                            Some(Symbol { grapheme: ")", span: oe }) => {
                                                return Err(Error {
                                                    kind: ErrorKind::InvalidSymbol(
                                                        "(*)".to_owned(),
                                                    ),
                                                    span: Span::combine(os, oe),
                                                })
                                            }
                                            _ => {}
                                        }
                                        i += 2;
                                        nest_level += 1;
                                    }
                                    _ => {
                                        i += 1;
                                    }
                                },
                                Some(Symbol { grapheme: "*", .. }) => match symbols.get(i + 1) {
                                    Some(Symbol { grapheme: ")", .. }) => {
                                        i += 2;
                                        nest_level -= 1;
                                    }
                                    _ => {
                                        i += 1;
                                    }
                                },
                                Some(_) => {
                                    i += 1;
                                }
                                None => {
                                    return Err(Error {
                                        kind: ErrorKind::UnterminatedComment,
                                        span: symbols.get(i - 1).unwrap().span,
                                    })
                                }
                            };
                        }
                    }
                    _ => break 'gap,
                },
                _ => break 'gap,
            };
        }
        match symbols.get(i) {
            Some(Symbol { grapheme: ",", span }) => {
                tokens.push(Token::new(TokenKind::Concatenation, *span));
                i += 1;
            }
            Some(Symbol { grapheme: "=", span }) => {
                tokens.push(Token::new(TokenKind::Definition, *span));
                i += 1;
            }
            Some(Symbol { grapheme: "|", span }) | Some(Symbol { grapheme: "!", span }) => {
                tokens.push(Token::new(TokenKind::DefinitionSeparator, *span));
                i += 1;
            }
            Some(Symbol { grapheme: ")", span }) => {
                tokens.push(Token::new(TokenKind::EndGroup, *span));
                i += 1;
            }
            Some(Symbol { grapheme: "]", span }) => {
                tokens.push(Token::new(TokenKind::EndOption, *span));
                i += 1;
            }
            Some(Symbol { grapheme: "/", span: start }) => match symbols.get(i + 1) {
                Some(Symbol { grapheme: ")", span: end }) => {
                    tokens.push(Token::new(TokenKind::EndOption, Span::combine(start, end)));
                    i += 2;
                }
                _ => {
                    tokens.push(Token::new(TokenKind::DefinitionSeparator, *start));
                    i += 1;
                }
            },
            Some(Symbol { grapheme: "}", span }) => {
                tokens.push(Token::new(TokenKind::EndRepeat, *span));
                i += 1;
            }
            Some(Symbol { grapheme: ":", span: start }) => match symbols.get(i + 1) {
                Some(Symbol { grapheme: ")", span: end }) => {
                    tokens.push(Token::new(TokenKind::EndRepeat, Span::combine(start, end)));
                    i += 2;
                }
                _ => {
                    return Err(Error {
                        kind: ErrorKind::InvalidSymbol(':'.to_string()),
                        span: *start,
                    });
                }
            },
            Some(Symbol { grapheme: "-", span }) => {
                tokens.push(Token::new(TokenKind::Exception, *span));
                i += 1;
            }
            Some(Symbol { grapheme: "*", span }) => {
                tokens.push(Token::new(TokenKind::Repetition, *span));
                i += 1;
            }
            Some(Symbol { grapheme: "(", span: start }) => match symbols.get(i + 1) {
                Some(Symbol { grapheme: "/", span: middle }) => match symbols.get(i + 2) {
                    Some(Symbol { grapheme: ")", span: end }) => {
                        return Err(Error {
                            kind: ErrorKind::InvalidSymbol("(/)".to_owned()),
                            span: Span::combine(start, end),
                        });
                    }
                    _ => {
                        tokens.push(Token::new(TokenKind::StartOption, Span::combine(start, middle)));
                        i += 2;
                    }
                },
                Some(Symbol { grapheme: ":", span: middle }) => match symbols.get(i + 2) {
                    Some(Symbol { grapheme: ")", span: end }) => {
                        return Err(Error {
                            kind: ErrorKind::InvalidSymbol("(:)".to_owned()),
                            span: Span::combine(start, end),
                        });
                    }
                    _ => {
                        tokens.push(Token::new(TokenKind::StartRepeat, Span::combine(start, middle)));
                        i += 2;
                    }
                },
                _ => {
                    tokens.push(Token::new(TokenKind::StartGroup, *start));
                    i += 1;
                }
            },
            Some(Symbol { grapheme: "[", span }) => {
                tokens.push(Token::new(TokenKind::StartOption, *span));
                i += 1;
            }
            Some(Symbol { grapheme: "{", span }) => {
                tokens.push(Token::new(TokenKind::StartRepeat, *span));
                i += 1;
            }
            Some(Symbol { grapheme: ";", span }) | Some(Symbol { grapheme: ".", span }) => {
                tokens.push(Token::new(TokenKind::Terminator, *span));
                i += 1;
            }
            Some(Symbol { grapheme: quote, span: os }) if *quote == "\'" || *quote == "\"" => {
                let mut string = String::new();
                let mut len = 0;
                i += 1;
                'terminal: loop {
                    match symbols.get(i) {
                        Some(Symbol { grapheme: c, span: oe }) if c == quote => {
                            if len == 0 {
                                return Err(Error {
                                    kind: ErrorKind::EmptyTerminal,
                                    span: Span::combine(os, oe),
                                });
                            } else {
                                tokens.push(Token::new(TokenKind::Terminal(string), Span::combine(os, oe)));
                                i += 1;
                                break 'terminal;
                            }
                        }
                        Some(Symbol { grapheme: c, .. }) => {
                            string.push_str(c);
                            i += 1;
                            len += 1;
                        }
                        None => {
                            return Err(Error {
                                kind: ErrorKind::UnterminatedTerminal,
                                span: symbols.get(i - 1).unwrap().span,
                            })
                        }
                    }
                }
            }
            Some(Symbol { grapheme: "?", span: os }) => {
                let mut string = String::new();
                i += 1;
                'special: loop {
                    match symbols.get(i) {
                        Some(Symbol { grapheme: "?", span: oe }) => {
                            tokens.push(Token::new(TokenKind::Special(string), Span::combine(os, oe)));
                            i += 1;
                            break 'special;
                        }
                        Some(Symbol { grapheme: c, .. }) => {
                            string.push_str(c);
                            i += 1;
                        }
                        None => {
                            return Err(Error {
                                kind: ErrorKind::UnterminatedSpecial,
                                span: symbols.get(i - 1).unwrap().span,
                            })
                        }
                    }
                }
            }
            Some(Symbol { grapheme: c, span: os }) if is_digit(c) => {
                let mut oe = *os;
                let mut integer = to_digit(c).unwrap() as usize;
                i += 1;
                'integer: loop {
                    match symbols.get(i) {
                        Some(Symbol { grapheme: c, span: o }) if is_digit(c) => {
                            integer = integer * 10 + to_digit(c).unwrap() as usize;
                            oe = *o;
                            i += 1;
                        }
                        Some(Symbol { grapheme: c, .. }) if is_whitespace(c) => {
                            i += 1;
                        }
                        _ => {
                            tokens.push(Token::new(TokenKind::Integer(integer), Span::combine(os, &oe)));
                            break 'integer;
                        }
                    }
                }
            }
            Some(Symbol { grapheme: c, span: os }) if is_alphabetic(c) => {
                let mut oe = *os;
                let mut string = c.to_string();
                i += 1;
                'nonterminal: loop {
                    match symbols.get(i) {
                        Some(Symbol { grapheme: c, span: o }) if is_alphanumeric(c) => {
                            string.push_str(c);
                            oe = *o;
                            i += 1;
                        }
                        Some(Symbol { grapheme: c, .. }) if is_whitespace(c) => {
                            i += 1;
                        }
                        _ => {
                            tokens.push(Token::new(TokenKind::Nonterminal(string), Span::combine(os, &oe)));
                            break 'nonterminal;
                        }
                    }
                }
            }
            Some(Symbol { grapheme: c, span }) => {
                return Err(Error {
                    kind: ErrorKind::InvalidSymbol((*c).to_string()),
                    span: *span,
                });
            }
            None => break 'tokens,
        }
    }

    return Ok(tokens);
}
