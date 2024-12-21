mod template_generate;
mod sql;

use convert_case::{Boundary, Case, Casing};
use crate::sql::fetch_table_struct::fetch_table_struct;
use crate::template_generate::generate::generate_context;

#[tokio::main]
async fn main() {

    let x = fetch_table_struct().await;

    let (tera, mut context) = generate_context();

    let mut entity_name = x.table_name.from_case(Case::Snake)
        .without_boundaries(&[Boundary::DigitUpper, Boundary::DigitLower])
        .to_case(Case::Camel);
    entity_name.push_str("Entity");

    context.insert("column_vec", &x.column_vec);
    context.insert("table_name", &x.table_name);
    context.insert("entity_name", &entity_name);

    let rendered = tera.render("sql/hello.tem", &context).unwrap();

    println!("{}", rendered);

}
