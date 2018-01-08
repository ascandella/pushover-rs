extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate url;

mod pushover_client;
use pushover_client::PushoverClient;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next()
        .expect("pass pushover key as first positional arg");
    let user = args.next().expect("pass user key as second arg");
    let message = args.next().expect("pass message as third arg");

    let pc = PushoverClient::from(&key);
    match pc {
        Some(mut client) => {
            if let Err(err) = client.push(&user, &message) {
                panic!(err)
            }
        }
        None => panic!("could not create client"),
    }
}
