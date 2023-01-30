use std::fmt::{Display, Write};

pub trait IteratorExt<T, E> {
    fn partition_result(self) -> Result<Vec<T>, Vec<E>>;
}

impl<T, E, I: Iterator<Item = Result<T, E>>> IteratorExt<T, E> for I {
    fn partition_result(self) -> Result<Vec<T>, Vec<E>> {
        let (types, errors): (Vec<_>, Vec<_>) = self.partition(Result::is_ok);

        let types = types
            .into_iter()
            .map(|v| unsafe { v.unwrap_unchecked() })
            .collect();

        let errors: Vec<_> = errors
            .into_iter()
            .map(|v| unsafe { v.unwrap_err_unchecked() })
            .collect();

        if errors.is_empty() {
            Ok(types)
        } else {
            Err(errors)
        }
    }
}

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
