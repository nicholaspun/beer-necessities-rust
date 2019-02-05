#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate rocket;

use rocket::{response, get, routes};
use rocket_cors;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize, Serialize)]
struct BeerEntry {
    date: String,
    brewery: String,
    name: String,
    description: String,
    aroma: String,
    taste: String,
    finish: String,
    rating: u8
}

#[get("/")]
fn index() -> response::status::Accepted<String> {
    response::status::Accepted(Some(format!("Ok")))
}

#[get("/beers")]
fn get_beers() -> String {
    let mut data = String::new();
    let mut f = File::open("beer.json").expect("Unable to open file");
    f.read_to_string(&mut data).expect("Unable to read string");
    // let deserialized: Vec<BeerEntry> = serde_json::from_str(&data).unwrap();
    data
}


fn main() {
    rocket::ignite()
        .mount("/", routes![index, get_beers])
        .attach(rocket_cors::Cors::default())
        .launch();
}
