use tera::{Context, Tera};

pub fn generate_context() -> (Tera,Context) {
    let tera = Tera::new("templates/**/*").unwrap();
    let mut context:Context = tera::Context::new();
    context.insert("name", "World");
    let products = vec!["product1", "product2"];
    context.insert("products", &products);

    (tera,context)
}
