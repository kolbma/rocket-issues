//! A very simple server
//!
//! Instructions for use:
//! Run "cargo run --bin server" in one command window. The HTTP
//! server starts and waits for requests.
//! In another command window, run "cargo run --bin client". The client
//! should send one query to the server and print the outcome.
//!
//! The server runs in debug mode and prints useful diagnostics to the
//! terminal screen.

#![deny(clippy::all)]
#![deny(keyword_idents)]
#![deny(non_ascii_idents)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
// #![deny(unused_crate_dependencies)]
#![deny(unused_qualifications)]
// #![deny(unused_results)]
#![deny(warnings)]

use rocket::{get, launch, log::LogLevel, routes, Config};

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(Config {
            log_level: LogLevel::Debug,
            ..Config::default()
        })
        .mount("/hello", routes![world])
}
