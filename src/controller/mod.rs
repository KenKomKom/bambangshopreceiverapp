use rocket::fairing::AdHoc;

pub mod notificiation;

pub fn route_stage() -> AdHoc {
    return AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
        rocket
            .mount("/", routes![notificiation::subscribe, notificiation::unsubscribe, notificiation::receive, notificiation::list])
    });
}
