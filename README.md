# pushover-rs

My first time using Rust. I opted to use Hyper+Tokio instead of the
much-friendlier-looking [Reqwest](https://docs.rs/reqwest) library so I could
get a feel for lower-level networking and futures in Rust.

## Usage

Compile-and-run:

```sh
cargo run -- $PUSHOVER_TOKEN $PUSHOVER_USER $MESSAGE
```

Build, then run:

```sh
cargo build
./target/debug/pushover $PUSHOVER_TOKEN $PUSHOVER_USER $MESSAGE
```