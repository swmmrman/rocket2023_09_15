#[macro_use] extern crate rocket;
use rocket::http::{CookieJar,Cookie};

#[get("/")]
fn index() -> &'static str {
    "Home page goes here."
}

#[get("/index.html")]
fn index2() -> &'static str {
    index()
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello world"
}

#[get("/get_cookie")]
fn get_cookie(jar: &CookieJar<'_>) -> &'static str{
    jar.add(Cookie::new("Key", "A Value"));
    "Cookie Set"
}

#[get("/check_cookie")]
fn check_cookie(jar: &CookieJar<'_>) -> Option<String>{
    jar.remove(Cookie::named("Key"));
    jar.get("Key").map(|val| format!("The result was {}", val))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index2,index,hello,get_cookie,check_cookie])
}