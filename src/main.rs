#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use handlebars::Handlebars;
use rocket::response::content;

#[derive(serde::Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/")]
fn index() -> content::Html<String> {
    let reg = Handlebars::new();
    let rendered = reg
        .render_template(
            include_str!("../templates/index.html.hbs"),
            &TemplateContext {
                name: String::from("Yamada Taro"),
            },
        )
        .unwrap();
    content::Html(rendered)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
