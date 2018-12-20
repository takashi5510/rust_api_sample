#[macro_use]
extern crate diesel;
extern crate chrono;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod decision;
pub mod models;
pub mod schema;
use self::models::{NewPlan, Plan};
use chrono::NaiveDateTime;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_plan(conn: &PgConnection, schedule_id: &i32, deadline_date: &NaiveDateTime) -> Plan {
    use self::schema::t_japan_post_shipment_plan;

    let new_plan = NewPlan {
        schedule_id: schedule_id,
        deadline_date: deadline_date,
    };

    diesel::insert_into(t_japan_post_shipment_plan::table)
        .values(&new_plan)
        .get_result(conn)
        .expect("Error saving new plan")
}
