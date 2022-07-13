use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use std::{path::PathBuf, vec};
use tabled::{
    object::{Rows},
    Disable, Modify, Rotate, Style, Table, TableIteratorExt,
};

const ICON: &str = "╭────╮
├────┴───╮
│        │
│        │
╰────────╯";

struct Folder {
    folder_name: String,
    folder_path: PathBuf,
    number_of_files: usize,
    size: String,
    sub_folders: bool,
}

pub fn parse_folder(folder: &PathBuf) {
    let folder_stats = Folder {
        folder_name: folder.file_name().unwrap().to_str().unwrap().to_string(),
        folder_path: folder.canonicalize().unwrap(),
        number_of_files: folder.read_dir().unwrap().count(),
        size: human_bytes(get_size(folder).unwrap() as f64).to_string(),
        sub_folders: folder
            .read_dir()
            .unwrap()
            .map(|x| x.unwrap().file_type().unwrap().is_dir())
            .any(|x| x),
    };

    let content = Table::new(vec![
        [
            folder_stats.folder_name.clone(),
            folder_stats.folder_path.clone().display().to_string(),
            folder_stats.number_of_files.to_string(),
            folder_stats.size.to_string(),
            folder_stats.sub_folders.to_string(),
        ],
        [
            "Folder Name".to_string(),
            "Folder Path".to_string(),
            "Number Of Files".to_string(),
            "Size".to_string(),
            "Sub Folders".to_string(),
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
        .table()
        .with(Disable::Row(..1))
        .with(Modify::new(Rows::first()).with(tabled::Alignment::center()))
        .with(Modify::new(Rows::first()).with(tabled::Alignment::center_vertical()))
        .with(Rotate::Left)
        .with(Style::rounded());
    bunt::print!("{}", output);
}
