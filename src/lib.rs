#![deny(warnings)]
use futures::future::{err, ok};
use futures::prelude::Future;
use std::io::{stdin, Error};

pub fn prompt(question: &str) -> Box<Future<Item = String, Error = Error>> {
    println!("{}", question);

    let mut buf = String::new();

    match stdin().read_line(&mut buf) {
        Ok(_) => Box::new(ok(buf)),
        Err(e) => Box::new(err(e)),
    }
}
