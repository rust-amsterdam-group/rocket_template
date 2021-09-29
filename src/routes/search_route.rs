use reqwest::Client;
use rocket::State;

#[get("/search?<query>")]
pub async fn search(query: &str, client: &State<Client>) -> &'static str {
    let response = client.get(format!("https://www.google.com/search?q={}", query))
        .send()
        .await
        .and_then(|r| r.error_for_status());

    return match response {
        Ok(_) => "success",
        Err(_) => "oh snap"
    };
}