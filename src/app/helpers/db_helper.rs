use std::default::Default;
use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use persistent::{Write,Read};
use iron::prelude::Chain;
use iron::BeforeMiddleware;
use iron::AfterMiddleware;
use iron::typemap::Key;


pub struct MyDb;

impl Key for MyDb {
    type Value = MyPool;
}
pub fn init_db_connection_pool (user: &str, pwd: &str, db_name:&str) -> (Read<MyDb>, Read<MyDb>){
    let opts = MyOpts{
        user: Some(user.to_string()),
        pass: Some(pwd.to_string()),
        db_name: Some(db_name.to_string()),
        ..Default::default()
    };
    let pool = MyPool::new(opts).unwrap();
    println!("dbpool init");
    Read::<MyDb>::both(pool)
}
