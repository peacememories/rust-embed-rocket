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
}

impl<T: RustEmbed> Server<T> {
    /// Create a new [`Server`](Server) by using a generic type that implements [`RustEmbed`](::rust_embed::RustEmbed).
    ///
    /// Example:
    /// ```rust
    /// let server = Server::<Assets>::new();
    /// let server: Server<Assets> = Server::new();
    /// ```
    pub fn new() -> Self {
        Server { tag: PhantomData }
    }

    /// Convenience function around [`Server::new`](Server::new). It takes an argument, but only uses its type information, so no overhead is introduced.
    ///
    /// Example:
    /// ```rust
    /// let server = Server::from(Assets);
    /// ```
    pub fn from(_assets: T) -> Self {
        Self::new()
    }
}

impl<T: RustEmbed> Clone for Server<T> {
    fn clone(&self) -> Self {
        Server { tag: PhantomData }
    }
}

impl<T: RustEmbed + 'static> Handler for Server<T> {
    fn handle<'r>(&self, request: &'r Request, _data: Data) -> Outcome<'r> {
        let path: PathBuf = request
            .get_segments(0)
            .map(|s| s.map_err(|_| "Error occurred while parsing segments"))
            .unwrap_or(Ok("".into()))
            .map_err(|e| Err(Status::new(400, e.into())))?;

        let path = if cfg!(feature = "index") && (path.is_dir() || path.to_str() == Some("")) {
            path.join("index.html")
        } else {
            path
        };

        let file_content =
            <T as RustEmbed>::get(path.to_string_lossy().as_ref()).ok_or(Err(Status::NotFound))?;
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
        vec![
            Route::new(Method::Get, "/", self.clone()),
            Route::new(Method::Get, "/<path..>", self),
        ]
    }
}
