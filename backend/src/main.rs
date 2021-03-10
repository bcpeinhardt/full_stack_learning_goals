//! The backend for my website. Main will probably contain the rocket routing stuff,
//! whereas the translation engine will be in a separate module (file)


#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod translate;

use translate::Translator;
use std::collections::HashMap;

#[get("/hello/<name>")]
fn hello(name: String) -> String {
    format!("{}", name)
}

#[get("/")]
fn index() -> &'static str {
    "Index.html"
}

#[get("/translation/<config>/<msg>")]
fn get_translation(config: String, msg: String) -> String {

    // Cache with HashMap from (config, msg) pair to result
    let mut cache: HashMap<(String, String), String> = HashMap::new();

    if let Some(translation) = cache.get(&(config.clone(), msg.clone())) {
        translation.to_string()
    } else {
        if let Ok(translator) = Translator::from(&config) {
            let result = translator.translate(&msg);
            cache.insert((config, msg), result.clone());
            result
        } else {
            "failed to parse config".to_string()
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index, hello, get_translation]).launch();
}
