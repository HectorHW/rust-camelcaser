use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn capitalize_rust(s:&str) -> PyResult<String> {
    let mut buffer = String::with_capacity(s.len());

    let mut iterator = s.split('_');

    buffer.push_str(iterator.next().unwrap());

    Ok(iterator.map(
        |s| capitalize(s)
    ).fold(buffer, |mut buf, sub| {buf.push_str(&sub); buf} ))

}

fn capitalize(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut chars = s.char_indices();

    let (_, first_letter) = chars.next().unwrap();

    let mut buffer = String::new();

    for subchar in first_letter.to_uppercase() {
        buffer.push(subchar);
    }

    let Some((offset_of_rest, _)) = chars.next() else {
        return buffer;
    };

    buffer.push_str(s.split_at(offset_of_rest).1);

    buffer
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