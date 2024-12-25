use std::path::PathBuf;
use tokio::fs::File;

pub enum  DirectoryFileEnumerate {
    //file relatively path Collection
    files(Vec<PathBuf>),
    //directory
    directory(Vec<DirectoryFileEnumerate>,PathBuf),
}