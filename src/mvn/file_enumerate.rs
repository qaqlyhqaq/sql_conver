use std::path::PathBuf;

pub enum  FileEnumerate {
    //file relatively path Collection
    file(File),
    //directory
    directory(Directory),
}

struct Directory {
    path: PathBuf,
}

struct File{
    path: PathBuf,
}