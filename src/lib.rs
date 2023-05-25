use std::char::ToUppercase;

use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn capitalize_rust(s:&str) -> PyResult<String> {
    let mut buffer = String::with_capacity(s.len());

    let mut iterator = s.split('_');

    buffer.push_str(iterator.next().unwrap());

    Ok(iterator.fold(buffer, |mut buf, s| {
        if let Some((capitalized_chars, string_tail)) = capitalize(s) {
            for char in capitalized_chars {
                buf.push(char);
            }
            buf.push_str(string_tail);
        }

        buf
    } ))

}

fn capitalize(s: &str) -> Option<(ToUppercase, &str)> {
    if s.is_empty() {
        return None;
    }
    let mut chars = s.chars();

    let first_letter = chars.next().unwrap();

    Some((
        first_letter.to_uppercase(), s.split_at(first_letter.len_utf8()).1
    ))
}

/// A Python module implemented in Rust.
#[pymodule]
fn tocamel(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(capitalize_rust, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests{
    use crate::capitalize_rust;

    #[test]
    fn string_capitalizes() {
        assert_eq!(capitalize_rust("after_before").unwrap(), "afterBefore")
    }

    #[test]
    fn string_without_spaces_capitalizes() {
        assert_eq!(capitalize_rust("action").unwrap(), "action")
    }

    #[test]
    fn empty_string_capitalizes() {
        assert_eq!(capitalize_rust("").unwrap(), "")
    }
}