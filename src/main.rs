#[macro_use]
extern crate rocket;

mod routes;


#[launch]
fn rocket() -> _ {
    set_cgr_config_if_required();
    rocket::build()
        .mount("/", routes![routes::index])
        .mount("/", routes![routes::search])
        .manage(reqwest::Client::new())
}

fn set_cgr_config_if_required() {
    if let Ok(port) = std::env::var("PORT") {
        std::env::set_var("ROCKET_PORT", port);
        std::env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    }
}

