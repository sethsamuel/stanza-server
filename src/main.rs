extern crate iron;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Echo {
	message: String
}

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
    	let echo = Echo { message: "Hello!".to_string() };
    	let payload = json::encode(&echo).unwrap();
    	Ok(Response::with((status::Ok, payload)))
    }

    let port = 3003;
    let host = format!("0.0.0.0:{}", port);
    Iron::new(hello_world).http(&*host).unwrap();
    println!("Running on port {}", port);
}