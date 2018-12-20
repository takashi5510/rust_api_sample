#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;

fn main() {
    println!(
        "Hello, world! ,{:?}",
        db::jp_shipment::plan::Plan::find_all()
    );
}
