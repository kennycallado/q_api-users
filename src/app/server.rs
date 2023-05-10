use rocket::fairing::AdHoc;

use crate::config::cors;
use crate::config::database;

use super::modules::routing as modules_routing;
use super::providers::interfaces::helpers::config_getter::ConfigGetter;
use super::routing as service_routing;

#[launch]
pub fn rocket() -> _ {
    let mut rocket_build = rocket::build();

    if ConfigGetter::get_migrations_run().unwrap_or(false) {
        println!("Using Rocket Database");
        rocket_build = rocket_build
            .attach(AdHoc::on_ignite(
                "Diesel Migrations",
                database::run_migrations,
            ))
            .attach(database::Db::fairing());
    }

    rocket_build
        .attach(service_routing::router())
        .attach(modules_routing::router())
        .attach(cors::Cors)
}
