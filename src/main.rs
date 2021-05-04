#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use handlebars::Handlebars;

// #[macro_use]
// extern crate serde_json;

use rocket::config::{Config, Environment};
use rocket::response::content;
use rocket_contrib::serve::StaticFiles;
// use rocket_contrib::templates::Template;
use rocket::State;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::path::{PathBuf, Path};
use std::{fs, io};

const TEMPLATE_STR: &str = include_str!("../templates/index.html.hbs");

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

struct MyConfig {
    full_path: PathBuf,
    ext: String,
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

fn get_files(full_path: &Path, filter_ext: &str) -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<PathBuf> = vec![];
    for entry in WalkDir::new(full_path) {
        let entry = entry?;
        let path = entry.path();
        let path_buf = path.to_path_buf();
        if entry.file_type().is_file() {
            if filter_ext != "" {
                if let Some(e) = path_buf.extension() {
                    if e == filter_ext {
                        files.push(path_buf);
                    }
                }
            } else {
                files.push(path_buf);
            }
        }
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

    Some(SoundFile {
        name,
        full_path,
        path,
    })
}

#[get("/")]
fn index(state: State<MyConfig>) -> content::Html<String> {
    let files: Vec<PathBuf> = get_files(&state.full_path, &state.ext).unwrap();
    let files: Vec<SoundFile> = convert_sound_files(files);

    let reg = Handlebars::new();
    let rendered = reg
        .render_template(TEMPLATE_STR, &TemplateContext { items: files })
        .unwrap();
    content::Html(rendered)
}

// TODO: ファイルの制限
// TODO: 絞り込み
// TODO: 複数階層をいいかんじに処理するなにか

use structopt::StructOpt;
use walkdir::{WalkDir, DirEntry};
use std::error::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "sss", author = "Takahiro Tsuchiya @corocn")]
struct Opt {
    #[structopt(short, long, default_value = "localhost")]
    bind_address: String,

    #[structopt(short, long, default_value = "8000")]
    port: u16,

    #[structopt(short, long, default_value = "")]
    ext: String,

    #[structopt(name = "TARGET_DIR", default_value = ".")]
    dir: String,
}

fn main() -> std::io::Result<()> {
    let opt: Opt = Opt::from_args();

    let full_path = fs::canonicalize(Path::new(&opt.dir))?;
    let config = MyConfig {
        full_path: full_path.to_owned(),
        ext: (&opt.ext).to_string()
    };

    println!("{:?} is mapped.", &config.full_path);

    let rocket_config = Config::build(Environment::Production)
        .address(opt.bind_address)
        .port(opt.port)
        .finalize()
        .unwrap();

    rocket::custom(rocket_config)
        .mount("/", routes![index])
        .mount("/_static", StaticFiles::from(&config.full_path))
        .manage(config)
        .launch();

    Ok(())
}
