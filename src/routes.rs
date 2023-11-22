#[macro_use] extern crate rocket;

#[get("/")]
pub fn ping() -> &'static str {
    "Hello, world!"
}

#[post("/")]
pub fn generate() {

}
