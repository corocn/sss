#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::templates::Template;
use std::{env, fs};

#[derive(serde::Serialize)]
struct TemplateContext<'a> {
    items: Vec<&'a str>,
}

#[get("/")]
fn index() -> Template {
    let mut path_buf = env::current_dir().unwrap();
    let dir = String::from("sample");
    path_buf.push(dir);
    let base_path = path_buf.to_str().unwrap();

    // for entry in fs::read_dir(base_path) {
    //     let path = entry.path();
    //
    //     dbg!(path.to_str().unwrap())
    // }

    Template::render("index", &TemplateContext {
        items: vec!["One", "Two", "Three"],
    })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
