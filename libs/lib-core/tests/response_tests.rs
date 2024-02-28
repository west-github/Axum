use axum::http::{header::ACCEPT, HeaderValue};
use lib_core::response::Action::*;
use lib_core::response::*;
use lib_core::*;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize, Clone, Copy)]
struct Test {
    data: &'static str,
}

impl Test {
    fn new() -> Self {
        let data = "This is a test";
        Self { data }
    }
}

#[test]
fn test_body() {
    let test = Test::new();

    let res = rsp!(OK, test);

    assert_eq!(res, Response::new(OK, test, NONE, None));
}

#[test]
fn test_action_only() {
    let rsp = rsp!(OK, action: action!(r: "/"));

    assert_eq!(rsp, Response::new(OK, (), REDIRECT("/".into()), None));
}

#[test]
fn test_body_with_single_header() {
    let test = Test::new();

    let res = rsp! {OK, test,
        (ACCEPT, HeaderValue::from_static("Foo"))
    };

    assert_eq!(
        res,
        Response::new(
            OK,
            test,
            NONE,
            Some(vec![(ACCEPT, HeaderValue::from_static("Foo"))])
        )
    );
}

#[test]
fn test_body_with_multiple_header() {
    let test = Test::new();

    let res = rsp! {OK, test,
        (ACCEPT, HeaderValue::from_static("Foo")),
        (ACCEPT, HeaderValue::from_static("Foo"))
    };

    assert_eq!(
        res,
        Response::new(
            OK,
            test,
            NONE,
            Some(vec![
                (ACCEPT, HeaderValue::from_static("Foo")),
                (ACCEPT, HeaderValue::from_static("Foo"))
            ])
        )
    );
}

#[test]
fn test_action_with_header() {
    let rsp = rsp!(OK, action: action!(r: "/"), (ACCEPT, HeaderValue::from_static("Foo")));

    assert_eq!(
        rsp,
        Response::new(
            OK,
            (),
            REDIRECT("/".into()),
            Some(vec![(ACCEPT, HeaderValue::from_static("Foo"))])
        )
    );
}

#[test]
fn test_action_with_multiple_header() {
    let rsp = rsp!(OK, action: action!(r: "/"),
        (ACCEPT, HeaderValue::from_static("Foo")),
        (ACCEPT, HeaderValue::from_static("Foo")),
        (ACCEPT, HeaderValue::from_static("Foo"))
    );

    assert_eq!(
        rsp,
        Response::new(
            OK,
            (),
            REDIRECT("/".into()),
            Some(vec![
                (ACCEPT, HeaderValue::from_static("Foo")),
                (ACCEPT, HeaderValue::from_static("Foo")),
                (ACCEPT, HeaderValue::from_static("Foo"))
            ])
        )
    );
}

#[test]
fn test_action_with_body() {
    let test = Test::new();
    let res = rsp!(OK, test, action: action!(r: "/"));

    assert_eq!(res, Response::new(OK, test, REDIRECT("/".into()), None));

    let res = rsp!(OK, test, action: action!(a: "We got the info"));

    assert_eq!(
        res,
        Response::new(OK, test, ALERT("We got the info".into()), None)
    );
}

#[test]
fn test_body_action_single_header() {
    let test = Test::new();

    let res = rsp! {OK, test, action: action!(a: "We got the info"),
        (ACCEPT, HeaderValue::from_static("Foo"))
    };

    assert_eq!(
        res,
        Response::new(
            OK,
            test,
            ALERT("We got the info".into()),
            Some(vec![(ACCEPT, HeaderValue::from_static("Foo"))])
        )
    );

    let res = rsp! {OK, test, action: action!(r: "/"),
        (ACCEPT, HeaderValue::from_static("Foo"))
    };

    assert_eq!(
        res,
        Response::new(
            OK,
            test,
            REDIRECT("/".into()),
            Some(vec![(ACCEPT, HeaderValue::from_static("Foo"))])
        )
    )
}

#[test]
fn test_body_action_with_multiple_header() {
    let test = Test::new();

    let res = rsp! {OK, test, action: action!(a: "We got the info"),
        (ACCEPT, HeaderValue::from_static("Foo")),
        (ACCEPT, HeaderValue::from_static("Foo"))
    };

    assert_eq!(
        res,
        Response::new(
            OK,
            test,
            ALERT("We got the info".into()),
            Some(vec![
                (ACCEPT, HeaderValue::from_static("Foo")),
                (ACCEPT, HeaderValue::from_static("Foo"))
            ])
        )
    );

    let res = rsp! {OK, test, action: action!(r: "/"),
            (ACCEPT, HeaderValue::from_static("Foo")),
            (ACCEPT, HeaderValue::from_static("Foo"))
    };

    assert_eq!(
        res,
        Response::new(
            OK,
            test,
            REDIRECT("/".into()),
            Some(vec![
                (ACCEPT, HeaderValue::from_static("Foo")),
                (ACCEPT, HeaderValue::from_static("Foo"))
            ])
        )
    )
}
