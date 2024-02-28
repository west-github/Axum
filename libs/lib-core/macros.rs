/// This macros are for lib-core only and not intended to use outside this crate
pub(crate) mod __private {

    /// This macros are for lib-core only and not intended to use outside this crate
    #[macro_export]
    macro_rules! __impl_from_for_validator_errors {
        ($from:ident, $variant:ident) => {
            impl From<$from> for $crate::validator::error::Error {
                fn from(value: $from) -> Self {
                    $crate::validator::error::Error::$variant(value)
                }
            }
        };
    }

    #[macro_export]
    macro_rules! status_codes {
        (
            $(
                $(#[$docs:meta])*
                ($num:expr, $konst:ident);
            )+
        ) => {
            $(
                $(#[$docs])*
                pub const $konst: axum::http::StatusCode = axum::http::StatusCode::$konst;
            )+
        }
    }
}

#[macro_export]
macro_rules! __impl_deref {
    ($ident:ident) => {
        impl<T> std::ops::Deref for $ident<T> {
            type Target = T;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $ident<T> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };

    ($ident:ident, $ty:ty) => {
        impl<T> std::ops::Deref for $ident<T> {
            type Target = $ty;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<T> std::ops::DerefMut for $ident<T> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

#[macro_export]
macro_rules! __impl_error_display {
    ($ident:ident) => {
        impl std::error::Error for $ident {}

        impl std::fmt::Display for $ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Error: {:?}", self)
            }
        }
    };
}
#[doc = r#"Action to be performed on the client
```rust;
// ACTION::NONE
action!();
// ACTION::REDIRECT(String::from("/"))
action!(r: "/");
// ACTION::ALERT(String::from("Some Message"))
action!(a: "Some Message");
```"#]
#[macro_export]
macro_rules! action {
    () => {
        $crate::response::Action::NONE
    };

    (r: $action:expr) => {
        $crate::response::Action::REDIRECT($action.into())
    };

    (a: $action:expr) => {
        $crate::response::Action::ALERT($action.into())
    };
}

#[doc = r#"Response is always in json format with T passed as the body

```rust
use lib_core::rsp;
use lib_core::action;
use lib_core::OK;
use axum::http::{header::ACCEPT, HeaderValue};

#[derive(serde::Serialize, Copy, Clone)]
pub struct Test {}
let test = Test{};
rsp!(OK, test);
rsp!(OK, action: action!(r: "/"));
rsp!(OK, test, action: action!(a: "Some Message"));

let header_name = ACCEPT;
let header_value = HeaderValue::from_static("true");

rsp!{OK, test, (header_name.clone(), header_value.clone())};
rsp!{OK, action: action!(r: "/"), (header_name.clone(), header_value.clone())};
rsp!{OK, test, action: action!(a: "Some Message"), (header_name, header_value)};
```"#]
#[macro_export]
macro_rules! rsp {
    ($status:expr, $body:expr) => {{
        $crate::response::Response::new($status, $body, $crate::action!(), None)
    }};

    ($status:expr, $body:expr, action: $action:expr) => {{
        $crate::response::Response::new($status, $body, $action, None)
    }};

    ($status:expr, $body:expr, $($header:expr),*) => {{
        $crate::response::Response::new($status, $body, $crate::action!(), Some(vec![$($header),*]))
    }};

    ($status:expr, $body:expr, action: $action:expr, $($header:expr),*) => {{
        $crate::response::Response::new($status, $body, $action, Some(vec![$($header),*]))
    }};

    // Action

    ($status:expr, action: $action:expr) => {{
        $crate::response::Response::new($status, (), $action, None)
    }};

    ($status:expr, action: $action:expr, $($header:expr),*) => {{
        $crate::response::Response::new($status, (), $action, Some(vec![$($header),*]))
    }};

    ($status:expr, action: $action:expr, $($header:expr),*) => {{
        $crate::response::Response::new($status, $body, $action, Some(vec![$($header),*]))
    }};
}

#[doc = r#"Wrapped Response in ok"#]
#[macro_export]
macro_rules! rsp_ok {

    ($status:expr, $body:expr) => {{
        Ok($crate::rsp!($status, $body))
    }};

    ($status:expr, $body:expr, action: $action:expr) => {{
        $crate::rsp!($status, $body, $action)
    }};

    ($status:expr, $body:expr, $($header:expr),*) => {{
        $crate::rsp!($status, $body, $($header),*)
    }};

    ($status:expr, $body:expr, action: $action:expr, $($header:expr),*) => {{
        $crate::rsp!($status, $body, $action, $($header),*)
    }};

    // Action

    ($status:expr, action: $action:expr) => {{
        $crate::rsp!($status, action: $action)
    }};


    ($status:expr, action; $action:expr, $($header:expr),*) => {{
        $crate::rsp!($status, action: $action, $($header),*)
    }};

    ($status:expr, action: $action:expr, $($header:expr),*) => {{
        $crate::rsp!($status, $body, $action, $($header),*)
    }};
}

#[macro_export]
macro_rules! unimpl {
    ($value:expr) => {
        println!("INFO: {} is yet to be implemented please check", $value);

        todo!()
    };
}
