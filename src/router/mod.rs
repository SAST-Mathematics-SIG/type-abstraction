use std::{collections::HashMap, fmt};

use handler::{BoxedHandler, Handler};

pub mod extractor;
pub mod handler;
pub mod into_response;
pub mod naive;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[derive(Clone)]
pub struct Response {
    pub status: Status,
    pub headers: Vec<(String, String)>,
    pub body: Option<Vec<u8>>,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Options,
}

#[repr(u16)]
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Ok = 200,
    Created = 201,
    NoContent = 204,
    MovedPermanently = 301,
    BadRequest = 400,
    NotFound = 404,
    InternalServerError = 500,
}

impl fmt::Debug for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use indexmap::IndexMap;
        f.debug_struct("Response")
            .field("status", &self.status)
            .field(
                "headers",
                &self
                    .headers
                    .iter()
                    // .map(|(k, v)| format!("{}: {}", k, v))
                    .cloned()
                    .collect::<IndexMap<_, _>>(),
            )
            .field(
                "body",
                &self.body.as_ref().map(|b| String::from_utf8_lossy(b)),
            )
            .finish()
    }
}
impl fmt::Debug for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "200 OK"),
            Self::Created => write!(f, "201 Created"),
            Self::NoContent => write!(f, "204 No Content"),
            Self::MovedPermanently => write!(f, "301 Moved Permanently"),
            Self::BadRequest => write!(f, "400 Bad Request"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::InternalServerError => write!(f, "500 Internal Server Error"),
        }
    }
}

#[derive(Debug, Default)]
pub struct App {
    handlers: HashMap<String, BoxedHandler>,
}

impl App {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn with_handler<E, H>(&mut self, path: &str, handler: H) -> &mut Self
    where
        H: Handler<E> + 'static,
    {
        self.handlers
            .insert(path.to_owned(), BoxedHandler::from_handler(handler));
        self
    }

    pub fn handle(&self, req: &Request) -> Response {
        self.handlers.get(&req.path).map_or_else(
            || Response {
                status: Status::NotFound,
                headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
                body: Some(format!("Path {} not found", req.path).into_bytes()),
            },
            |handler| handler.handle(req),
        )
    }
}
