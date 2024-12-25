#![feature(slice_as_array)]
#![feature(let_chains)]
#![feature(if_let_guard)]

mod template_generate;
mod sql;
mod mvn;
mod collection;
mod File;

use std::io::Write;
use std::path::Path;
use convert_case::{Boundary, Case, Casing};
use crate::sql::fetch_table_struct::fetch_table_struct;
use crate::template_generate::generate::generate_context;


fn capitalize_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}


#[tokio::main]
async fn main() {

    let x = fetch_table_struct().await;

    let (tera, mut context) = generate_context();

    let mut entity_name = x.table_name.clone();
    //去除表前缀
    if entity_name.starts_with("tb_"){
        entity_name.remove(0);
        entity_name.remove(0);
        entity_name.remove(0);
    }

    let mut entity_name:String = entity_name.from_case(Case::Snake)
        .without_boundaries(&[Boundary::DigitUpper, Boundary::DigitLower])
        .to_case(Case::Camel);
    //添加后缀
    entity_name.push_str("Entity");

    let entity_name = capitalize_first_letter(entity_name.as_str());


    let chrono_local_now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    context.insert("now_time", &chrono_local_now);
    context.insert("column_vec", &x.column_vec);
    context.insert("table_name", &x.table_name);
    context.insert("entity_name", &entity_name);

    let rendered = tera.render("sql/java_entity.tem", &context).unwrap();

    // println!("{}", rendered);

    //写入到文件
    let java_entity_directory = std::path::Path::new("E:/project/official_website/official-bus/official-bus-cms/src/main/java/com/yymt/bus/cms/persistence/resource/entity");

    if !java_entity_directory.exists()|| !java_entity_directory.is_dir() {
        panic!("无效路径");
    }

    let entity_java_path = format!("{}.java", entity_name);
    let mut java_entity_directory =java_entity_directory.to_path_buf();
    java_entity_directory.push(&entity_java_path);
    // let buf = binding.as_path();
    let buf = java_entity_directory;

    //打开文件
    println!("binding: {:?}", &buf);
    let mut entity_java_file = std::fs::File::options().write(true).append(false).create(true).open(Path::new(&buf)).unwrap();

    write!(entity_java_file, "{}", rendered).unwrap();

    drop(entity_java_file);

    println!("写入完毕!");

}
