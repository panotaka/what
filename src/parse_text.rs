use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use hyperpolyglot;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
    vec,
};
use tabled::{object::Rows, Disable, Modify, Rotate, Style, Table, TableIteratorExt};
use whatlang::{detect, Lang, Script};

const ICON: &str = "╭────╮
├────┴───╮
│        │
│        │
╰────────╯";

struct Text {
    file_name: String,
    file_path: PathBuf,
    number_of_lines: usize,
    size: String,
    language: String,
}

pub fn parse_text(text: &PathBuf) {
    let text_content = BufReader::new(File::open(text).unwrap())
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let text_stats = Text {
        file_name: text.file_name().unwrap().to_str().unwrap().to_string(),
        file_path: text.canonicalize().unwrap(),
        // count the number of lines in the file
        number_of_lines: text_content.len(),
        size: human_bytes(get_size(text).unwrap() as f64).to_string(),
        // get the language of the file if hyperpolyglot returns "text", then use whatlang
        language: (|| {
            let lang = hyperpolyglot::detect(text)
                .unwrap()
                .unwrap()
                .language()
                .to_string();
            if lang == "Text" {
                whatlang::detect(&text_content.join("\n"))
                    .unwrap()
                    .lang()
                    .to_string()
            } else {
                lang.to_string()
            }
        })(),
    };

    let content = Table::new(vec![
        [
            text_stats.file_name.clone(),
            text_stats.file_path.clone().display().to_string(),
            text_stats.number_of_lines.to_string(),
            text_stats.size.to_string(),
            text_stats.language.clone(),
            text_stats.language.clone(),
        ],
        [
            "File Name".to_string(),
            "File Path".to_string(),
            "Number Of Lines".to_string(),
            "Size".to_string(),
            "Sub Folders".to_string(),
            "Language".to_string(),
        ],
    ])
    .with(Disable::Row(..1))
    .with(Modify::new(Rows::last()).with(tabled::Alignment::right()))
    .with(Modify::new(Rows::first()).with(tabled::Alignment::left()))
    .with(Rotate::Right)
    .with(Style::blank())
    .to_string();

    let data = vec![[ICON], [&content]];

    let output = data
        .iter()
        .map(|x| x.join("\n"))
        .collect::<Vec<String>>()
        .join("\n");

    println!("{}", output);
}
