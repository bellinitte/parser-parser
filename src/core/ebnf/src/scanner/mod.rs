use error::{Error, ErrorKind};
use unicode_segmentation::UnicodeSegmentation;

pub mod error;

pub(super) type Symbol = (usize, char);

#[derive(Debug, Clone)]
pub struct Symbols {
    symbols: Vec<Symbol>,
    len: usize,
}

pub(super) fn scan<'a>(string: &'a str) -> Result<Vec<Symbol>, Error> {
    string
        .grapheme_indices(true)
        .zip(0..)
        .map(|((_, s), i)| s.chars().map(move |c| (i, c)))
        .flatten()
        .map(|(i, c)| -> Result<(usize, char), Error> {
            if c.is_control() && !c.is_whitespace() {
                Err(Error {
                    kind: ErrorKind::ControlCharacter(c),
                    position: i,
                })
            } else {
                Ok((i, c))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::scan;
    use super::{Error, ErrorKind};

    #[test]
    fn test_control_characters() {
        use std::str;

        assert_eq!(
            scan(str::from_utf8(&[0x41, 0x01, 0x02, 0x42]).unwrap()),
            Err(Error {
                kind: ErrorKind::ControlCharacter('\u{1}'),
                position: 1,
            })
        );
        assert_eq!(
            scan(str::from_utf8(&[0x0a, 0x0d]).unwrap()),
            Ok(vec![(0, '\n'), (1, '\r')])
        );
    }

    #[test]
    fn test_multiline() {
        assert_eq!(
            scan(" abc \n = def "),
            Ok(vec![
                (0, ' '),
                (1, 'a'),
                (2, 'b'),
                (3, 'c'),
                (4, ' '),
                (5, '\n'),
                (6, ' '),
                (7, '='),
                (8, ' '),
                (9, 'd'),
                (10, 'e'),
                (11, 'f'),
                (12, ' ')
            ])
        );
    }
}
