extern crate diesel;
extern crate rust_sample_api;

use self::diesel::prelude::RunQueryDsl;
use self::models::Plan;
use self::rust_sample_api::{establish_connection, models};

fn main() {
    use rust_sample_api::schema::t_japan_post_shipment_plan::dsl::t_japan_post_shipment_plan;

    let connection = establish_connection();
    let results = t_japan_post_shipment_plan
        .load::<Plan>(&connection)
        .expect("Error loading t_japan_post_shipment_plan");

    println!("Displaying {} t_japan_post_shipment_plan", results.len());
    for plan in results {
        println!("{:?}", plan);
    }
}
