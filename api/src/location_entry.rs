
use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::types::Timestamp;


#[derive(Debug, Clone)]
pub struct Point {
    pub lat: f32,
    pub long: f32,
}

#[table_name = "location_entries"]
#[derive(Debug, Clone)]
pub struct LocationEntry {
    pub id: Option<i32>,
    pub checked_on: Timestamp,
    pub location: Point,
    pub by_user: Option<i32>,
}

impl LocationEntry {
    pub fn all(conn: &PgConnection) -> Vec<LocationEntry> {
        Vec::new()
    }

    pub fn insert(conn: &PgConnection, time: Timestamp, loc: Point, user_id: i32) -> bool {
        false
    }

    pub fn delete(conn: &PgConnection, id: i32) -> bool {
        false
    }

    pub fn update(conn: &PgConnection) -> bool {
        false
    }
}
