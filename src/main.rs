mod template_generate;

use tera::Tera;
use crate::template_generate::generate::generate_context;

fn main() {

    let (tera,context) = generate_context();

    let rendered = tera.render("sql/hello.tem", &context).unwrap();

    println!("{}", rendered);
}
