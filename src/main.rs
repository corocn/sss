#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::path::PathBuf;
use std::{env, fs, io};

#[derive(serde::Serialize)]
struct TemplateContext {
    items: Vec<SoundFile>,
}

#[derive(Debug)]
struct SoundFile {
    name: String,
    path: String,
    full_path: String,
}

impl Serialize for SoundFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("SoundFile", 3)?;
        s.serialize_field("name", &self.name)?;
        s.serialize_field("path", &self.path)?;
        s.serialize_field("full_path", &self.full_path)?;
        s.end()
    }
}

fn get_files() -> io::Result<Vec<PathBuf>> {
    let mut path_buf = env::current_dir().unwrap();
    let dir = String::from("sample");
    path_buf.push(dir);
    let base_path = path_buf.to_str().unwrap();

    let mut files: Vec<PathBuf> = vec![];

    for entry in fs::read_dir(base_path)? {
        let entry = entry?;
        let path = entry.path();
        files.push(path);
    }

    Ok(files)
}

fn convert_sound_files(files: Vec<PathBuf>) -> Vec<SoundFile> {
    let mut sound_files = vec![];
    for file in files {
        let name = file.file_name().unwrap().to_str().unwrap();
        let full_path = file.to_str().unwrap().to_string();
        let path = format!("/_wav/{}", name);

        sound_files.push(SoundFile {
            name: name.to_string(),
            full_path,
            path,
        })
    }

    sound_files
}

#[get("/")]
fn index() -> Template {
    let files: Vec<PathBuf> = get_files().unwrap();
    let files: Vec<SoundFile> = convert_sound_files(files);

    Template::render("index", &TemplateContext { items: files })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
