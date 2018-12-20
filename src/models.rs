use super::schema::t_japan_post_shipment_plan;

#[derive(Insertable)]
#[table_name = "t_japan_post_shipment_plan"]
pub struct NewPlan<'a> {
    pub schedule_id: &'a i32,
    pub deadline_date: &'a chrono::NaiveDateTime,
}

#[derive(Queryable, Debug)]
pub struct Plan {
    pub id: i32,
    pub schedule_id: i32,
    pub deadline_date: chrono::NaiveDateTime,
}
