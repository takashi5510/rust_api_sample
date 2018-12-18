extern crate rust_sample_api;
extern crate diesel;

use self::rust_sample_api::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {
    use rust_sample_api::schema::t_japan_post_shipment_plan::dsl::*;
 
    let connection = establish_connection();
    let results = t_japan_post_shipment_plan
        .load::<Plan>(&connection)
        .expect("Error loading t_japan_post_shipment_plan");

    println!("Displaying {} t_japan_post_shipment_plan", results.len());
    for plan in results {
        println!("{:?}", plan);
    }
}