mod template_generate;
mod sql;

use crate::sql::fetch_table_struct::fetch_table_struct;
use crate::template_generate::generate::generate_context;

#[tokio::main]
async fn main() {

    let x = fetch_table_struct().await;

    let (tera, mut context) = generate_context();

    context.insert("column_vec", &x);

    let rendered = tera.render("sql/hello.tem", &context).unwrap();

    println!("{}", rendered);

}
