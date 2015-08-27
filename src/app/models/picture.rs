#![allow(dead_code)]
#![allow(unused_imports)]
use super::base_triat::*;
use app::helpers::db_helper::MyDb;
use persistent::{Read};
use std::sync::Arc;
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use iron::typemap::Key;

pub struct Picture {
    id: Option<i32>,
    url: Option<String>,
    title: Option<String>,
    date: Option<String>,
}
impl Key for Picture {
    type Value = Vec<Picture>;
}
impl ToJson for Picture {
    fn to_json (&self) -> Json{
        let mut map: BTreeMap<String , Json> = BTreeMap::new();
        map.insert("id".to_string(), self.id.to_json());
        map.insert("url".to_string(), self.url.to_json());
        map.insert("title".to_string(), self.title.to_json());
        map.insert("date".to_string(), self.date.to_json());
        map.to_json()
    }
}
impl BaseStruct<Picture> for Picture {
    fn get_type() -> Option<String> {
        Some("pictures".to_string())
    }
    fn new(id: &i32, file_name: &str) -> Option<Picture> {
        let info: Vec<&str> = file_name.split(|c| c == '_' || c == '.').collect();
        let url: String = "/static/pictures/".to_string() + file_name;
        let picture = Picture {
            id: Some(id.clone()),
            url: Some(url),
            title: Some(info[0].to_string()),
            date: Some(info[1].to_string()),
        };
        Some(picture)
    }
    fn init_data() -> (Read<Picture>, Read<Picture>) {
        let pictures: Vec<Picture> = Picture::get_all().unwrap();
        Read::<Picture>::both(pictures)
    }
}

impl BaseTransaction<Picture> for Picture {}
