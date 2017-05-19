use chrono::prelude::*;
use reqwest;
use errors::*;
use std::io::Read;
use json::JsonValue;
use json;

pub struct User {
    username: String,
    email: String,
    timezone: FixedOffset,
}

lazy_static! {
    static ref CLIENT: reqwest::Client =  reqwest::Client::new().expect("cannot create reqwest client");
    static ref NOW:DateTime<UTC>= UTC::now();
}

impl User {
    pub fn new(username: &str, email: &str, timezone: i32, is_east: bool) -> Self {
        let timezone = timezone * 3600 * if is_east { 1 } else { -1 };

        Self {
            username: username.to_owned(),
            email: email.to_owned(),
            timezone: FixedOffset::east(timezone),
        }
    }
    fn get_events(&self) -> Result<JsonValue> {
        let mut j = String::new();
        CLIENT
            .get(&format!("https://api.github.com/users/{}/events", self.username))
            .send()?
            .read_to_string(&mut j)?;
        json::parse(&j).chain_err(|| "filed to parse events json")
    }
    pub fn today_has_push(&self) -> Result<bool> {
        let events = self.get_events()?;
        if let JsonValue::Array(events) = events {
            for x in events.iter().filter(|x| x["type"] == "PushEvent") {
                let push_date = x["created_at"]
                    .as_str()
                    .unwrap()
                    .parse::<DateTime<UTC>>()?
                    .with_timezone(&self.timezone);
                if is_same_day(&push_date, &NOW.with_timezone(&self.timezone)) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }
    fn is_my_commit(&self, commit: &JsonValue) -> bool {
        commit["author"]["email"] == self.email
    }
}

fn is_same_day<Tz: TimeZone>(lhs: &DateTime<Tz>, rhs: &DateTime<Tz>) -> bool {
    lhs.year() == rhs.year() && lhs.month() == rhs.month() && lhs.day() == rhs.day()
}
