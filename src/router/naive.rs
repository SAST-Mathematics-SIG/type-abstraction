use super::{Method, Request, Response, Status};

pub fn handle_request(req: Request) -> Response {
    match req {
        Request {
            method: Method::Get,
            path,
            ..
        } if path == "/hello" => Response {
            status: Status::Ok,
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
            body: b"Hello, world!".to_vec(),
        },

        // Any other GET request should return a 404
        Request {
            method: Method::Get,
            path,
            ..
        } => Response {
            status: Status::NotFound,
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
            body: format!("Path {} not found", path).into_bytes(),
        },

        // OPTIONS requests to /hello should return a 200 with the allowed methods
        Request {
            method: Method::Options,
            path,
            ..
        } if path == "/hello" => Response {
            status: Status::Ok,
            headers: vec![
                ("Content-Type".to_string(), "text/plain".to_string()),
                ("Allow".to_string(), "GET, OPTIONS".to_string()),
            ],
            body: b"GET, OPTIONS".to_vec(),
        },

        // OPTIONS requests to any other path should return a 404
        Request {
            method: Method::Options,
            path,
            ..
        } => Response {
            status: Status::NotFound,
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
            body: format!("Path {} not found", path).into_bytes(),
        },

        // Any other request should return a 400
        Request { method, .. } => Response {
            status: Status::BadRequest,
            headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
            body: format!("Unsupported method {:?}", method).into_bytes(),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_hello() {
        let req = Request {
            method: Method::Get,
            path: "/hello".to_string(),
            headers: vec![
                ("Host".to_string(), "example.com".to_string()),
                ("Accept".to_string(), "text/plain".to_string()),
            ],
            body: vec![],
        };
        let res = handle_request(req);

        dbg!(res);
    }

    #[test]
    fn test_get_world() {
        let req = Request {
            method: Method::Get,
            path: "/world".to_string(),
            headers: vec![
                ("Host".to_string(), "example.com".to_string()),
                ("Accept".to_string(), "text/plain".to_string()),
            ],
            body: vec![],
        };

        let res = handle_request(req);

        dbg!(res);
    }

    #[test]
    fn test_post_hello() {
        let req = Request {
            method: Method::Post,
            path: "/hello".to_string(),
            headers: vec![
                ("Host".to_string(), "example.com".to_string()),
                ("Accept".to_string(), "text/plain".to_string()),
            ],
            body: br#"{"name":"jswn"}"#.to_vec(),
        };

        let res = handle_request(req);

        dbg!(res);
    }

    #[test]
    fn test_options_hello() {
        let req = Request {
            method: Method::Options,
            path: "/hello".to_string(),
            headers: vec![
                ("Host".to_string(), "example.com".to_string()),
                ("Accept".to_string(), "text/plain".to_string()),
            ],
            body: vec![],
        };

        let res = handle_request(req);

        dbg!(res);
    }
}
