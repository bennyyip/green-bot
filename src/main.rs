extern crate json;
use json::*;

fn main() {
    let j = include_str!("../events.json");
    let m = json::parse(j).unwrap();
    let m = match m {
        JsonValue::Array(arr) => {
            arr.into_iter()
                .filter(|x| x["type"] == "PushEvent")
                .collect()
        }
        _ => vec![],
    };
    let email = "yebenmy@protonmail.com";

    for x in m {
        println!("{}", x["created_at"]);
        if let JsonValue::Array(ref commits) = x["payload"]["commits"] {
            println!("{}",
                     commits
                         .iter()
                         .filter(|commit| commit["author"]["email"] == email)
                         .count());
        }
    }
}
