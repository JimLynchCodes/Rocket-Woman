#![feature(proc_macro_hygiene, decl_macro)]
use rocket::response::content;
use rocket::response::status;
use rocket::http::Status;

#[macro_use] extern crate rocket;

// struct Foo {
//     foo: String
// }

// #[get("/ok")]
// fn new_ok() -> Status {
//     // status::Accepted(Some(format!("id: '{}'", "foo")))

//     let response = Status::new(202, "Somewhat Successful");

//     response
// }

// #[get("/")]
// fn json() -> content::Json<&'static str> {
//     content::Json("{\n\t'hi': 'world'\n}")
// }

#[get("/s")]
fn new_s() -> status::Custom<content::Json<&'static str>> {
    // status::Created(Some(format!("id: '{}'", "foo")))
    // status::BadRequest("".to_string())

    let response_payload = content::Json("{\n\t'hi': 'world'\n}");

    let response = status::Custom(Status::ImATeapot, response_payload);

    response
}

// #[get("/")]
// fn json() -> content::Json<&'static str> {
//     content::Json("{\n\t'hi': 'world'\n}")
// }

// #[get("/")]
// fn index() -> &'static Foo {
    // Foo {
    //     "foo": "Hello, world!"
    // }
// }

fn main() {
    // rocket::ignite().mount("/s", routes![new_s]);
    rocket::ignite()
        // .mount("/", routes![json])
        .mount("/", routes![new_s])
        // .mount("/", routes![new_ok])
        .launch();
}