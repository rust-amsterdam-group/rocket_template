#[macro_use]
extern crate rocket;

# [get("/")]
asyhasync fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    set_cgr_config_if_required();
    rocket::build()
        .mount("/", routes![index])
}

fn set_cgr_config_if_required() {
    if let Ok(port) = std::env::var("PORT") {
        std::env::set_var("ROCKET_PORT", port);
        std::env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    }
}

