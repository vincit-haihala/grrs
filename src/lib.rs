use anyhow::Result;
#[allow(unused_imports)]
use std::io::BufRead;

pub fn find_matches<T>(lines: T, pattern: &str, mut output: impl std::io::Write) -> Result<()>
where
    T: IntoIterator,
    T::Item: ToString,
{
    for line in lines {
        let line = line.to_string();
        if line.contains(pattern) {
            writeln!(output, "{}", line).expect("Writing to buffer failed");
        }
    }
    Ok(())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    let lines = std::io::Cursor::new("lorem ipsum\nsolem dolor\nsit amet")
        .lines()
        .map(|l| l.expect("Error reading line"));
    find_matches(lines, "lorem", &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n")
}
