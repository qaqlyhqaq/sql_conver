use quick_xml::events::Event;
use std::fs::read;
use std::path::PathBuf;

pub struct Pom {
    //pom.xml file path
    file_path: PathBuf,
    //artifactId
    artifact_id: String,
    //是否具有源码文件 , 如果有,则记录路径
    src_store: Option<PathBuf>,
}

impl Pom {
    pub fn form_path(path: PathBuf) {
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

        let mut buf = Vec::new();

        loop {
            let vec1 = buf.clone();

            let event = reader.read_event_into(&mut buf).unwrap();

            let string2 = String::from_utf8(vec1).expect("TODO: panic message");

            match event {
                Event::Start(start_tag) => {
                    let string =
                        String::from_utf8(start_tag.name().0.into()).expect("TODO: panic message");
                    println!("start_tag:{}", string);
                }
                Event::End(end) => {
                    let string =
                        String::from_utf8(end.name().0.into()).expect("TODO: panic message");
                    println!("End:{}", string);
                }
                Event::Decl(decl) => {
                    let string1 = String::from_utf8(decl.to_vec()).unwrap();
                    println!("Decl:{}", string1);
                }
                Event::PI(pi) => {
                    let string1 = String::from_utf8(pi.to_vec()).unwrap();
                    println!("pi:{}", string1);
                }
                Event::DocType(docType) => {
                    let string1 = String::from_utf8(docType.to_vec()).unwrap();
                    println!("DocType:{}", string1);
                }
                Event::CData(data) => {
                    let string1 = String::from_utf8(data.to_vec()).unwrap();
                    println!("CDATA:{}", string1);
                }
                Event::Eof => {
                    break;
                }

                Event::Empty(_) => {}
                Event::Text(text) => {
                    let string = String::from_utf8(text.to_vec()).expect("TODO: panic message");
                    println!("Text:{}", string);
                }
                Event::Comment(comment) => {
                    let string = String::from_utf8(comment.to_vec()).expect("TODO: panic message");
                    println!("Comment:{}", string);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mvn::root_pom_parse::Pom;

    #[test]
    fn is_work() {
        let path1 =
            std::path::Path::new("E:/project/official_website/official-api/official-api-cms");
        Pom::form_path(path1.into());
    }
}
