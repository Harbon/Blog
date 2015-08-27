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

pub struct Music {
    id: Option<i32>,
    url: Option<String>,
    icon_url: Option<String>,
    title: Option<String>,
    date: Option<String>,
}
impl Key for Music {
    type Value = Vec<Music>;
}
impl ToJson for Music {
    fn to_json (&self) -> Json {
        let mut map: BTreeMap<String , Json> = BTreeMap::new();
        map.insert("id".to_string(), self.id.to_json());
        map.insert("url".to_string(), self.url.to_json());
        map.insert("icon_url".to_string(), self.icon_url.to_json());
        map.insert("title".to_string(), self.title.to_json());
        map.insert("date".to_string(), self.date.to_json());
        map.to_json()
    }
}
impl BaseStruct<Music> for Music {
    fn get_type() -> Option<String> {
        Some("musics".to_string())
    }
    fn new(id: &i32, file_name: &str) -> Option<Music>{
        let info: Vec<&str> = file_name.split(|c| c=='_' || c=='.').collect();
        let url: String = "/static/musics/".to_string() + file_name;
        let icon_url: String = "/static/music_icons/".to_string() + info[0] + ".jpg";
        let music = Music {
            id: Some(id.clone()),
            title: Some(info[0].to_string()),
            url:Some(url),
            icon_url:Some(icon_url),
            date: Some(info[1].to_string()),
        };
        Some(music)
    }
    fn init_data() -> (Read<Music>, Read<Music>) {
        let musics: Vec<Music> = Music::get_all().unwrap();
        Read::<Music>::both(musics)
    }
}
impl BaseTransaction<Music> for Music {}
