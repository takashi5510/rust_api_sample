extern crate rust_sample_api;

use self::rust_sample_api::decision as d;
//use self::decision::*;
//use self::decision;
//use self::decision as d;

fn main() {
    let results = d::select().expect("Error loading t_japan_post_shipment_decision"); // TODO: expect not use. pattern match switch

    println!(
        "Displaying {} t_japan_post_shipment_decision",
        results.len()
    );
    for decision in results {
        println!("{:?}", decision);
    }
}
