#[get("/")]
pub fn resource_access_index() -> &'static str {
    "Resource access index!"
}