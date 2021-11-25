use reqwest::Client;
use rocket::State;

#[get("/?<query>")]
pub async fn search(query: &str, client: &State<Client>) -> &'static str {
    client
        .get(format!("https://en.wikipedia.org/wiki/{}", query))
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_or("oh snap!", |_| "success")
}
