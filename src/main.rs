#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate json;
extern crate reqwest;
use json::*;
use chrono::prelude::*;
mod user;
mod errors {
    error_chain!{
        foreign_links {
            IO(::std::io::Error);
            Net(::reqwest::Error);
            Json(::json::Error);
            Chrono(::chrono::ParseError);
        }
    }
}

fn main() {
    let username = "bennyyip";
    let email = "yebenmy@protonmail.com";

    let ben = user::User::new(username, email, 8, true);
    println!("{}", ben.today_has_push().unwrap());
}
