# Rust Embed Rocket

This crate provides a static file server for [Rocket](https://rocket.rs) backed by the [`rust-embed`](https://crates.io/crates/rust-embed) crate. This allows users to ship their frontend with their webserver.

**!!!** This crate is currently very young and naively written. Please check the code thoroughly before even thinking about using it in anything important **!!!**

## Install

Since the crate is currently not hosted on [crates.io](https://crates.io), you need to add it to your project using git dependencies:

```toml
#Cargo.toml
#...
[dependencies]
rust-embed-rocket = {git = "https://github.com/peacememories/rust-embed-rocket"}
#...
```

## Example

Just create a `RustEmbed` structure as described in their readme and use it to create an `Asset` struct:

```rust
use rocket;
use rust_embed::RustEmbed;
use rust_embed_rocket::Server;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn main() {
    rocket::ignite().mount("/", Server::from(Asset)).launch();
}
```

## Configuration Options

### Rank

The Rocket rank (see [here](https://rocket.rs/v0.4/guide/requests/#forwarding)) can be set by changing the `rank` option supplied to `from_config`.

Default: `0`

### Serve Index

It is common to want a url like `/foo` to serve the file `/foo/index.html`. This can be done by enabling the boolean `serve_index` in the config.

Default: `false`

## Feedback & Contributions

Feedback and Contributions are welcome, either through GitHubs issue and pr tracker or on Riot/IRC under the handle `@peacememories`

Please respect the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct)
