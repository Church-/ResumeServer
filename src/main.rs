#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;

use rocket_contrib::{Json, Value};

//Static file serving, for serving up .tar.gz archives of portfolio projects.
#[get("/files/<file>")]
fn files(file: PathBuf) ->  {
    NamedFile::open("/portfolio/projects/".join(file)).ok()
}



#[post("/posts/<id>")]
fn write_posts(id: isize) -> &'static str {
    
}


#[get("/projects/<name>")]
fn get_project(name: &RawStr) -> Json<project> {
	let project = db::Project::from(name);
	
	Json(project)
}


#[get("/posts/<id>")]
fn get_posts(id: i32) -> Json<post> {
	let post = db::Post::from(id);

	Json(post)
}

#[get("/")]
fn index() {}

