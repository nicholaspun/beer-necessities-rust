#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate rocket;

use rocket::response::status;
use rocket::http::Status;

struct BeerEntry {
    id: u32,
    brewery: String,
    name: String,
    description: String,
    aroma: String,
    taste: String,
    finish: String,
    rating: u8
}

#[get("/")]
fn index() -> status::Accepted<String> {
    status::Accepted(Some(format!("Ok")))
}

#[get("/beers")]
fn get_beers() -> status::Accepted<String> {
    status::Accepted(Some(format!("Ok")))
}


fn main() {
    rocket::ignite().mount("/", routes![index, get_beers]).launch();
}
