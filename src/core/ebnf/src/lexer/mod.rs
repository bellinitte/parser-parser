use super::span::{Location, Span, Spanned, Spanning};
use error::Error;
pub use token::Token;
use unicode_segmentation::UnicodeSegmentation;

pub mod error;
#[cfg(test)]
mod tests;
pub mod token;

fn is_whitespace(string: & str) -> bool {
    match string {
        "\n" | "\r" | "\r\n" => return true,
        _ => {}
    }
    matches!(string.chars().next(), Some(ch) if ch.is_whitespace())
}

fn is_digit(string: &str) -> bool {
    matches!(string.chars().next(), Some(ch) if ch.is_digit(10))
}

fn to_digit(string: &str) -> Option<u32> {
    string.chars().next().and_then(|ch| ch.to_digit(10))
}

fn is_alphabetic(string: &str) -> bool {
    matches!(string.chars().next(), Some(ch) if ch.is_alphabetic())
}

fn is_alphanumeric(string: &str) -> bool {
    matches!(string.chars().next(), Some(ch) if ch.is_alphanumeric())
}

fn scan<'a>(string: &'a str) -> Result<Vec<Spanned<&'a str>>, Spanned<Error>> {
    string
        .graphemes(true)
        .scan(Location::new(), |location, grapheme| {
            let current_location = *location;
            (*location).column += grapheme.chars().count();
            match grapheme {
                "\n" | "\r" | "\r\n" => {
                    (*location).line += 1;
                    (*location).column = 0;
                }
                _ => {}
            }
            Some(Spanned {
                node: grapheme,
                span: Span {
                    from: current_location,
                    to: *location,
                },
            })
        })
        .map(Ok)
        .collect::<Result<Vec<Spanned<&'a str>>, Spanned<Error>>>()
}

pub(super) fn lex(string: &str) -> Result<Vec<Spanned<Token>>, Spanned<Error>> {
    let symbols = scan(string)?;

    let mut tokens = Vec::new();
    let mut i = 0;

    'tokens: loop {
        'gap: loop {
            match symbols.get(i).as_ref() {
                Some(Spanned { node: c, .. }) if is_whitespace(c) => {
                    i += 1;
                }
                Some(Spanned {
                    node: "(",
                    span: os,
                }) => match symbols.get(i + 1) {
                    Some(Spanned { node: "*", .. }) => {
                        match symbols.get(i + 2) {
                            Some(Spanned {
                                node: ")",
                                span: oe,
                            }) => {
                                return Err(Error::InvalidSymbol("(*)".to_owned())
                                    .spanning(Span::combine(os, oe)))
                            }
                            _ => {}
                        }
                        i += 2;
                        let mut nest_level = 1;
                        // comment
                        while nest_level != 0 {
                            match symbols.get(i) {
                                Some(Spanned {
                                    node: "(",
                                    span: os,
                                }) => match symbols.get(i + 1) {
                                    Some(Spanned { node: "*", .. }) => {
                                        match symbols.get(i + 2) {
                                            Some(Spanned {
                                                node: ")",
                                                span: oe,
                                            }) => {
                                                return Err(Error::InvalidSymbol("(*)".to_owned())
                                                    .spanning(Span::combine(os, oe)))
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
                                Some(Spanned { node: "*", .. }) => match symbols.get(i + 1) {
                                    Some(Spanned { node: ")", .. }) => {
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
                                    return Err(Error::UnterminatedComment
                                        .spanning(symbols.get(i - 1).unwrap().span))
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
            Some(Spanned { node: ",", span }) => {
                tokens.push(Token::Concatenation.spanning(*span));
                i += 1;
            }
            Some(Spanned { node: "=", span }) => {
                tokens.push(Token::Definition.spanning(*span));
                i += 1;
            }
            Some(Spanned { node: "|", span }) | Some(Spanned { node: "!", span }) => {
                tokens.push(Token::DefinitionSeparator.spanning(*span));
                i += 1;
            }
            Some(Spanned { node: ")", span }) => {
                tokens.push(Token::EndGroup.spanning(*span));
                i += 1;
            }
            Some(Spanned { node: "]", span }) => {
                tokens.push(Token::EndOption.spanning(*span));
                i += 1;
            }
            Some(Spanned {
                node: "/",
                span: start,
            }) => match symbols.get(i + 1) {
                Some(Spanned {
                    node: ")",
                    span: end,
                }) => {
                    tokens.push(Token::EndOption.spanning(Span::combine(start, end)));
                    i += 2;
                }
                _ => {
                    tokens.push(Token::DefinitionSeparator.spanning(*start));
                    i += 1;
                }
            },
            Some(Spanned { node: "}", span }) => {
                tokens.push(Token::EndRepeat.spanning(*span));
                i += 1;
            }
            Some(Spanned {
                node: ":",
                span: start,
            }) => match symbols.get(i + 1) {
                Some(Spanned {
                    node: ")",
                    span: end,
                }) => {
                    tokens.push(Token::EndRepeat.spanning(Span::combine(start, end)));
                    i += 2;
                }
                _ => {
                    return Err(Error::InvalidSymbol(':'.to_string()).spanning(*start));
                }
            },
            Some(Spanned { node: "-", span }) => {
                tokens.push(Token::Exception.spanning(*span));
                i += 1;
            }
            Some(Spanned { node: "*", span }) => {
                tokens.push(Token::Repetition.spanning(*span));
                i += 1;
            }
            Some(Spanned {
                node: "(",
                span: start,
            }) => match symbols.get(i + 1) {
                Some(Spanned {
                    node: "/",
                    span: middle,
                }) => match symbols.get(i + 2) {
                    Some(Spanned {
                        node: ")",
                        span: end,
                    }) => {
                        return Err(Error::InvalidSymbol("(/)".to_owned())
                            .spanning(Span::combine(start, end)));
                    }
                    _ => {
                        tokens.push(Token::StartOption.spanning(Span::combine(start, middle)));
                        i += 2;
                    }
                },
                Some(Spanned {
                    node: ":",
                    span: middle,
                }) => match symbols.get(i + 2) {
                    Some(Spanned {
                        node: ")",
                        span: end,
                    }) => {
                        return Err(Error::InvalidSymbol("(:)".to_owned())
                            .spanning(Span::combine(start, end)));
                    }
                    _ => {
                        tokens.push(Token::StartRepeat.spanning(Span::combine(start, middle)));
                        i += 2;
                    }
                },
                _ => {
                    tokens.push(Token::StartGroup.spanning(*start));
                    i += 1;
                }
            },
            Some(Spanned { node: "[", span }) => {
                tokens.push(Token::StartOption.spanning(*span));
                i += 1;
            }
            Some(Spanned { node: "{", span }) => {
                tokens.push(Token::StartRepeat.spanning(*span));
                i += 1;
            }
            Some(Spanned { node: ";", span }) | Some(Spanned { node: ".", span }) => {
                tokens.push(Token::Terminator.spanning(*span));
                i += 1;
            }
            Some(Spanned {
                node: quote,
                span: os,
            }) if *quote == "\'" || *quote == "\"" => {
                let mut string = String::new();
                let mut len = 0;
                i += 1;
                'terminal: loop {
                    match symbols.get(i) {
                        Some(Spanned { node: c, span: oe }) if c == quote => {
                            if len == 0 {
                                return Err(Error::EmptyTerminal.spanning(Span::combine(os, oe)));
                            } else {
                                tokens
                                    .push(Token::Terminal(string).spanning(Span::combine(os, oe)));
                                i += 1;
                                break 'terminal;
                            }
                        }
                        Some(Spanned { node: c, .. }) => {
                            string.push_str(c);
                            i += 1;
                            len += 1;
                        }
                        None => {
                            return Err(Error::UnterminatedTerminal
                                .spanning(symbols.get(i - 1).unwrap().span))
                        }
                    }
                }
            }
            Some(Spanned {
                node: "?",
                span: os,
            }) => {
                let mut string = String::new();
                i += 1;
                'special: loop {
                    match symbols.get(i) {
                        Some(Spanned {
                            node: "?",
                            span: oe,
                        }) => {
                            tokens.push(Token::Special(string).spanning(Span::combine(os, oe)));
                            i += 1;
                            break 'special;
                        }
                        Some(Spanned { node: c, .. }) => {
                            string.push_str(c);
                            i += 1;
                        }
                        None => {
                            return Err(Error::UnterminatedSpecial
                                .spanning(symbols.get(i - 1).unwrap().span))
                        }
                    }
                }
            }
            Some(Spanned { node: c, span: os }) if is_digit(c) => {
                let mut oe = *os;
                let mut integer = to_digit(c).unwrap() as usize;
                i += 1;
                'integer: loop {
                    match symbols.get(i) {
                        Some(Spanned { node: c, span: o }) if is_digit(c) => {
                            integer = integer * 10 + to_digit(c).unwrap() as usize;
                            oe = *o;
                            i += 1;
                        }
                        Some(Spanned { node: c, .. }) if is_whitespace(c) => {
                            i += 1;
                        }
                        _ => {
                            tokens.push(Token::Integer(integer).spanning(Span::combine(os, &oe)));
                            break 'integer;
                        }
                    }
                }
            }
            Some(Spanned { node: c, span: os }) if is_alphabetic(c) => {
                let mut oe = *os;
                let mut string = c.to_string();
                i += 1;
                'nonterminal: loop {
                    match symbols.get(i) {
                        Some(Spanned { node: c, span: o }) if is_alphanumeric(c) => {
                            string.push_str(c);
                            oe = *o;
                            i += 1;
                        }
                        Some(Spanned { node: c, .. }) if is_whitespace(c) => {
                            i += 1;
                        }
                        _ => {
                            tokens
                                .push(Token::Nonterminal(string).spanning(Span::combine(os, &oe)));
                            break 'nonterminal;
                        }
                    }
                }
            }
            Some(Spanned { node: c, span }) => {
                return Err(Error::InvalidSymbol((*c).to_string()).spanning(*span));
            }
            None => break 'tokens,
        }
    }

    Ok(tokens)
}
