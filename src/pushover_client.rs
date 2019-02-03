use futures::{future, Future, Stream};
use hyper::http::uri::InvalidUri;
use hyper::Uri;
use hyper::{self, Body, Client, Request};
use hyper_tls::HttpsConnector;
use std::io::{self, Error, ErrorKind};
use tokio_core::reactor::Core;
use url::form_urlencoded;

#[derive(Debug)]
pub enum ClientError {
    Error(Error),
    HyperTlsError(hyper_tls::Error),
    UriError(InvalidUri),
}

impl From<Error> for ClientError {
    fn from(error: Error) -> Self {
        ClientError::Error(error)
    }
}

impl From<hyper_tls::Error> for ClientError {
    fn from(error: hyper_tls::Error) -> Self {
        ClientError::HyperTlsError(error)
    }
}

impl From<InvalidUri> for ClientError {
    fn from(error: InvalidUri) -> Self {
        ClientError::UriError(error)
    }
}

pub struct PushoverClient<'a> {
    core: Core,
    client: Client<HttpsConnector<hyper::client::HttpConnector>>,
    key: &'a str,
    uri: Uri,
}

impl<'a> PushoverClient<'a> {
    pub fn from(key: &'a str) -> Result<Self, ClientError> {
        let uri = "https://api.pushover.net/1/messages.json".parse()?;

        let core = Core::new()?;

        let https = HttpsConnector::new(4)?;
        let client = Client::builder().build(https);

        Ok(PushoverClient {
            core,
            client,
            key: &key,
            uri,
        })
    }

    fn make_body(&self, user: &str, message: &str) -> Body {
        let str_body = form_urlencoded::Serializer::new(String::new())
            .append_pair("user", user)
            .append_pair("token", self.key)
            .append_pair("message", message)
            .finish();

        Body::from(str_body)
    }

    pub fn push(&mut self, user: &str, message: &str) -> io::Result<()> {
        let req = Request::builder()
            .uri(self.uri.clone())
            .method("POST")
            .body(self.make_body(&user, &message))
            .unwrap();

        let work = self
            .client
            .request(req)
            .map(|res| {
                res.into_body()
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

        let body_work = self.core.run(work)?;
        if let Ok(resp_body) = self.core.run(body_work) {
            println!("{}", resp_body);
        }

        Ok(())
    }
}
