#![feature(proc_macro_hygiene, decl_macro, backtrace)]

use rocket::{get, routes, Request, Response};
use rocket_contrib::serve::StaticFiles;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

#[get("/name")]
fn get_name() -> String {
  "Brian Duff".to_owned()
}

// CORS fairing so we can access the api from a different port on localhost.
// This should be disabled in production.

pub struct CORS();

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response
        }
    }

    fn on_response(&self, _: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn main() {
  rocket::ignite()
    .attach(CORS())
    .mount("/api/", routes![get_name])
    .mount("/", StaticFiles::from("client/build"))
    .launch();
}
