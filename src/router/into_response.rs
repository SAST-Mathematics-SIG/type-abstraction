use std::borrow::Cow;

use super::{Request, Response, Status};

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

pub trait Handler<Res>
where
    Res: IntoResponse,
{
    fn handle_request(&self, request: Request) -> Res;
}

// fn real_handler<Res, H>(handler: &H, request: Request) -> Response
// where
//     Res: IntoResponse,
//     H: Handler<Res>,
// {
//     handler.handle_request(request).into_response()
// }

impl<F, Res> Handler<Res> for F
where
    F: Fn(Request) -> Res,
    Res: IntoResponse,
{
    fn handle_request(&self, request: Request) -> Res {
        self(request)
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl IntoResponse for Cow<'static, str> {
    fn into_response(self) -> Response {
        Response {
            status: Status::Ok,
            headers: vec![(
                "Content-Type".to_string(),
                "text/plain; charset=utf-8".to_string(),
            )],
            body: match self {
                Cow::Borrowed(b) => b.as_bytes().to_vec(),
                Cow::Owned(o) => o.into_bytes(),
            },
        }
    }
}

impl IntoResponse for Cow<'static, [u8]> {
    fn into_response(self) -> Response {
        Response {
            status: Status::Ok,
            headers: vec![(
                "Content-Type".to_string(),
                "application/octet-stream".to_string(),
            )],
            body: match self {
                Cow::Borrowed(b) => b.to_vec(),
                Cow::Owned(o) => o,
            },
        }
    }
}

impl IntoResponse for &'static [u8] {
    fn into_response(self) -> Response {
        Cow::<'_, [u8]>::Borrowed(self).into_response()
    }
}

impl<const N: usize> IntoResponse for &'static [u8; N] {
    fn into_response(self) -> Response {
        self[..].into_response()
    }
}

impl<const N: usize> IntoResponse for [u8; N] {
    fn into_response(self) -> Response {
        self.to_vec().into_response()
    }
}

impl IntoResponse for Vec<u8> {
    fn into_response(self) -> Response {
        Cow::<'_, [u8]>::Owned(self).into_response()
    }
}

impl IntoResponse for &'static str {
    fn into_response(self) -> Response {
        Cow::<'_, str>::Borrowed(self).into_response()
    }
}

impl IntoResponse for String {
    fn into_response(self) -> Response {
        Cow::<'_, str>::Owned(self).into_response()
    }
}
