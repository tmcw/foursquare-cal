use std::fs::File;

use chrono::{Duration, NaiveDateTime};
use clap::{arg, command};
use icalendar::{Calendar, Class, Component, Event, EventLike, Property};
use serde::Deserialize;
use std::io::prelude::*;

#[derive(Debug, Deserialize)]
struct Foursquare {
    items: Vec<Checkin>,
}

#[derive(Debug, Deserialize)]
struct Venue {
    name: String,
    url: String,
    id: String,
}

#[derive(Debug, Deserialize)]
struct Checkin {
    timeZoneOffset: i64,
    venue: Venue,
    createdAt: String,
}

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([files] ... "File inputs"))
        .get_matches();

    let files_opt = matches.get_many::<String>("files");

    if files_opt.is_none() {
        println!("No files given. Usage: foursquare-cal data/checkins1.json …");
        return;
    }

    let files = files_opt.unwrap().map(|s| s.as_str());

    let mut my_calendar = Calendar::new();
    my_calendar.name("Foursquare");

    for filename in files {
        let file = File::open(filename).unwrap();
        let json: Foursquare = serde_json::from_reader(file).unwrap();

        for checkin in json.items {
            let time = NaiveDateTime::parse_from_str(&checkin.createdAt, "%Y-%m-%d %H:%M:%S.%f")
                .map(|n| n + Duration::minutes(checkin.timeZoneOffset))
                .unwrap();

            my_calendar.push(
                Event::new()
                    .summary(&format!("Checked in at {:?}", checkin.venue.name))
                    .location(&checkin.venue.name)
                    .add_property("COLOR", "#D98E15")
                    .add_property("URL", &checkin.venue.url)
                    .description("Foursquare checkin")
                    .starts(time)
                    .class(Class::Confidential)
                    .ends(time + Duration::hours(1))
                    .done(),
            );
        }
    }

    println!("Writing {:?} events to checkins.ics", my_calendar.len());

    let mut cal = File::create("checkins.ics").unwrap();
    cal.write_all(my_calendar.done().to_string().as_bytes())
        .unwrap();
}
