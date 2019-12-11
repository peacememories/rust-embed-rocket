use rocket;
use rocket_embed_serve::Server;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "examples/public/"]
struct Asset;

fn main() {
    rocket::ignite().mount("/", Server::from(Asset)).launch();
}
