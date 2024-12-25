use quick_xml::events::Event;
use std::fs::read;
use std::io::Read;
use std::path::PathBuf;
use quick_xml::name::QName;

#[derive(Debug)]
pub struct Pom {
    //pom.xml file path
    file_path: PathBuf,
    //artifactId
    artifact_id: String,
    //是否具有源码文件 , 如果有,则记录路径
    src_store: Option<PathBuf>,
    sub_module: Vec<Pom>,
}

impl Pom {
    pub fn form_path(path: PathBuf) -> Pom{
        assert!(path.is_dir(), "路径不为目录");
        assert!(path.exists(), "路径不存在");

        let mut buf = path.to_path_buf();

        buf.push("pom.xml");

        assert!(buf.exists(), "路径下不存在 pom.xml 文件!");

        let vec = read(buf).unwrap();

        let pom_file_content: String = String::from_utf8(vec).unwrap();

        let mut reader = quick_xml::Reader::from_str(pom_file_content.as_str());
        let config = reader.config_mut();
        config.trim_text(true);
        config.expand_empty_elements = true;

        let mut pom = Pom {
            file_path: Default::default(),
            artifact_id: "".to_string(),
            src_store: None,
            sub_module: vec![],
        };

        loop {
            match reader.read_event() {
                Ok(Event::Start(start_tag))
                    if start_tag.name().0.eq("module".as_bytes()) =>  {
                    let result = reader.read_text(start_tag.name()).unwrap();
                    println!("text:{}", result);
                },
                Ok(Event::Start(start_tag))
                if start_tag.name().0.eq("project".as_bytes()) =>  {
                    let result = reader.read_text(start_tag.name()).unwrap();
                    pom.artifact_id = result.to_string();
                },
                Ok(Event::Eof)=>break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => {
                    //ignores !
                    // println!("Event:{}", event_string);
                }
            }
        }

        pom
    }
}

#[cfg(test)]
mod tests {
    use crate::mvn::root_pom_parse::Pom;

    #[test]
    fn is_work() {
        let path1 =
            // std::path::Path::new("E:/project/official_website/official-api/official-api-cms");
            // std::path::Path::new("E:/project/official_website/official-bus");
            std::path::Path::new("E:/project/official_website");
        let pom =Pom::form_path(path1.into());
        dbg!(pom);
    }
}
