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

pub struct Server<T: RustEmbed + 'static> {
    tag: PhantomData<fn(T)>,
}

impl<T: RustEmbed> Server<T> {
    pub fn new() -> Self {
        Server { tag: PhantomData }
    }

    pub fn from(_assets: T) -> Self {
        Server { tag: PhantomData }
    }
}

impl<T: RustEmbed> Clone for Server<T> {
    fn clone(&self) -> Self {
        Server { tag: PhantomData }
    }
}

impl<T: RustEmbed> Handler for Server<T> {
    fn handle<'r>(&self, request: &'r Request, _data: Data) -> Outcome<'r> {
        let segments_option = request
            .get_segments(0)
            .map(|s| s.map_err(|_| "Error occurred while parsing segments"));
        let segments = segments_option.unwrap_or(Err("No path supplied"));
        let path: PathBuf = segments.map_err(|e| Err(Status::new(400, e.into())))?;
        let file_content = <T as RustEmbed>::get(path.to_string_lossy().as_ref())
            .ok_or(Err(Status::new(404, "File not found")))?;
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

impl<T: RustEmbed> Into<Vec<Route>> for Server<T> {
    fn into(self) -> Vec<Route> {
        vec![Route::new(Method::Get, "/<path..>", self)]
    }
}