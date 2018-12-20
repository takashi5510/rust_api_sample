extern crate diesel;
extern crate rust_sample_api;

use self::rust_sample_api::*;
use std::io::stdin;

use chrono::{NaiveDate, NaiveDateTime};

fn main() {
    let connection = establish_connection();

    println!("What would you like your schedule_id to be?");
    let mut std_in = String::new();
    stdin().read_line(&mut std_in).unwrap();
    let std_in = &std_in[..(std_in.len() - 1)]; // Drop the newline character
    let schedule_id: i32 = std_in.parse().unwrap();

    let deadline_date: NaiveDateTime = NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11);;
    //let deadline_date = Utc::now();

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        schedule_id, EOF
    );
    let plan = create_plan(&connection, &schedule_id, &deadline_date);
    println!("\nSaved draft {} with id {}", schedule_id, plan.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
