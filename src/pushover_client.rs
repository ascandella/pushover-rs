use url::form_urlencoded;
use std::io::{self, Error, ErrorKind};
use hyper::{self, Client, Method, Request};
use tokio_core::reactor::Core;
use futures::{future, Future, Stream};
use hyper_tls::HttpsConnector;

type HttpsClient = Client<HttpsConnector<hyper::client::HttpConnector>>;

pub struct PushoverClient<'a> {
  core: Core,
  client: HttpsClient,
  key: &'a String,
}

impl<'a> PushoverClient<'a> {
  pub fn from(key: &'a String) -> Option<Self> {
    let core = match Core::new() {
      Ok(c) => c,
      Err(_) => return None,
    };

    let client = Client::configure()
      .connector(HttpsConnector::new(4, &core.handle()).unwrap())
      .build(&core.handle());

    Some(PushoverClient {
      core: core,
      client: client,
      key: &key,
    })
  }

  pub fn push(&mut self, user: &String, message: &String) -> io::Result<()> {
    let uri = match "https://api.pushover.net/1/messages.json".parse() {
      Ok(uri) => uri,
      Err(err) => return Err(Error::new(ErrorKind::Other, err)),
    };

    let mut req = Request::new(Method::Post, uri);
    let body = form_urlencoded::Serializer::new(String::new())
      .append_pair("user", user)
      .append_pair("token", self.key)
      .append_pair("message", message)
      .finish();

    req.set_body(body);
    let work = self
      .client
      .request(req)
      .map(|res| {
        println!("POST: {}", res.status());
        println!("Headers: \n{}", res.headers());
        println!("Body:");
        res
          .body()
          .fold(Vec::new(), |mut v, chunk| {
            v.extend(&chunk[..]);
            future::ok::<_, hyper::Error>(v)
          })
          .and_then(|chunks| {
            let s = String::from_utf8(chunks).unwrap();
            future::ok::<_, hyper::Error>(s)
          })
      })
      .map_err(|err| Error::new(ErrorKind::Other, err));

    match self.core.run(work) {
      Ok(body_work) => {
        if let Ok(resp_body) = self.core.run(body_work) {
          println!("Body: {}", resp_body);
        }
      }
      Err(err) => return Err(err),
    }

    Ok(())
  }
}
