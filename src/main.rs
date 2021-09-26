#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    set_port_if_provided();
    rocket::build()
        .mount("/", routes![index])
}

fn set_port_if_provided() {
    if let Ok(port) = std::env::var("PORT") {
        std::env::set_var("ROCKET_PORT", port);
        std::env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    }
}

