
#![feature(plugin, custom_derive, const_fn, custom_attribute)]
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

mod status;
mod location_entry;
mod db;

// The URL to the database, set via the `DATABASE_URL` environment variable.
static DATABASE_URL: &'static str = env!("DATABASE_URL");
static VERSION: Option<&'static str> = option_env!("GIT_VERSION");

#[get("/users")]
fn users() -> &'static str {
    "Users"
}

fn main() {
    let mut loading_status = status::ServerStatus::new(VERSION);

    // TODO: Set this after everything has initialized. After iginite() and
    //       other boot checks..
    loading_status.set_status(status::ServerCondition::Running);

    rocket::ignite()
        .manage(db::init_db_pool(DATABASE_URL))
        .mount("/v1/", routes![status::get, status::get_as_txt, users])
        .manage(loading_status)
        .launch();
}
