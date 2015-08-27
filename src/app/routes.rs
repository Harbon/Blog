#![allow(dead_code)]
use app::controllers;
use router::Router;

pub fn all() -> Router {
	let mut router = Router::new();
    router.get("/", controllers::get_main);
    router.get("/docs", controllers::get_docs_list);
	// router.get("/docs/:id", controllers::get_doc);
	router.get("/musics", controllers::get_musics_list);
	router.get("/pictures", controllers::get_pictures_list);
    router
}
