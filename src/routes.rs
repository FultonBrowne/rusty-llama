
use crate::models::GenerateIngest;
use rocket::serde::json::Json;

#[get("/")]
pub fn ping() -> &'static str {
    ""
}

#[post("/generate", format="json", data="<data>")]
pub fn generate(data: Json<GenerateIngest>) {

}

