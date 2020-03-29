use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

/// Writes `contents` into the file with the given `filename`
///
fn write_file(filename: &str, contents: String) -> std::io::Result<()> {
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
/// let result = find_pattern("pattern".to_string(), &mut "pattern".to_string(), 1, 1);
/// assert_eq!(result, 1);
/// ```
///
/// # Failures
/// No result to find
/// ```
/// let result = find_pattern("".to_string(), &mut "".to_string(), 0, 0);
/// assert_eq!(result, -1);
/// ```
fn find_pattern(pattern: String, text: &mut String, start: i32, end: i32) -> i32 {

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
/// let result = split_text(&mut "line1\nline2\nline3".to_string(), 2, 2);
/// assert_eq!(result, "line2".to_string());
/// ```
fn split_text(text: &mut String, start: i32, end: i32) -> String{
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
fn export_by_pattern(pattern: String, text: &mut String) -> String{

    let mut text = text.clone();

    let mut full_pattern_start = String::from("<!-- export -->");
    let mut full_pattern_end = String::from("<!-- /export -->");
    if pattern != "".to_string() {
        full_pattern_start = format!("{}{}{}", String::from("<!-- export "), pattern, String::from(" -->"));
        full_pattern_end = format!("{}{}{}", String::from("<!-- /export "), pattern, String::from(" -->"));
    }

    let line1 = find_pattern(full_pattern_start, &mut text, -1, -1);
    let line2 = find_pattern(full_pattern_end, &mut text, -1, -1);
    let sub_text = split_text(&mut text, line1, line2);
    return sub_text;
}

#[derive(StructOpt)]
struct Cli {
    /// The path to the markdown file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::from_args();
    let mut file_contents = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    let sub_text = export_by_pattern("".to_string(), &mut file_contents);

    if sub_text.chars().count() <= 0 {
        println!("Nothing to export");
        return;
    }

    write_file("export.md", sub_text).unwrap();
    println!("Wrote in file `export.md`");

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
        assert_eq!(find_pattern("pattern".to_string(), &mut "pattern".to_string(), 1, 1), 1);
    }

    #[test]
    fn find_pattern_line2() {
        assert_eq!(find_pattern("pattern".to_string(), &mut "line1\npattern".to_string(), 1, 2), 2);
    }

    #[test]
    fn find_pattern_out_of_scope() {
        assert_eq!(find_pattern("pattern".to_string(), &mut "pattern".to_string(), 2, 2), -1);
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