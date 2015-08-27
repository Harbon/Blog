#![allow(dead_code)]
#![allow(unused_imports)]


pub mod helpers ;
pub mod controllers;
pub mod routes;

pub mod models;

use mysql::error::MyResult;
use iron::status;
use router::Router;
use iron::prelude::*;
use handlebars_iron::{Template, HandlebarsEngine};
use mount::Mount;
use std::path::Path;
use staticfile::Static;
use persistent::{Write,Read};
use self::models::doc::Doc;
use self::models::music::Music;
use self::models::picture::Picture;
use self::models::base_triat::*;

 // static mut DOCS: Option<Vec<Doc>> = None;
pub fn run () {
    println!("start run app ");
    let mut mount = Mount::new();
    mount.mount("/", routes::all());
    mount.mount("/static", Static::new(Path::new("/home/Harbon/Blog/src/static/")));
    let mut middleware:Chain = Chain::new(mount);
    middleware.link(Doc::init_data());
    middleware.link(Music::init_data());
    middleware.link(Picture::init_data());
    middleware.link_after(helpers::template_engine_helper::init_template_engine());
    Iron::new(middleware).http("localhost:3000").unwrap();
    println!("stop run app ");
}
