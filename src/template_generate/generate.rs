use tera::{Context, Tera};

pub fn generate_context() -> (Tera,Context) {
    let tera = Tera::new("templates/**/*").unwrap();
    let mut context:Context = tera::Context::new();

    (tera,context)
}
