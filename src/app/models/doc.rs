#![allow(dead_code)]
#![allow(unused_imports)]
use super::base_triat::*;
use app::helpers::db_helper::MyDb;
use std::sync::Arc;
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use std::path::Path;
use std::fs::File;
use std::io::Read as FileRead;
use persistent::{Write,Read};
use iron::typemap::Key;
use hoedown::Markdown;
use hoedown::Html;
use hoedown::renderer::html::Flags;
use hoedown::Render;

#[derive(Debug)]
pub struct Doc {
    id: Option<i32>,
    title: Option<String>,
    content: Option<String>,
    date: Option<String>,
}

impl Key for Doc {
    type Value = Vec<Doc>;
}
impl ToJson for Doc {
    fn to_json(&self) -> Json {
        let mut map: BTreeMap<String, Json> = BTreeMap::new();
        map.insert("id".to_string(), self.id.to_json());
        map.insert("title".to_string(), self.title.to_json());
        map.insert("content".to_string(), self.content.to_json());
        map.insert("date".to_string(), self.date.to_json());
        map.to_json()
    }
}

impl BaseStruct<Doc> for Doc {
    fn get_type () -> Option<String> {
        Some("docs".to_string())
    }
    fn new (id: &i32, file_name: &str) -> Option<Doc>{
        println!("id222:{}, file_name:{}", id, file_name);
        let info: Vec<&str> = file_name.split(|c| c == '_' || c=='.').collect();
        println!("name:{}", info[0]);
        let url = "/Users/harbon/Desktop/Rust/Blog_file/src/static/docs/".to_string() + file_name;
        println!("url:{}", url);
        let file_path: &Path = Path::new(&url);
        let mut file: File = File::open(file_path).unwrap();
        let mut content: String = String::new();
        file.read_to_string(&mut content);
        let doc_content = Markdown::new(&content);
        let mut html = Html::new(Flags::empty(),0);
        let html_content = html.render(&doc_content).to_str().unwrap().to_string();

        let doc = Doc{
            id:Some(id.clone()),
            title: Some(info[0].to_string()),
            content: Some(html_content),
            date: Some(info[1].to_string()),
        };
        Some(doc)
    }
    fn init_data() -> (Read<Doc>, Read<Doc>) {
        let docs: Vec<Doc> = Doc::get_all().unwrap();
        Read::<Doc>::both(docs)
    }
}
impl BaseTransaction<Doc> for Doc {}
