use std::fs::read;
use std::path::PathBuf;

pub struct Pom {
    //pom.xml file path
    file_path: PathBuf,
    //artifactId
    artifact_id:String,
    //是否具有源码文件 , 如果有,则记录路径
    src_store:Option<PathBuf>,
}


impl Pom {
    pub fn form_path(path:PathBuf){

        assert!(path.is_dir(),"路径不为目录");
        assert!(!path.exists(),"路径不存在");

        let mut buf = path.to_path_buf();

        buf.push("pom.xml");

        assert!(buf.exists(),"路径下不存在 pom.xml 文件!");

        let vec = read(buf).unwrap();

        let pom_file_content = String::from_utf8(vec).unwrap();

    }
}