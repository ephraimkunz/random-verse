#[macro_use]
extern crate rocket;

use rocket::serde::{self, json::Json};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use time_tz::{timezones, OffsetDateTimeExt};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
struct Verse {
    volume_title: String,
    book_title: String,
    book_short_title: String,
    chapter_number: usize,
    verse_number: usize,
    verse_title: String,
    verse_short_title: String,
    scripture_text: String,
}

const SCRIPTURES_JSON_TEXT: &str = include_str!("../data/lds-scriptures-json.txt");

#[get("/")]
fn index() -> Json<Verse> {
    let now_utc = time::OffsetDateTime::now_utc();
    let west_coast = timezones::db::america::LOS_ANGELES;
    let now_local = now_utc.to_timezone(west_coast);

    let mut hasher = DefaultHasher::new();
    now_local.date().hash(&mut hasher);
    let hash = hasher.finish();

    let verses: Vec<Verse> = serde_json::from_str(SCRIPTURES_JSON_TEXT)
        .expect("Unable to deserialize scriptures from JSON");

    Json(verses[(hash % verses.len() as u64) as usize].clone())
}

#[shuttle_runtime::main]
async fn init() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
