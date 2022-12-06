use fst::automaton::Levenshtein;
use fst::{IntoStreamer, Set};
use std::fmt::{Display, Write};

use crate::Error;

pub fn print_list<T: Display>(iter: impl IntoIterator<Item = T>) -> String {
    let mut iter = iter.into_iter().peekable();
    let mut out = String::new();

    while let Some(el) = iter.next() {
        if iter.peek().is_some() {
            write!(out, "{}, ", el).unwrap();
        } else if out.is_empty() {
            write!(out, "{}", el).unwrap();
        } else {
            write!(out, "or {}", el).unwrap();
        }
    }

    out
}

pub fn find_similar<I, T>(
    words: I,
    query: impl AsRef<str>,
) -> std::result::Result<Vec<String>, Error>
where
    T: AsRef<[u8]>,
    I: IntoIterator<Item = T>,
{
    let words = Set::from_iter(words)?;

    // Build our fuzzy query.
    let lev = Levenshtein::new(query.as_ref(), 3)?;

    // Apply our fuzzy query to the set we built.
    let stream = words.search(lev).into_stream();

    let suggestions = stream.into_strs()?;

    Ok(suggestions)
}
