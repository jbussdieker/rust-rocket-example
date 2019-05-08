use super::rocket;
use rocket::local::Client;

#[test]
fn index() {
    let rocket = rocket::ignite().mount("/", routes![super::index]);
    let client = Client::new(rocket).unwrap();
    let mut response = client.get("/").dispatch();
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}
