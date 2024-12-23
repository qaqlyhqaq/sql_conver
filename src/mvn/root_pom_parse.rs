use std::fs::read;
use std::path::PathBuf;
use quick_xml::events::Event;

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

        let pom_file_content:String = String::from_utf8(vec).unwrap();

        let mut reader = quick_xml::Reader::from_str(pom_file_content.as_str());
        let config = reader.config_mut();
        config.trim_text(true);
        config.expand_empty_elements = true;

        let mut buf = Vec::new();

        loop {
            let event = reader.read_event_into(&mut buf).unwrap();

            match event {
                Event::Start(start_tag) => {
                    let string = String::from_utf8(start_tag.name().0.into()).expect("TODO: panic message");
                    println!("start_tag:{}",string);
                }
                Event::Eof => {
                    break;
                }
                _ =>{
                    //ignore
                }
            }
        }

        }
}


#[cfg(test)]
mod tests{
    #[test]
    fn is_work(){

    }
}