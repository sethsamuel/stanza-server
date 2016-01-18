extern crate iron;
extern crate rustc_serialize;
extern crate hyper;

use std::io::Read;

use iron::prelude::*;
use iron::status;

// use rustc_serialize::json;

use hyper::Client;
use hyper::header::UserAgent;

#[derive(RustcDecodable, RustcEncodable)]
struct Echo {
	message: String
}

fn main() {
	fn hello_world(_: &mut Request) -> IronResult<Response> {
		// let echo = Echo { message: "Hello!".to_string() };
		// let payload = json::encode(&echo).unwrap();
		let client = Client::new();

		let mut res = client.get("https://api.github.com/search/users?q=sethsamuel")
			.header(UserAgent("Stanza Server".to_owned()))
			.send()
			.unwrap();

		let mut body = String::new();
		res.read_to_string(&mut body).unwrap();

		Ok(Response::with((status::Ok, body)))
	}

	let port = 3003;
	let host = format!("0.0.0.0:{}", port);
	println!("Running on port {}", port);
	Iron::new(hello_world).http(&*host).unwrap();
}