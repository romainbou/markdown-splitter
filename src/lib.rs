use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const OPENNING_TAG: &str = "<!-- export";
const CLOSING_TAG: &str = "<!-- /export";
const END_TAG: &str = "-->";

/// Writes `contents` into the file with the given `filename`
///
pub fn write_file<P: AsRef<Path>>(filename: P, contents: String) -> std::io::Result<()> {
    let filename_copy = filename.as_ref().clone();
    let mut file = File::create(&filename).expect("Unable to create file");
    let result = file.write_all(contents.as_bytes());
    println!("`{}` created", filename_copy.to_str().unwrap());
    result
}

/// Returns boolean if the line starts by the openning tag
///
/// # Examples
///
/// Opening tag
/// ```
/// let result = markdown_splitter::is_tag("<!-- export ok -->", true);
/// assert_eq!(result, true);
/// ```
/// 
/// Closing tag
/// ```
/// let result = markdown_splitter::is_tag("<!-- /export -->", false);
/// assert_eq!(result, true);
/// ```
/// 
/// ```
/// let result = markdown_splitter::is_tag("not a tag", true);
/// assert_eq!(result, false);
/// ```
pub fn is_tag(line: &str, openning: bool) -> bool {
    let tag = if openning { OPENNING_TAG } else { CLOSING_TAG };
    line.find(&tag) == Some(0)
}

/// Extracts the filename from the openning tag
///
/// # Examples
///
/// ```
/// let result = markdown_splitter::get_export_filename("<!-- export ok -->");
/// assert_eq!(result, "ok");
/// ```
/// ```
/// let result = markdown_splitter::get_export_filename("<!-- export ok.md -->");
/// assert_eq!(result, "ok.md");
/// ```
/// ```
/// let result = markdown_splitter::get_export_filename("<!-- export -->");
/// assert_eq!(result, "");
/// ```
pub fn get_export_filename(line: &str) -> &str {

    // TODO add extension if missing
    // const MARKDOWN_EXTENSION: &str = ".md";

    let first_part: Vec<&str> = line.split(OPENNING_TAG).collect();
    if first_part.len() > 0 {
        let secound_part: Vec<&str> = first_part[1].trim().split(END_TAG).collect();
        if secound_part.len() > 0 {
            let filename = secound_part[0].trim();
            if filename.len() > 0 {
                return filename;
            }
        }
    }
    ""
}

#[derive(Default, Clone)]
pub struct TextPart {
    filename: String,
    contents: String
}

/// Extract parts of of the text if their associated filename
///
pub fn traverse_text(text: &mut String) -> Vec<TextPart> {

    let mut part_collection: Vec<TextPart> = Vec::new();
    let mut is_open: bool = false;
    let mut current_part = TextPart::default();

    for (_, line) in text.as_str().lines().enumerate() {
        if !is_open && is_tag(line, true) {
            is_open = true;
            let filename: &str = get_export_filename(line);

            let part = TextPart {
                filename: String::from(filename),
                contents: String::from("")
            };
            current_part = part;
        }

        if is_open {
            if is_tag(line, false){
                // Closing tag
                is_open = false;

                // Add current part to part collection
                let part_copy = current_part.clone();
                part_collection.push(part_copy)

            } else if !is_tag(line, true) {
                // Add line to current export
                let part_copy = current_part.clone();
                if part_copy.contents.len() > 0 {
                    current_part.contents = format!("{}\n{}", part_copy.contents, line);
                } else {
                    current_part.contents = format!("{}{}", part_copy.contents, line);
                }
            }
        }
    }

    part_collection
}

/// For each parts, save them a file with their filename
///
pub fn save_parts(default_filename: &str, parts: Vec<TextPart>) {
    for part in parts {
        let mut filename = part.filename;
        if filename.len() == 0 {
            filename = default_filename.to_string();
        }
        write_file(filename, part.contents).unwrap();
    }
}