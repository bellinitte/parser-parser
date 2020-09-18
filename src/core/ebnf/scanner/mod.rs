use error::Error;
use unicode_segmentation::UnicodeSegmentation;

pub mod error;

pub(super) type Symbol = (usize, char);

pub(super) fn scan<'a>(string: &'a str) -> Result<Vec<Symbol>, Error> {
    string
        .grapheme_indices(true)
        .zip(0..)
        .map(|((_, s), i)| s.chars().map(move |c| (i, c)))
        .flatten()
        .map(|(i, c)| -> Result<(usize, char), Error> {
            if c.is_control() && !c.is_whitespace() {
                Err(Error::ControlCharacter(c))
            } else {
                Ok((i, c))
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::scan;
    use super::Error;

    #[test]
    fn test_control_characters() {
        use std::str;

        assert_eq!(
            scan(str::from_utf8(&[0x41, 0x01, 0x02, 0x42]).unwrap()),
            Err(Error::ControlCharacter('\u{1}'))
        );
        assert_eq!(
            scan(str::from_utf8(&[0x0a, 0x0d]).unwrap()),
            Ok(vec![(0, '\n'), (1, '\r')])
        );
    }
}
