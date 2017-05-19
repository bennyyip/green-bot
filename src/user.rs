use chrono::prelude::*;
use reqwest;
use errors::*;
use std::io::Read;
use std::str::FromStr;
use json::JsonValue;
use json;

pub struct User {
    username: String,
    email: String,
    timezone: FixedOffset,
}

lazy_static! {
    static ref client: reqwest::Client =  reqwest::Client::new().expect("cannot create reqwest client");
    static ref now:DateTime<UTC>= UTC::now();
}
static GITHUB_API: &'static str = "https://api.github.com/users/{}/events";

impl User {
    pub fn new(username: &str, email: &str, timezone: i32, is_east: bool) -> Self {
        let timezone = timezone * 3600 * if is_east { 1 } else { -1 };

        Self {
            username: username.to_owned(),
            email: email.to_owned(),
            timezone: FixedOffset::east(timezone),
        }
    }
    fn is_my_commit(&self, commit: &JsonValue) -> bool {
        commit["author"]["email"] == self.email
    }
    fn get_events(&self) -> Result<JsonValue> {
        let mut j = String::new();
        client
            .get(&format!("https://api.github.com/users/{}/events", self.username))
            .send()?
            .read_to_string(&mut j)?;
        json::parse(&j).chain_err(|| "filed to parse events json")
    }
    pub fn today_has_push(&self) -> Result<bool> {
        let events = self.get_events()?;
        let events = match events {
            JsonValue::Array(arr) => {
                arr.into_iter()
                    .filter(|x| x["type"] == "PushEvent")
                    .collect()
            }
            _ => vec![],
        };
        for x in events {
            let push_date = x["created_at"]
                .as_str()
                .unwrap()
                .parse::<DateTime<UTC>>()?
                .with_timezone(&self.timezone);
            if is_same_day(&push_date, &now.with_timezone(&self.timezone)) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

fn is_same_day<Tz: TimeZone>(lhs: &DateTime<Tz>, rhs: &DateTime<Tz>) -> bool {
    lhs.year() == rhs.year() && lhs.month() == rhs.month() && lhs.day() == rhs.day()
}
