extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use futures::Future;
use hyper::{Client, Method, Request};
use hyper::header::ContentType;
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use std::io::{self, Error, ErrorKind};

type HttpsClient = Client<HttpsConnector<hyper::client::HttpConnector>>;

fn main() {
    let mut core = Core::new().expect("could not create core");
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    if let Err(err) = run(&mut core, &client) {
        panic!(err)
    }
}

fn run(core: &mut Core, client: &HttpsClient) -> io::Result<()> {
    let uri = match "https://api.pushover.net/1/messages.json".parse() {
        Ok(uri) => uri,
        Err(err) => return Err(Error::new(ErrorKind::Other, err)),
    };

    let mut req = Request::new(Method::Get, uri);
    req.headers_mut().set(ContentType::json());
    //let mut body = req.body();
    let work = client
        .request(req)
        .map(|res| {
            println!("POST: {}", res.status());
            println!("Headers: \n{}", res.headers())
        })
        .map_err(|err| Error::new(ErrorKind::Other, err));

    core.run(work)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
