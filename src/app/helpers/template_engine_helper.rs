use handlebars_iron::{Template, HandlebarsEngine};

pub fn init_template_engine() -> HandlebarsEngine {
    HandlebarsEngine::new("/Users/harbon/Desktop/Rust/Blog_file/src/templates", ".hbs")
}
