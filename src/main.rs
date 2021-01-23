use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use warp::Filter;

use std::collections::HashMap;

// From https://github.com/nightscout/cgm-remote-monitor/blob/master/swagger.yaml
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Entry {
    #[serde(rename = "type")]
    type_: String,
    date_string: String,
    date: i64,
    sgv: f64,
    direction: String,
    noise: f64,
    filtered: f64,
    unfiltered: f64,
    rssi: f64,
}

const DB_FILE: &str = "./cgmserver.sqlite";
const PORT: u16 = 8004;
const RECENT_MINUTES: i64 = 45;

#[tokio::main]
async fn main() {
    let connection = Connection::open(DB_FILE).unwrap();

    let _ = connection.execute_batch(include_str!("init.sql"));

    let entries_post = warp::post()
        .and(warp::path!("api" / "v1" / "entries"))
        .and(warp::body::json())
        .map(|body: Vec<Entry>| {
            // lol just open the db every request why not
            if let Ok(connection) = Connection::open(DB_FILE) {
                for e in body {
                    let q = connection.execute_named(
                        include_str!("insert_entry.sql"),
                        &[
                            (":type_", &e.type_),
                            (":date_string", &e.date_string),
                            (":date_", &e.date),
                            (":sgv", &e.sgv),
                            (":direction", &e.direction),
                            (":noise", &e.noise),
                            (":filtered", &e.filtered),
                            (":unfiltered", &e.unfiltered),
                            (":rssi", &e.rssi),
                        ],
                    );
                    if q.is_err() {
                        println!("{:?}", q);
                        return http::StatusCode::BAD_REQUEST;
                    };
                }
                http::StatusCode::OK
            } else {
                println!("error opening sqlite db");
                http::StatusCode::BAD_REQUEST
            }
        });

    let recent_get = warp::get()
        .and(warp::path("recent"))
        .map(|| -> Box<dyn warp::Reply> {
            // lol just open the db every request why not
            if let Ok(connection) = Connection::open(DB_FILE) {
                let mut recents: HashMap<i64, Entry> = HashMap::new();
                let mut q = connection.prepare(include_str!("get_recent.sql")).unwrap();
                for recent in q
                    .query_map(rusqlite::params![&RECENT_MINUTES], |row| {
                        Ok(Entry {
                            type_: row.get("type_")?,
                            date_string: row.get("date_string")?,
                            date: row.get("date_")?,
                            sgv: row.get("sgv")?,
                            direction: row.get("direction")?,
                            noise: row.get("noise")?,
                            filtered: row.get("filtered")?,
                            unfiltered: row.get("unfiltered")?,
                            rssi: row.get("rssi")?,
                        })
                    })
                    .unwrap()
                {
                    if let Ok(recent) = recent {
                        recents.insert(recent.date, recent);
                    }
                }
                Box::new(warp::reply::json(&recents))
            } else {
                Box::new(http::StatusCode::INTERNAL_SERVER_ERROR)
            }
        });

    let paths = entries_post.or(recent_get);

    warp::serve(paths).run(([0, 0, 0, 0], PORT)).await;
}
