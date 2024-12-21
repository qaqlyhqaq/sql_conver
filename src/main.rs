use tera::Tera;

fn main() {

    let tera = Tera::new("templates/**/*").unwrap();
    let mut context = tera::Context::new();
    context.insert("name", "World");
    let products = vec!["product1", "product2"];
    context.insert("products", &products);

    let rendered = tera.render("sql/hello.tem", &context).unwrap();
    println!("{}", rendered);
}
