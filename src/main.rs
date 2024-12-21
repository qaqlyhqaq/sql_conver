use tera::Tera;

fn main() {

    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();
    context.insert("name", "World");

    let rendered = tera.render("sql/hello.tem", &context).unwrap();
    println!("{}", rendered);
}
