#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[cfg(test)]
mod tests;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::{content,NamedFile,Redirect};

const FILENAME: &'static str = "static/index.html";

#[get("/")]
fn root() -> io::Result<NamedFile> {
    NamedFile::open(FILENAME)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[catch(500)]
fn err_exception(req: &rocket::Request) ->  content::Html<String> {
    content::Html(format!("<p>Whoops!,5555, but '{}' is not a valid path!</p>
            <p>The server encountered an internal error while processing this request.</p>
            <p>                                                         by mowatermelon.</p>",
            req.uri()))
}

#[catch(404)]
fn not_found() -> Redirect {
    Redirect::to("/#/NotFound")
}


fn main() {
    let e = rocket::ignite()
        .mount("/", routes![root,files])
        .catch(catchers![not_found,err_exception])
        .launch();

    println!("Whoops! Rocket didn't launch!");
    println!("This went wrong: {}", e);
}