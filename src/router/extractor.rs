use super::Request;

pub trait Extractor {
    fn extract(req: &Request) -> Option<Self>
    where
        Self: Sized;
}

pub struct Method(pub super::Method);
pub struct Path(pub String);
pub struct Body(pub Vec<u8>);
pub struct UserAgent(pub String);

// And more...

impl Extractor for Method {
    fn extract(req: &Request) -> Option<Self> {
        Some(Self(req.method))
    }
}

impl Extractor for Path {
    fn extract(req: &Request) -> Option<Self> {
        Some(Self(req.path.clone()))
    }
}

impl Extractor for Body {
    fn extract(req: &Request) -> Option<Self>
    where
        Self: Sized,
    {
        req.body.clone().map(Self)
    }
}

impl Extractor for UserAgent {
    fn extract(req: &Request) -> Option<Self> {
        req.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case("user-agent"))
            .map(|(_, v)| Self(v.clone()))
    }
}

impl<E> Extractor for Option<E>
where
    E: Extractor,
{
    fn extract(req: &Request) -> Option<Self> {
        Some(E::extract(req))
    }
}

// Some tricks from `core::fmt::Debug` implementation for tuples

macro_rules! impl_tuple {
    () => ();
    ( $($name:ident,)+ ) => (
        maybe_tuple_doc! {
            $($name)+ @
            impl<$($name),+> Extractor for ($($name,)+)
                where $($name: Extractor),+
            {
                #[allow(non_snake_case)]
                fn extract(req: &Request) -> Option<Self> {
                    $(
                        let $name = $name::extract(req)?;
                    )+
                    Some(($($name,)+))
                }
            }
        }
        peel! { $($name,)+ }
    )
}

macro_rules! peel {
    ($name:ident, $($other:ident,)*) => (impl_tuple! { $($other,)* })
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

#[allow(unused_macros)]
macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

impl_tuple! { E12, E11, E10, E9, E8, E7, E6, E5, E4, E3, E2, E, }
