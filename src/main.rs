extern crate router;
extern crate iron;
extern crate mysql;
extern crate persistent;
extern crate handlebars_iron;
extern crate rustc_serialize;
extern crate urlencoded;
extern crate mount;
extern crate staticfile;
extern crate chrono;
extern crate hoedown;

#[macro_use]
extern crate lazy_static;
mod app;

fn main() {
    app::run();
}
