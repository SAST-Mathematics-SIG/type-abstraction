use std::fmt;

pub mod naive;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(Clone)]
pub struct Response {
    pub status: Status,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
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
            .field("body", &String::from_utf8_lossy(&self.body))
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
