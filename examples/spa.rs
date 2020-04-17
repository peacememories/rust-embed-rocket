use rocket;
use rust_embed::RustEmbed;
use rust_embed_rocket::{Config, Server};

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Assets;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            Server::from_config(
                Assets,
                Config {
                    rank: 0,
                    serve_index: false,
                    spa: true,
                },
            ),
        )
        .launch();
}
