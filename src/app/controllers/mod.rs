#![allow(dead_code)]
use mysql::error::MyResult;
use iron::status;
use iron::prelude::*;
use app::models::doc::*;
use app::models::music::*;
use app::models::picture::*;
use app::models::base_triat::*;
use mysql::conn::pool::MyPool;
use persistent::{Read};
use app::helpers::db_helper::MyDb;
use std::sync::Arc;
use mysql::conn::QueryResult;
use mysql::value::from_row;
use urlencoded::UrlEncodedQuery;
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use handlebars_iron::Template;
use chrono::*;


//主页
pub fn get_main (_req: &mut Request) -> IronResult<Response> {

    Ok(Response::with((status::Ok, "success")))
}
//获取文章列表
pub fn get_docs_list (_req: &mut Request) -> IronResult<Response> {
    let current_time: String = UTC::now().format("%y-%m-%d").to_string();
    println!("current_time:{}", current_time);
    let docs: Arc<Vec<Doc>> = _req.get::<Read<Doc>>().unwrap();
    let mut tree_map: BTreeMap<String, Json> = BTreeMap::new();
    tree_map.insert("docs".to_string(), docs.to_json());
    Ok(Response::with((status::Ok, Template::new("docs", tree_map))))
}
//显示文章具体信息
// pub fn get_doc (_req: &mut Request) -> IronResult<Response> {
//     let get_querys = _req.get_ref::<UrlEncodedQuery>().unwrap();
//     let id = &(get_querys.get("id").unwrap().get(0).unwrap());
//     let doc:Doc = Doc::get_item_by_id(id, &pool).unwrap();
//     Ok(Response::with((status::Ok, Template::new("docs", doc))))
// }

//获取音乐列表
pub fn get_musics_list (_req: &mut Request) -> IronResult<Response> {
    let musics: Arc<Vec<Music>> = _req.get::<Read<Music>>().unwrap();
    let mut tree_map: BTreeMap<String, Json> = BTreeMap::new();
    tree_map.insert("musics".to_string(), musics.to_json());
    Ok(Response::with((status::Ok, Template::new("musics", tree_map))))
}

//获取图片列表
pub fn get_pictures_list (_req: &mut Request) -> IronResult<Response> {
    let pictures: Arc<Vec<Picture>> = _req.get::<Read<Picture>>().unwrap();
    let mut tree_map: BTreeMap<String, Json> = BTreeMap::new();
    tree_map.insert("pictures".to_string(), pictures.to_json());
    Ok(Response::with((status::Ok, Template::new("pictures", tree_map))))
}
