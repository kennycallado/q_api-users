#[cfg(feature = "db")]
use rocket::fairing::AdHoc;

use crate::config::cors;
#[cfg(feature = "db")]
use crate::config::database;

use super::providers::interfaces::helpers::config_getter::ConfigGetter;
use super::providers::interfaces::helpers::cron::CronManager;
use super::providers::interfaces::helpers::fetch::Fetch;

use super::modules::routing as modules_routing;
use super::routing as service_routing;

#[launch]
pub async fn rocket() -> _ {
    let mut rocket_build = rocket::build();

    // Only attach the database if migrations_run is true
    // if ConfigGetter::get_migrations_run().unwrap_or(false) {
    #[cfg(feature = "db")]
    {
        rocket_build = rocket_build
            .attach(database::Db::fairing())
            .attach(AdHoc::on_ignite(
                "Diesel Migrations",
                database::run_migrations,
            ));
    }

    // Only manage the fetch if fetch is true
    if ConfigGetter::get_fetch().unwrap_or(false) {
        rocket_build = rocket_build.manage(Fetch::new());
    }

    // Only manage the cron if cron is true
    if ConfigGetter::get_cron().unwrap_or(false) {
        rocket_build = rocket_build.manage(CronManager::new().await);
    }

    rocket_build
        .attach(cors::Cors)
        .attach(service_routing::router())
        .attach(modules_routing::router())
}
