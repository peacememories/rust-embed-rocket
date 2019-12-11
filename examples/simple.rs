use rocket;
use rust_embed::RustEmbed;
use rust_embed_rocket::Server;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn main() {
    rocket::ignite().mount("/", Server::from(Asset)).launch();
}
