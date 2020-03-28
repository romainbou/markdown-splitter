use std::fs::File;
use std::io::prelude::*;
use structopt::StructOpt;

fn write_file(filename: &str, contents: String) -> std::io::Result<()> {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(contents.as_bytes())
}

// Returns line number of the first occurence of a pattern in a string
// -1 otherwive
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

    write_file("export.md", sub_text).unwrap();

}
