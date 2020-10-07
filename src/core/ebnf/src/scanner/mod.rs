use error::{Error, ErrorKind};
use unicode_segmentation::UnicodeSegmentation;
use super::error::{Location, Span};

pub mod error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Symbol {
    grapheme: String,
    span: Span,
}

pub(super) fn scan<'a>(string: &'a str) -> Result<Vec<Symbol>, Error> {
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
                grapheme: grapheme.to_owned(),
                span: Span {
                    from: current_location,
                    to: *location,
                }
            })
        })
        .map(|symbol| Ok(symbol))
        .collect::<Result<Vec<Symbol>, Error>>()
}

#[cfg(test)]
mod tests {
    use super::scan;
    use super::{Error, ErrorKind};
    use super::Symbol;
    use super::Span;

    #[test]
    fn test_control_characters() {
        use std::str;

        assert_eq!(
            scan(str::from_utf8(&[0x41, 0x01, 0x02, 0x42]).unwrap()),
            Err(Error {
                kind: ErrorKind::ControlCharacter("\u{1}".into()),
                position: 1,
            })
        );
        assert_eq!(
            scan(str::from_utf8(&[0x0a, 0x0d]).unwrap()),
            Ok(vec![
                Symbol { grapheme: "\n".into(), span: Span::from(((0, 0), (0, 1))) },
                Symbol { grapheme: "\r".into(), span: Span::from(((0, 1), (0, 2))) },
            ])
        );
    }

    #[test]
    fn test_multiline() {
        assert_eq!(
            scan(" abc \n = def "),
            Ok(vec![
                Symbol { grapheme: " ".into(), span: Span::from(((0, 0), (1, 0))) },
                Symbol { grapheme: "a".into(), span: Span::from(((1, 0), (2, 0))) },
                Symbol { grapheme: "b".into(), span: Span::from(((2, 0), (3, 0))) },
                Symbol { grapheme: "c".into(), span: Span::from(((3, 0), (4, 0))) },
                Symbol { grapheme: " ".into(), span: Span::from(((4, 0), (5, 0))) },
                Symbol { grapheme: "\n".into(), span: Span::from(((5, 0), (0, 1))) },
                Symbol { grapheme: " ".into(), span: Span::from(((0, 1), (1, 1))) },
                Symbol { grapheme: "=".into(), span: Span::from(((1, 1), (2, 1))) },
                Symbol { grapheme: " ".into(), span: Span::from(((2, 1), (3, 1))) },
                Symbol { grapheme: "d".into(), span: Span::from(((3, 1), (4, 1))) },
                Symbol { grapheme: "e".into(), span: Span::from(((4, 1), (5, 1))) },
                Symbol { grapheme: "f".into(), span: Span::from(((5, 1), (6, 1))) },
                Symbol { grapheme: " ".into(), span: Span::from(((6, 1), (7, 1))) },
            ])
        );
    }

    #[test]
    fn test_multiple_unicode_code_points() {
        assert_eq!(
            scan("aéf = abc;"),
            Ok(vec![
                Symbol { grapheme: "a".into(), span: Span::from(((0, 0), (1, 0))) },
                Symbol { grapheme: "e\u{301}".into(), span: Span::from(((1, 0), (3, 0))) },
                Symbol { grapheme: "f".into(), span: Span::from(((3, 0), (4, 0))) },
                Symbol { grapheme: " ".into(), span: Span::from(((4, 0), (5, 0))) },
                Symbol { grapheme: "=".into(), span: Span::from(((5, 0), (6, 0))) },
                Symbol { grapheme: " ".into(), span: Span::from(((6, 0), (7, 0))) },
                Symbol { grapheme: "a".into(), span: Span::from(((7, 0), (8, 0))) },
                Symbol { grapheme: "b".into(), span: Span::from(((8, 0), (9, 0))) },
                Symbol { grapheme: "c".into(), span: Span::from(((9, 0), (10, 0))) },
                Symbol { grapheme: ";".into(), span: Span::from(((10, 0), (11, 0))) },
            ])
        );
    }
}
