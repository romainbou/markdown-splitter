use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Writes `contents` into the file with the given `filename`
///
pub fn write_file<P: AsRef<Path>>(filename: P, contents: String) -> std::io::Result<()> {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(contents.as_bytes())
}

/// Returns line number of the first occurence of a pattern in a string
///
/// If no reslut is found in the given scope (from start to end) it returns `-1`
///
/// # Examples
///
/// ```
/// let result = markdown_splitter::find_pattern("pattern".to_string(), &mut "pattern".to_string(), 1, 1);
/// assert_eq!(result, 1);
/// ```
///
/// # Failures
/// No result to find
/// ```
/// let result = markdown_splitter::find_pattern("".to_string(), &mut "".to_string(), 0, 0);
/// assert_eq!(result, -1);
/// ```
pub fn find_pattern(pattern: String, text: &mut String, start: i32, end: i32) -> i32 {
    for (line_number, line) in text.as_str().lines().enumerate() {
        let line_number = line_number as i32;
        let line_number = line_number + 1; // Line count starts by 1
        let is_pattern_found = line.find(&pattern);
        if is_pattern_found != None {
            if (start == -1 || line_number >= start) && (end == -1 || line_number <= end) {
                return line_number;
            }
        }
    }

    return -1;
}

/// Returns a part of a given text between lines `start` and `end`
///
/// # Examples
///
/// ```
/// let result = markdown_splitter::split_text(&mut "line1\nline2\nline3".to_string(), 1, 3);
/// assert_eq!(result, "line2".to_string());
/// ```
pub fn split_text(text: &mut String, start: i32, end: i32) -> String {
    let mut sub_text: String = "".to_string();
    for (line_number, line) in text.as_str().lines().enumerate() {
        let line_number = line_number as i32;
        let line_number = line_number + 1; // Line count starts by 1
        if line_number > start && line_number < end {
            sub_text = sub_text + line;
        }
    }
    return sub_text;
}

/// Returns the text between the `<!-- export -->` `<!-- /export -->` tags
///
pub fn export_by_pattern(pattern: String, text: &mut String) -> String {
    let mut text = text.clone();

    let mut full_pattern_start = String::from("<!-- export -->");
    let mut full_pattern_end = String::from("<!-- /export -->");
    if pattern != "".to_string() {
        full_pattern_start = format!(
            "{}{}{}",
            String::from("<!-- export "),
            pattern,
            String::from(" -->")
        );
        full_pattern_end = format!(
            "{}{}{}",
            String::from("<!-- /export "),
            pattern,
            String::from(" -->")
        );
    }

    let line1 = find_pattern(full_pattern_start, &mut text, -1, -1);
    let line2 = find_pattern(full_pattern_end, &mut text, -1, -1);
    let sub_text = split_text(&mut text, line1, line2);
    return sub_text;
}

#[cfg(test)]
mod tests {

    use super::*;

    // find_pattern
    #[test]
    fn find_pattern_empty() {
        assert_eq!(find_pattern("".to_string(), &mut "".to_string(), 1, 1), -1);
    }

    #[test]
    fn find_pattern_found() {
        assert_eq!(
            find_pattern("pattern".to_string(), &mut "pattern".to_string(), 1, 1),
            1
        );
    }

    #[test]
    fn find_pattern_line2() {
        assert_eq!(
            find_pattern(
                "pattern".to_string(),
                &mut "line1\npattern".to_string(),
                1,
                2
            ),
            2
        );
    }

    #[test]
    fn find_pattern_out_of_scope() {
        assert_eq!(
            find_pattern("pattern".to_string(), &mut "pattern".to_string(), 2, 2),
            -1
        );
    }

    // split_text
    #[test]
    fn split_text_example() {
        let result = split_text(&mut "line1\nline2\nline3".to_string(), 1, 3);
        assert_eq!(result, "line2".to_string());
    }

    #[test]
    fn split_text_empty() {
        let result = split_text(&mut "line1\nline2\nline3".to_string(), 2, 2);
        assert_eq!(result, "".to_string());
    }
}
