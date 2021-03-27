#![feature(proc_macro_hygiene, decl_macro, backtrace)]

use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;

#[get("/hello")]
fn get_hello() -> String {
  "Hello World".to_owned()
}

fn main() {
  rocket::ignite()
    .mount("/api/", routes![get_hello])
    .mount("/", StaticFiles::from("client/build"))
    .launch();
}
