#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Home page goes here."
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello world"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index,hello])
}