use rocket::handler::Outcome;
use rocket::http::ContentType;
use rocket::http::Method;
use rocket::http::Status;
use rocket::response::Content;
use rocket::response::Responder;
use rocket::Data;
use rocket::Handler;
use rocket::Request;
use rocket::Route;
use rust_embed::RustEmbed;
use std::marker::PhantomData;
use std::path::PathBuf;

pub struct Server<T: RustEmbed> {
    tag: PhantomData<fn(T)>,
    config: Config,
}

#[derive(Clone)]
pub struct Config {
    pub rank: isize,
    pub serve_index: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            rank: -2,
            serve_index: false,
        }
    }
}

impl<T: RustEmbed> Server<T> {
    /// Create a new [`Server`](Server) by using a generic type that implements [`RustEmbed`](::rust_embed::RustEmbed).
    ///
    /// Example:
    /// ```rust
    /// let server = Server::<Assets>::new();
    /// let server: Server<Assets> = Server::new();
    /// ```
    #[deprecated]
    pub fn new() -> Self {
        Server {
            tag: PhantomData,
            config: Default::default(),
        }
    }

    /// Convenience function around [`Server::from_config`](Server::new). It takes an argument, but only uses its type information, so no overhead is introduced.
    ///
    /// Example:
    /// ```rust
    /// let server = Server::from(Assets);
    /// ```
    pub fn from(assets: T) -> Self {
        Self::from_config(assets, Default::default())
    }

    /// Create a new [`Server`](Server) configured with the provided [`Config`](Config).
    ///
    /// Example:
    /// ```rust
    /// let server = Server::from_config(Assets, Config {rank: 2, serve_index: false});
    /// ```
    pub fn from_config(_assets: T, config: Config) -> Self {
        Self {
            tag: PhantomData,
            config: config,
        }
    }
}

impl<T: RustEmbed> Clone for Server<T> {
    fn clone(&self) -> Self {
        Server {
            tag: PhantomData,
            config: self.config.clone(),
        }
    }
}

impl<T: RustEmbed + 'static> Handler for Server<T> {
    fn handle<'r>(&self, request: &'r Request, data: Data) -> Outcome<'r> {
        let path: PathBuf = request
            .get_segments(0)
            .map(|s| s.map_err(|_| "Error occurred while parsing segments"))
            .unwrap_or(Ok("".into()))
            .map_err(|e| Err(Status::new(400, e.into())))?;

        let path = if self.config.serve_index && (path.is_dir() || path.to_str() == Some("")) {
            path.join("index.html")
        } else {
            path
        };

        let file_content =
            <T as RustEmbed>::get(path.to_string_lossy().as_ref()).ok_or(Ok(data))?;
        let content_type: ContentType = path
            .extension()
            .map(|x| x.to_string_lossy())
            .and_then(|x| ContentType::from_extension(&x))
            .unwrap_or(ContentType::Plain);
        Outcome::Success(
            Content(content_type, file_content.into_owned())
                .respond_to(request)
                .map_err(|e| Err(e))?,
        )
    }
}

impl<T: RustEmbed + 'static> Into<Vec<Route>> for Server<T> {
    fn into(self) -> Vec<Route> {
        if self.config.serve_index {
            vec![
                Route::ranked(self.config.rank, Method::Get, "/", self.clone()),
                Route::ranked(self.config.rank, Method::Get, "/<path..>", self),
            ]
        } else {
            vec![Route::ranked(
                self.config.rank,
                Method::Get,
                "/<path..>",
                self.clone(),
            )]
        }
    }
}
