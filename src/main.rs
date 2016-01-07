extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
    	Ok(Response::with((status::Ok, "Hello!")))
    }

    let port = 3003;
    let host = format!("0.0.0.0:{}", port);
    Iron::new(hello_world).http(&*host).unwrap();
    println!("Running on port {}", port);
}