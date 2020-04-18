use markdown_splitter::*;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The path to the markdown file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

    /// Output filename (default: `export.md`)
    #[structopt(short = "o", long = "output", default_value = "export.md")]
    output: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let mut file_contents = std::fs::read_to_string(&args.path).expect("could not read file");

    let parts = traverse_text(&mut file_contents);
    save_parts(&args.output.to_str().unwrap(), parts);
}
