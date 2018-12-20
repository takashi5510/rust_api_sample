use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;

table! {
    t_japan_post_shipment_plan (id) {
        id -> Integer,
        schedule_id -> Integer,
        deadline_date -> Timestamp,
    }
}

#[derive(Insertable)]
#[table_name = "t_japan_post_shipment_plan"]
pub struct NewPlan {
    pub schedule_id: i32,
    pub deadline_date: NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Plan {
    pub id: i32,
    pub schedule_id: i32,
    pub deadline_date: NaiveDateTime,
}

impl Plan {
    pub fn find_all() -> Result<Vec<Plan>, diesel::result::Error> {
        use self::t_japan_post_shipment_plan::dsl::t_japan_post_shipment_plan;
        let conn = super::establish_connection();
        t_japan_post_shipment_plan.load::<Plan>(&conn)
    }
}

impl NewPlan {
    pub fn save(self) -> Result<Plan, diesel::result::Error> {
        let conn = super::establish_connection();
        diesel::insert_into(t_japan_post_shipment_plan::table)
            .values(self)
            .get_result(&conn)
    }
}

#[test]
pub fn find_all() {
    println!("{:?}", Plan::find_all());
}

#[test]
pub fn save() {
    let new_plan = NewPlan {
        schedule_id: 1234,
        deadline_date: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
    };
    println!("{:?}", new_plan.save());
}
