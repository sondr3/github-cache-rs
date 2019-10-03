use crate::User;

use rocket_contrib::json::Json;

#[get("/")]
fn index() -> Json<User> {
    Json(user)
}

pub fn run() {
    rocket::ignite().mount("/", routes![index]).launch();
}
