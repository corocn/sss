#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use handlebars::Handlebars;

use rocket::config::{Config, Environment};
use rocket::response::content;
use rocket::State;
use rocket_contrib::serve::StaticFiles;

use std::fs;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

mod searcher;
use crate::searcher::{get_files, SoundFile};

const TEMPLATE_STR: &str = include_str!("../templates/index.html.hbs");

#[derive(serde::Serialize)]
struct TemplateContext {
    items: Vec<SoundFile>,
}

struct MyConfig {
    full_path: PathBuf,
    ext: String,
}

#[get("/")]
fn index(state: State<MyConfig>) -> content::Html<String> {
    let files = get_files(&state.full_path, &state.ext).unwrap();
    let reg = Handlebars::new();
    let rendered = reg
        .render_template(TEMPLATE_STR, &TemplateContext { items: files })
        .unwrap();
    content::Html(rendered)
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sss", author = "Takahiro Tsuchiya @corocn")]
struct Opt {
    #[structopt(short, long, default_value = "localhost")]
    bind_address: String,

    #[structopt(short, long, default_value = "8000")]
    port: u16,

    #[structopt(short, long, default_value = "wav")]
    ext: String,

    #[structopt(name = "TARGET_DIR", default_value = ".")]
    dir: String,
}

fn main() -> std::io::Result<()> {
    let opt: Opt = Opt::from_args();

    let full_path = fs::canonicalize(Path::new(&opt.dir))?;
    let config = MyConfig {
        full_path: full_path.to_owned(),
        ext: (&opt.ext).to_string(),
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
