use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use diesel::prelude::*;

table! {
    t_japan_post_shipment_plan (id) {
        id -> Integer,
        schedule_id -> Integer,
        deadline_date -> Timestamp,
        shipment_date -> Timestamp,
        item_count -> Integer,
        total_weight -> Nullable<Integer>,
        bucket_count -> Nullable<Integer>,
        product_id -> Integer,
        product_name -> Nullable<Text>,
        release_date -> Timestamp,
        active -> Integer,
        created_date -> Timestamp,
        updated_date -> Timestamp,
        removed_date -> Nullable<Timestamp>,
    }
}

#[derive(Insertable)]
#[table_name = "t_japan_post_shipment_plan"]
pub struct NewPlan {
    pub schedule_id: i32,
    pub deadline_date: NaiveDateTime,
    pub shipment_date: NaiveDateTime,
    pub item_count: i32,
    pub total_weight: Option<i32>,
    pub bucket_count: Option<i32>,
    pub product_id: i32,
    pub product_name: Option<String>,
    pub release_date: NaiveDateTime,
    pub active: i32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub removed_date: Option<NaiveDateTime>,
}

#[derive(Queryable, Debug)]
pub struct Plan {
    pub id: i32,
    pub schedule_id: i32,
    pub deadline_date: NaiveDateTime,
    pub shipment_date: NaiveDateTime,
    pub item_count: i32,
    pub total_weight: Option<i32>,
    pub bucket_count: Option<i32>,
    pub product_id: i32,
    pub product_name: Option<String>,
    pub release_date: NaiveDateTime,
    pub active: i32,
    pub created_date: NaiveDateTime,
    pub updated_date: NaiveDateTime,
    pub removed_date: Option<NaiveDateTime>,
}

impl Plan {
    pub fn find_all() -> Result<Vec<Plan>, diesel::result::Error> {
        use self::t_japan_post_shipment_plan::dsl::t_japan_post_shipment_plan;
        let conn = super::establish_connection();
        t_japan_post_shipment_plan.load::<Plan>(&conn)
    }
    pub fn find_by_id(target_id: i32) -> Result<Plan, diesel::result::Error> {
        use self::t_japan_post_shipment_plan::columns::*;
        use self::t_japan_post_shipment_plan::dsl::t_japan_post_shipment_plan;
        let conn = super::establish_connection();
        t_japan_post_shipment_plan
            .filter(id.eq(&target_id))
            .first::<Plan>(&conn)
    }
    pub fn delete(target_id: i32) -> Result<usize, diesel::result::Error> {
        use self::t_japan_post_shipment_plan::columns::*;
        use self::t_japan_post_shipment_plan::dsl::t_japan_post_shipment_plan;
        let conn = super::establish_connection();
        diesel::delete(t_japan_post_shipment_plan.filter(id.eq(&target_id))).execute(&conn)
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
pub fn find_by_id() {
    println!("{:?}", Plan::find_by_id(145).unwrap());
}

#[test]
pub fn save_and_delete() {
    let new_plan = NewPlan {
        schedule_id: 1234,
        deadline_date: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        shipment_date: NaiveDate::from_ymd(2016, 7, 18).and_hms(9, 10, 11),
        item_count: 999,
        total_weight: Some(2345),
        bucket_count: Some(5),
        product_id: 98765,
        product_name: Some("テスト".to_string()),
        release_date: NaiveDate::from_ymd(2016, 7, 20).and_hms(9, 10, 11),
        active: 1,
        created_date: Local::now().naive_local(),
        updated_date: Local::now().naive_local(),
        removed_date: None,
    };
    match (new_plan.save()) {
        Ok(target_plan) => {
            let rs = Plan::delete(target_plan.id);
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
