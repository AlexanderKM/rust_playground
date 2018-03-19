#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate rocket;
#[macro_use] extern crate serde_derive;
extern crate rocket_contrib;

use rocket_contrib::Json;

#[derive(Serialize, Deserialize)]
struct Concert {
    band: String,
    month: String,
    day: u16,
    hour: u16
}

#[post("/concerts", format = "application/json", data = "<concert>")]
fn new_concert(concert: Json<Concert>) -> Json<Concert> {
    concert
}

#[get("/hello/<name>/<age>/<cool>")]
fn world(name: String, age: u8, cool: bool) -> String {
        if cool {
            format!("You're a cool {} year old, {}! 😎 ", age, name)
        } else {
            format!("You're less cool, {}", name)
        }
}

fn main() {
        rocket::ignite().mount("/", routes![world, new_concert]).launch();
}

