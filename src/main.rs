mod template_generate;
mod sql;

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

    println!("{}", rendered);

}
