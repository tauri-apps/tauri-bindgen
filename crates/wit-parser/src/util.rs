use std::fmt::Display;
use std::fmt::Write;

pub fn print_list<T: Display>(iter: impl IntoIterator<Item = T>) -> String {
    let mut iter = iter.into_iter().peekable();
    let mut out = String::new();

    while let Some(el) = iter.next() {
        if iter.peek().is_some() {
            write!(out, "{el}, ").unwrap();
        } else if out.is_empty() {
            write!(out, "{el}").unwrap();
        } else {
            write!(out, "or {el}").unwrap();
        }
    }

    out
}

pub fn find_similar<I, T>(words: I, query: impl AsRef<str>) -> Vec<String>
where
    T: AsRef<str>,
    I: IntoIterator<Item = T>,
{
    words
        .into_iter()
        .filter_map(|word| {
            if distance::damerau_levenshtein(word.as_ref(), query.as_ref()) <= 3 {
                Some(word.as_ref().to_string())
            } else {
                None
            }
        })
        .collect()
}
