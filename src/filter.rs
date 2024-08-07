#![warn(clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

use anyhow::Result;
use iter_tools::Itertools;

/// Returns a list of matches. A match is equivalent to `[start..end]`
pub fn filter(pattern: &str, buffer: &[u8]) -> Result<Vec<(usize, usize)>> {
    let regex = regex::bytes::RegexBuilder::new(pattern)
        .unicode(false)
        .dot_matches_new_line(true)
        .build()?;

    let mut matches = vec![];
    for regex_match in regex.find_iter(buffer) {
        let start = regex_match.start();
        let end = regex_match.end();
        matches.push((start, end));
    }

    Ok(matches)
}

#[allow(dead_code)]
pub fn parse_buffer(buffer: &str) -> Vec<u8> {
    let buffer = buffer
        .split_whitespace()
        .filter(|v| !v.is_empty())
        .map(|v| {
            #[allow(clippy::unwrap_used)]
            let temp = u8::from_str_radix(v, 16).unwrap();
            temp
        })
        .collect_vec();

    buffer
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {

    use super::*;

    #[test]
    fn no_unicode() {
        let pattern = r"\x01.{31}\x02.{31}";
        let buffer = parse_buffer(
            "
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            01 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  86 18 45 00
            02 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
            00 00 00 00  00 00 00 00  00 00 00 00  87 19 46 01
        ",
        );

        let &(start, end) = filter(pattern, &buffer).unwrap().first().unwrap();

        assert_eq!(start, 32);
        assert_eq!(end, 96);
    }

    #[test]
    fn dot_matches_new_line() {
        let pattern = r"\x01.*";
        let buffer = parse_buffer(
            "
            01 00 00 00  00 00 00 00  00 00 00 00  00 00 00 0a
            02 00 00 00  00 00 00 00  00 00 00 00  00 00 00 00
        ",
        );

        let &(start, end) = filter(pattern, &buffer).unwrap().first().unwrap();

        assert_eq!(start, 0);
        assert_eq!(end, 32);
    }

    #[test]
    fn plaintext() {
        let pattern = r"(?i)rizard";
        let buffer = b"CHARIZARD".as_slice();

        let &(start, end) = filter(pattern, buffer).unwrap().first().unwrap();

        assert_eq!(start, 3);
        assert_eq!(end, 9);
    }
}
