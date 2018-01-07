extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use futures::Future;
use hyper::{Client, Method, Request};
use hyper::header::ContentType;
use tokio_core::reactor::Core;

fn main() {
    run()
}

fn run() {
    let mut core = Core::new().expect("could not create core");
    let client = Client::configure()
        .connector(::hyper_tls::HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());
    let uri = "https://api.pushover.net/1/messages.json".parse().unwrap();
    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(ContentType::json());
    //let mut body = req.body();
    let work = client
        .request(req)
        .map(|res| {
            println!("POST: {}", res.status());
            println!("Headers: \n{}", res.headers())
        })
        .map_err(|err| panic!("Error: {:?}", err));

    core.run(work).expect("Could not run tokio core");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
