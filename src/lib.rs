#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub mod decision;

use self::models::{NewPlan, Plan};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_plan(conn: &PgConnection, schedule_id: &i32) -> Plan {
    use self::schema::t_japan_post_shipment_plan;

    let new_plan = NewPlan {
        schedule_id: schedule_id,
    };

    diesel::insert_into(t_japan_post_shipment_plan::table)
        .values(&new_plan)
        .get_result(conn)
        .expect("Error saving new plan")
}