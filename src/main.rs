#![license = "MIT"]
#![deny(missing_doc)]
#![deny(unused_result, unused_result, unnecessary_qualification,
        non_camel_case_types, unused_variable, unnecessary_typecast)]

//! A seed project for the [Iron](https://github.com/iron/iron) web framework.

extern crate iron;

use std::io::net::ip::Ipv4Addr;
use iron::{Iron, ServerT, Request, Response, Ingot, Alloy};
use iron::ingot::{Status, Continue, Unwind};

#[deriving(Clone)]
// Create your own Ingot.
struct HelloWorld;

impl HelloWorld {
    fn new() -> HelloWorld { HelloWorld }
}

impl<Rq: Request, Rs: Response> Ingot<Rq, Rs> for HelloWorld {
    // This defines the ingot's behavior when it first sees the Request.
    fn enter(&mut self, _req: &mut Rq, res: &mut Rs, _alloy: &mut Alloy) -> Status {
        match res.write(bytes!("Hello World!")) {
            Ok(_) => {
                println!("Wrote: Hello World!");
                Continue
            },
            Err(err) => {
                println!("Error: {}", err);
                Unwind
            }
        }
    }
}

fn main() {
    let mut server: ServerT = Iron::new();
    // Add the ingot to your server.
    server.smelt(HelloWorld::new());
    server.listen(Ipv4Addr(127, 0, 0, 1), 3000);
}
