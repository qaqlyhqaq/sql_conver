use std::path::PathBuf;

pub enum FileEnumerate {
    //file relatively path Collection
    file(File),
    //directory
    directory(Directory),
}

struct Directory {
    path: PathBuf,
    //directory file
    current_path_files: Vec<File>,
}

struct File {
    path: PathBuf,
}
