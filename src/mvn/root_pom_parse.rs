use std::any::Any;
use std::fmt::{Debug, Formatter};
use quick_xml::events::{BytesStart, Event};
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


trait Stack<T>
where
T:Debug,
{
    fn push(&mut self,item:T);
    fn pop(&mut self,)->T;
    fn len(&self) -> usize;
    fn display(&self,f: &mut Formatter<'_>);
}

impl<T: std::fmt::Display + std::fmt::Debug> Stack<T> for Vec<T> {

    fn push(&mut self, item: T) {
        self.push(item);
    }

    fn pop(&mut self) -> T {
        self.pop().unwrap()
    }

    fn len(&self) -> usize{
        self.len()
    }

    fn display(&self,f: &mut Formatter<'_>){
        write!(f, "{:?}", self).unwrap();
    }

}

impl<T: std::fmt::Debug> Debug for dyn Stack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.display(f);

        return Ok(());
    }
}

impl Pom {
    pub fn form_path(path: PathBuf) -> Pom{
        assert!(path.is_dir(), "路径不为目录");
        assert!(path.exists(), "路径不存在");

        let mut buf = path.to_path_buf();

        buf.push("pom.xml");

        assert!(buf.exists(), "路径下不存在 pom.xml 文件!");

        let vec = read(&buf).unwrap();

        let pom_file_content: String = String::from_utf8(vec).unwrap();

        let mut reader = quick_xml::Reader::from_str(pom_file_content.as_str());
        let config = reader.config_mut();
        config.trim_text(true);
        config.expand_empty_elements = true;

        let mut pom = Pom {
            file_path: buf.to_str().unwrap().into(),
            artifact_id: "".to_string(),
            src_store: None,
            sub_module: vec![],
        };

         let mut  vec1 = Vec::new();
        let stack: &mut dyn Stack<std::string::String> = &mut vec1;

        loop {
            let event = reader.read_event();

            match event.clone() {
                Ok(Event::Start(start_tag))
                if start_tag.name().0.eq("module".as_bytes())
                    && stack.len() == 2 => {
                    let result = reader
                        .clone()
                        .read_text(start_tag.name())
                        .unwrap();
                    let current_path = buf.as_path().clone();
                    let current_path = current_path.parent().unwrap();
                    let current_path = current_path.join(result.clone().to_string());
                    pom.sub_module.push(Pom::form_path(current_path));
                    println!("module:{}", result);
                },
                Ok(Event::Start(start_tag))
                if start_tag.name().0.eq("artifactId".as_bytes())
                && stack.len() == 1=>  {
                    let result = reader
                        .clone()
                        .read_text(start_tag.name()).unwrap();
                    pom.artifact_id = result.to_string();
                },
                Ok(Event::Eof)=>break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => {
                    //ignores !
                    // println!("Event:{}", event_string);
                }
            }


            match event.clone() {
                Ok(Event::End(end_tag)) =>  {
                    let string1 = String::from_utf8(end_tag.name().0.to_vec()).unwrap();
                    let string2 = stack.pop();
                    assert_eq!(string1, string2,"不相等,stack:{:?}",stack);
                },
                Ok(Event::Start(start_tag)) =>  {
                    let string1 = String::from_utf8(start_tag.name().0.to_vec()).unwrap();
                    stack.push(string1);
                },
                _ => {
                    //ignores !
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
