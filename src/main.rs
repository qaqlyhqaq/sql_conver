







fn main() {
    use tera::Tera;

    // Create a new Tera instance and add a template from a string
    let mut tera = Tera::new("templates/**/*").unwrap();
    // tera.add_raw_template("hello", "Hello, {{ name }}!").unwrap();
    // Prepare the context with some data
    let mut context = tera::Context::new();
    context.insert("name", "World");

    // Render the template with the given context
    let rendered = tera.render("sql/hello.tem", &context).unwrap();
    println!("{}", rendered);
    // assert_eq!(rendered, "Hello, World!");

    // println!("Hello, world!");
}
