#[allow(dead_code)]
use std::sync::Arc;
use std::fs;
use std::fs::*;
use std::path::*;

use persistent::{Write,Read};

#[derive(Debug)]

pub enum FileErr {
    NoSuchFile,
    FileBroken,
}
pub enum FiledValue{
    NULL,
    Int(i32),
    FileName(String),
}
pub type FileResult<T> = Result<T, FileErr>;

 pub trait BaseTransaction<T: BaseStruct<T>>{
    fn get_all() -> Result<Vec<T>,String> {

        let dir_name: String = T::get_type().unwrap();
        let url = "/Users/harbon/Desktop/Rust/Blog_file/src/static/".to_string() + &dir_name;
        let static_path: &Path = Path::new(&url);
        let read_dir: ReadDir = read_dir(static_path).unwrap();
        let mut count = 0i32;
        let file_names: Vec<String> = read_dir.map(|dir_entry|{
                dir_entry.unwrap()
            }).map(|dir_entry|{
                    let file_name = dir_entry.file_name().into_string().unwrap_or("UNRESOLVED_NAME".to_string());
                    file_name
                }).collect();
        let mut objects: Vec<T> = vec![];
        for name in &file_names {
            if !name.starts_with(".") {
                count = count + 1;
                let object = T::new(&count, &name).unwrap();
                objects.push(object);
            }
        }
        Ok(objects)
    }

 }

 pub trait BaseStruct<T> {
     fn get_type() -> Option<String>;
     fn new(id: &i32, file_name: &str) -> Option<T>;
     fn init_data() -> (Read<T>, Read<T>);
 }
