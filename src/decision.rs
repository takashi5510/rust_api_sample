#[derive(Insertable)]
#[table_name="t_japan_post_shipment_decision"]
pub struct NewDecision<'a> {
    pub schedule_id: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct Decision {
    pub id: i32,
    pub schedule_id: i32
}

// table! -> 
// pub mod t_japan_post_shipment_decision{
//   pub mod dsl {
//     id...
//   }
// }
table! {
    t_japan_post_shipment_decision (id) {
        id -> Integer,
        schedule_id -> Integer,
    }
}

pub fn select() -> std::result::Result<std::vec::Vec<Decision>, diesel::result::Error> {
    use super::*;
    use self::t_japan_post_shipment_decision::dsl::*;

    let connection = establish_connection();
    t_japan_post_shipment_decision.load::<Decision>(&connection)
//        .expect("Error loading t_japan_post_shipment_decision");
}