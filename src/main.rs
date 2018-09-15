use futures::prelude::{Async, Poll};
use futures::task;
use futures::Future;

fn main() -> Result<(), ()> {
    let future = MyFuture {
        message: Option::None,
    };

    future
        .map(|message| println!("{}", message.unwrap()))
        .wait()
}

struct MyFuture {
    message: Option<&'static str>,
}

impl Future for MyFuture {
    type Item = Option<&'static str>;

    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        println!("knock knock, who is polling?");
        match &mut self.message {
            Some(message) => Ok(Async::Ready(Some(message))),
            None => {
                self.message = Some("Finally I am ready!");
                task::current().notify();
                Ok(Async::NotReady)
            }
        }
    }
}
