use rocket::response::stream::TextStream;
use crate::models::GenerateIngest;
use rocket::serde::json::Json;
use rocket::serde::json::json;
use rocket::tokio::time::{self, Duration};

#[get("/")]
pub fn ping() -> &'static str {
    ""
}

#[post("/generate", format="json", data="<data>")]
pub fn generate(data: Json<GenerateIngest>) -> TextStream![String] {
    println!("Data received as: {}", data.model);
    TextStream! {
        let mut interval = time::interval(Duration::from_secs(1));
        let mut i = 0;
        loop {
            interval.tick().await;
            let message = json!({ "message": "hello", "count": i }).to_string() + "\n";
            yield message;
            if i == 5 {
                let goodbye_message = json!({ "message": "goodbye" }).to_string() + "\n";
                yield goodbye_message;
                break;
            }
            i += 1;
        }
    }
}

