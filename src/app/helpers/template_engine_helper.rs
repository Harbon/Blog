use handlebars_iron::{Template, HandlebarsEngine};

pub fn init_template_engine() -> HandlebarsEngine {
    // HandlebarsEngine::new("/home/Harbon/Blog/src/templates", ".hbs")
    HandlebarsEngine::new("/home/harbon/Blog/src/templates", ".hbs")
}
