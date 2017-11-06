
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2_diesel;
extern crate r2d2;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use r2d2_diesel::ConnectionManager;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use std::env;
use std::ops::Deref;

mod status;

// Database connection pool
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");
static VERSION: &'static str = env!("GIT_VERSION");


// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &DbConn as an &PgConnection.
impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[get("/users")]
fn users() -> &'static str {
    "Users"
}

fn init_db_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
    r2d2::Pool::new(config, manager).expect("db pool")
}

fn main() {
    let mut loading_status = status::ServerStatus::new(VERSION.to_owned());
    loading_status.set_status(status::ServerCondition::Running);

    rocket::ignite()
       // .manage(init_db_pool())
        .mount("/", routes![status::get])
        .manage(loading_status)
        .launch();
}
