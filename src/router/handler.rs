use std::fmt;

use super::{extractor::Extractor, into_response::IntoResponse, Request, Response};

pub trait Handler<E> {
    fn handle(&self, request: &Request) -> Response;
}

impl<F, R> Handler<()> for F
where
    F: Fn() -> R,
    R: IntoResponse,
{
    fn handle(&self, _: &Request) -> Response {
        self().into_response()
    }
}

// Some tricks from `core::fmt::Debug` implementation for tuples

macro_rules! impl_handler {
    () => {};
    ( $($name:ident,)+ ) => (
        maybe_tuple_doc! {
            $($name)+ @
            impl<F, R, $($name),+> Handler<($($name,)*)> for F
            where
                F: Fn($($name),+) -> R,
                $($name: Extractor,)+
                R: IntoResponse,
            {
                #[allow(non_snake_case)]
                fn handle(&self, request: &Request) -> Response {
                    let ($($name,)+) = <($($name,)*)>::extract(request).expect("failed to extract");
                    self($($name),+).into_response()
                }
            }
        }
        peel! { $($name,)+ }
    );
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (impl_handler! { $($other,)* })
}

macro_rules! maybe_tuple_doc {
    ($a:ident @ $(#[$meta:meta])* $item:item) => {
        #[cfg_attr(all(doc, nightly), doc(fake_variadic))]
        #[doc = "This trait is implemented for tuples up to twelve items long."]
        $(#[$meta])*
        $item
    };
    ($a:ident $($rest_a:ident)+ @ $(#[$meta:meta])* $item:item) => {
        #[cfg_attr(all(doc, nightly), doc(hidden))]
        $(#[$meta:meta])*
        $item
    };
}

impl_handler! { E12, E11, E10, E9, E8, E7, E6, E5, E4, E3, E2, E, }

/// Type-erased version of a [`Handler`].
pub(crate) struct BoxedHandler(Box<dyn Fn(&Request) -> Response>);

impl fmt::Debug for BoxedHandler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoxedHandler").finish()
    }
}

impl BoxedHandler {
    pub(crate) fn from_handler<E, H>(handler: H) -> Self
    where
        H: Handler<E> + 'static,
    {
        BoxedHandler(Box::new(move |req| handler.handle(req)))
    }
}

impl Handler<()> for BoxedHandler {
    fn handle(&self, req: &Request) -> Response {
        (self.0)(req)
    }
}
