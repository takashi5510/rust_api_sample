extern crate rust_sample_api;
extern crate diesel;

use self::rust_sample_api::*;
//use self::decision::*;
//use self::decision;
use self::decision as d;

fn main() {
 
    let results = d::select()
        .expect("Error loading t_japan_post_shipment_decision"); // TODO: expect not use. pattern match switch

    println!("Displaying {} t_japan_post_shipment_decision", results.len());
    for decision in results {
        println!("{:?}", decision);
    }

}