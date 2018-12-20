use diesel::prelude::*;

table! {
    t_japan_post_shipment_decision (id) {
        id -> Integer,
        schedule_id -> Integer,
    }
}

#[derive(Insertable)]
#[table_name = "t_japan_post_shipment_decision"]
pub struct NewDecision<'a> {
    pub schedule_id: &'a i32,
}

#[derive(Queryable, Debug)]
pub struct Decision {
    pub id: i32,
    pub schedule_id: i32,
}

impl Decision {
    pub fn find_all() -> Result<Vec<Decision>, diesel::result::Error> {
        use self::t_japan_post_shipment_decision::dsl::*;
        let conn = super::establish_connection();
        t_japan_post_shipment_decision.load::<Decision>(&conn)
    }
}

#[test]
pub fn find_all() {
    println!("{:?}", Decision::find_all());
}
