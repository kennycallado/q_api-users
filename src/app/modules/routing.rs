use crate::app::modules::users::controller::routes as user_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket.mount("/api/v1/user", user_routes())
    })
}
