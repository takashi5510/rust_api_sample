use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use diesel::prelude::*;

table! {
    t_japan_post_shipment_decision (id) {
        id -> Integer,
        schedule_id -> Integer,
        set_pattern_id -> Integer,
        shipment_date -> Timestamp,
        item_count -> Integer,
        package_weight -> Nullable<Integer>,
        package_length -> Nullable<Integer>,
        package_width -> Nullable<Integer>,
        package_thickness -> Nullable<Integer>,
        product_id -> Integer,
        product_name -> Nullable<Text>,
        release_date -> Timestamp,
        status_code -> Integer,
        active -> Integer,
        created_date -> Timestamp,
        updated_date -> Timestamp,
        removed_date -> Nullable<Timestamp>,
    }
}

#[derive(Insertable)]
#[table_name = "t_japan_post_shipment_decision"]
pub struct NewDecision {
    pub schedule_id: i32,
    pub set_pattern_id: i32,
    pub shipment_date: NaiveDateTime,
    pub item_count: i32,
    pub package_weight: Option<i32>,
    pub package_length: Option<i32>,
    pub package_width: Option<i32>,
    pub package_thickness: Option<i32>,
    pub product_id: i32,
    pub product_name: Option<String>,
    pub release_date: NaiveDateTime,
    pub status_code: i32,
    pub active: i32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub removed_date: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct Decision {
    pub id: i32,
    pub schedule_id: i32,
    pub set_pattern_id: i32,
    pub shipment_date: NaiveDateTime,
    pub item_count: i32,
    pub package_weight: Option<i32>,
    pub package_length: Option<i32>,
    pub package_width: Option<i32>,
    pub package_thickness: Option<i32>,
    pub product_id: i32,
    pub product_name: Option<String>,
    pub release_date: NaiveDateTime,
    pub status_code: i32,
    pub active: i32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub removed_date: Option<NaiveDateTime>,
}

impl Decision {
    pub fn find_all() -> Result<Vec<Decision>, diesel::result::Error> {
        use self::t_japan_post_shipment_decision::dsl::*;
        let conn = super::establish_connection();
        t_japan_post_shipment_decision.load::<Decision>(&conn)
    }
    pub fn find_by_id(target_id: i32) -> Result<Decision, diesel::result::Error> {
        use self::t_japan_post_shipment_decision::columns::*;
        use self::t_japan_post_shipment_decision::dsl::t_japan_post_shipment_decision;
        let conn = super::establish_connection();
        t_japan_post_shipment_decision
            .filter(id.eq(&target_id))
            .first::<Decision>(&conn)
    }
    pub fn delete(target_id: i32) -> Result<usize, diesel::result::Error> {
        use self::t_japan_post_shipment_decision::columns::*;
        use self::t_japan_post_shipment_decision::dsl::t_japan_post_shipment_decision;
        let conn = super::establish_connection();
        diesel::delete(t_japan_post_shipment_decision.filter(id.eq(&target_id))).execute(&conn)
    }
}

impl NewDecision {
    pub fn save(self) -> Result<Decision, diesel::result::Error> {
        let conn = super::establish_connection();
        diesel::insert_into(t_japan_post_shipment_decision::table)
            .values(self)
            .get_result(&conn)
    }
}

#[test]
pub fn find_all() {
    println!("{:?}", Decision::find_all().unwrap());
}

#[test]
pub fn find_by_id() {
    println!("{:?}", Decision::find_by_id(5).unwrap());
}

#[test]
pub fn save_and_delete() {
    let new_decision = NewDecision {
        schedule_id: 1234,
        set_pattern_id: 4567,
        shipment_date: NaiveDate::from_ymd(2016, 7, 18).and_hms(9, 10, 11),
        item_count: 999,
        package_weight: Some(123),
        package_length: Some(234),
        package_width: Some(345),
        package_thickness: Some(456),
        product_id: 98765,
        product_name: Some("テスト".to_string()),
        release_date: NaiveDate::from_ymd(2016, 7, 20).and_hms(9, 10, 11),
        status_code: 0,
        active: 1,
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
        removed_date: None,
    };
    match new_decision.save() {
        Ok(target_decision) => {
            let rs = Decision::delete(target_decision.id);
            println!("{:?}", rs.unwrap())
        }
        Err(e) => println!("{:?}", e),
    }
    /*
        if let Ok(target_plan) = new_plan.save() {
            let rs = Plan::delete(target_plan.id);
            println!("{:?},{:?}", target_plan, rs.unwrap());
        };
    */
}
