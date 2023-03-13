use std::fmt::Display;
use std::fmt::Write;

use crate::Error;

pub trait IteratorExt<T, E> {
    fn partition_result<FromT, FromE>(self) -> Result<FromT, FromE>
    where
        FromT: FromIterator<T>,
        FromE: FromIterator<E>;
}

impl<T, E, I> IteratorExt<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
{
    fn partition_result<FromT, FromE>(self) -> Result<FromT, FromE>
    where
        FromT: FromIterator<T>,
        FromE: FromIterator<E>,
    {
        let (types, errors): (Vec<_>, Vec<_>) = self.partition(Result::is_ok);

        if errors.is_empty() {
            let results: FromT = types
                .into_iter()
                .map(|v| unsafe { v.unwrap_unchecked() })
                .collect();

            Ok(results)
        } else {
            let errors: FromE = errors
                .into_iter()
                .map(|v| unsafe { v.unwrap_err_unchecked() })
                .collect();

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

pub fn detect_invalid_input(input: &str) -> crate::Result<()> {
    // Disallow specific codepoints.
    for (pos, ch) in input.chars().enumerate() {
        match ch {
            '\n' | '\r' | '\t' => {}

            // Bidirectional override codepoints can be used to craft source code that
            // appears to have a different meaning than its actual meaning. See
            // [CVE-2021-42574] for background and motivation.
            //
            // [CVE-2021-42574]: https://cve.mitre.org/cgi-bin/cvename.cgi?name=CVE-2021-42574
            '\u{202a}' | '\u{202b}' | '\u{202c}' | '\u{202d}' | '\u{202e}' | '\u{2066}'
            | '\u{2067}' | '\u{2068}' | '\u{2069}' => {
                return Err(Error::bidirectional_override_codepoint(pos));
            }

            // Disallow several characters which are deprecated or discouraged in Unicode.
            //
            // U+149 deprecated; see Unicode 13.0.0, sec. 7.1 Latin, Compatibility Digraphs.
            // U+673 deprecated; see Unicode 13.0.0, sec. 9.2 Arabic, Additional Vowel Marks.
            // U+F77 and U+F79 deprecated; see Unicode 13.0.0, sec. 13.4 Tibetan, Vowels.
            // U+17A3 and U+17A4 deprecated, and U+17B4 and U+17B5 discouraged; see
            // Unicode 13.0.0, sec. 16.4 Khmer, Characters Whose Use Is Discouraged.
            '\u{149}' | '\u{673}' | '\u{f77}' | '\u{f79}' | '\u{17a3}' | '\u{17a4}'
            | '\u{17b4}' | '\u{17b5}' => {
                return Err(Error::deprecated_codepoint(pos));
            }

            // Disallow control codes other than the ones explicitly recognized above,
            // so that viewing a wit file on a terminal doesn't have surprising side
            // effects or appear to have a different meaning than its actual meaning.
            ch if ch.is_control() => {
                return Err(Error::control_code(pos));
            }

            _ => {}
        }
    }

    Ok(())
}
