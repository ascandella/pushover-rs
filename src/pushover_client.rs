use futures::{future, Future, Stream};
use hyper::Uri;
use hyper::{self, Client, Method, Request};
use hyper_tls::HttpsConnector;
use std::io::{self, Error, ErrorKind};
use tokio_core::reactor::Core;
use url::form_urlencoded;

type HttpsClient = Client<HttpsConnector<hyper::client::HttpConnector>>;

pub struct PushoverClient<'a> {
    core: Core,
    client: HttpsClient,
    key: &'a String,
    uri: Uri,
}

impl<'a> PushoverClient<'a> {
    pub fn from(key: &'a String) -> Option<Self> {
        let uri = match "https://api.pushover.net/1/messages.json".parse() {
            Ok(uri) => uri,
            Err(err) => {
                println!("Unable to parse pushover URI: {}", err);
                return None;
            }
        };

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
            uri: uri,
        })
    }

    fn make_body(&self, user: &String, message: &String) -> String {
        form_urlencoded::Serializer::new(String::new())
            .append_pair("user", user)
            .append_pair("token", self.key)
            .append_pair("message", message)
            .finish()
    }

    pub fn push(&mut self, user: &String, message: &String) -> io::Result<()> {
        let mut req = Request::new(Method::Post, self.uri.clone());
        req.set_body(self.make_body(&user, &message));

        let work = self
            .client
            .request(req)
            .map(|res| {
                res.body()
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
