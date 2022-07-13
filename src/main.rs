use clap::Parser;

mod parse_folder;
mod parse_text;

/// A program to generate a detailed report of a file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The file(s) to analyze
    #[clap(required = true, parse(from_os_str))]
    files: Vec<std::path::PathBuf>,

    /// Search folders recursively
    #[clap(short, long)]
    recursive: bool,
}

fn get_files_recursively(path: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut files: Vec<std::path::PathBuf> = Vec::new();
    for file in path.read_dir().unwrap() {
        let file = file.unwrap();
        if file.file_type().unwrap().is_dir() {
            files.extend(get_files_recursively(&file.path()));
        } else {
            files.push(file.path());
        }
    }
    files
}

fn main() {
    let args = Args::parse();

    let files = if args.recursive {
        let mut file_vec = Vec::new();
        for file in &args.files {
            if file.is_dir() {
                file_vec.extend(get_files_recursively(file));
            } else {
                file_vec.push(file.clone());
            }
        }
        file_vec
    } else {
        args.files.clone()
    };

    for file in &files {
        if file.is_dir() {
            parse_folder::parse_folder(file);
        } else {
            parse_text::parse_text(file);
        }
    }
}
