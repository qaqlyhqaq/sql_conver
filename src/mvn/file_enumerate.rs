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

impl FileEnumerate {
    pub fn form_path(path: &PathBuf) -> FileEnumerate {
        let directory1 = Directory {
            path:Default::default(),
            current_path_files: vec![],
        };
        FileEnumerate::directory(directory1)
    }
}



#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::mvn::file_enumerate::FileEnumerate;

    #[test]
    fn it_works() {
        FileEnumerate::form_path(&PathBuf::from("E:/rust/sql_conver/src"));
    }
}