use futures::future::{ok, Future};
use questionnaire::prompt;

fn main() {
    prompt("What is your name")
        .and_then(|name| {
            println!("Ho your name is {}", name);
            ok(())
        })
        .wait();
}
