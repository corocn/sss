#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use handlebars::Handlebars;

#[macro_use]
extern crate serde_json;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::path::PathBuf;
use std::{env, fs, io};
use rocket::config::{ Config, Environment};
use rocket::response::content;

const template_str: &str = include_str!("../templates/index.html.hbs");

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
    let base_path = env::current_dir().unwrap();
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
    for path in files {
        if let Some(x) = path_buf_to_sound_file(&path) {
            sound_files.push(x);
        }
    }

    sound_files
}

fn path_buf_to_sound_file(path: &PathBuf) -> Option<SoundFile> {
    let name = path.file_name()?.to_str()?.to_string();
    let full_path = path.to_str()?.to_string();
    let path = format!("/_static/{}", name);

    Some(SoundFile { name, full_path, path })
}

#[get("/")]
fn index() -> content::Html<String> {
    let files: Vec<PathBuf> = get_files().unwrap();
    let files: Vec<SoundFile> = convert_sound_files(files);

    let mut reg = Handlebars::new();
    let rendered = reg.render_template(template_str, &TemplateContext { items: files }).unwrap();
    content::Html(rendered)
}

// TODO: 対象ディレクトリの指定
// TODO: ファイルの制限
// TODO: 絞り込み
// TODO: bind address
// TODO: port

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "sss", author = "Takahiro Tsuchiya @corocn")]
struct Opt {
    #[structopt(short, long, default_value = "localhost")]
    bind_address: String,

    #[structopt(short, long, default_value = "8000")]
    port: u16,
}

fn main() {
    let opt = Opt::from_args();
    dbg!(&opt);

    let current_dir = env::current_dir().unwrap().to_str().unwrap().to_string();
    println!("current directory: {}", &current_dir);

    let config = Config::build(Environment::Production)
        .address(opt.bind_address)
        .port(opt.port)
        .finalize().unwrap();

    rocket::custom(config)
        .mount("/", routes![index])
        .mount("/_static", StaticFiles::from(current_dir))
        .launch();
}
