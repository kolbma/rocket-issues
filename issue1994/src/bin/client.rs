//! A very simple HTTP client that produces an invalid HTTP request.
//! In "path and query", where slash '/' is supposed to be a separator,
//! we do erroneous percent encoding "/" -> "%2F", just to see what the
//! HTTP server does.
//!
//! Instructions for use:
//! Run "cargo run --bin server" in one command window. The HTTP
//! server starts and waits for requests.
//! In another command window, run "cargo run --bin client". The client
//! should send one query to the server and print the outcome.
//! With "cargo run --bin client -- --http1" the request is done with 
//! HTTP/1.1 and works as expected.

#![deny(clippy::all)]
#![deny(keyword_idents)]
#![deny(non_ascii_idents)]
#![deny(unreachable_pub)]
#![deny(unsafe_code)]
// #![deny(unused_crate_dependencies)]
#![deny(unused_qualifications)]
// #![deny(unused_results)]
#![deny(warnings)]

use hyper::{http, Body, Client, Method, StatusCode};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let is_http1 = std::env::args().find(|s| s.ends_with("-http1")).is_some();

    // http2 is the requirement for the issue!!!
    let client = if is_http1 {
        Client::new()
    } else {
        Client::builder().http2_only(true).build_http()
    };

    // Check route of server first
    let req = hyper::Request::get("http://127.0.0.1:8000/hello/world").body(Body::empty())?;
    dbg!(&req);
    let res = client.request(req).await?;
    dbg!(&res);
    assert!(res.status().is_success());

    // Variant 1
    // Seems to work as expected. Path is exactly "%2Fhello%2Fworld".
    let req = hyper::Request::get("http://127.0.0.1:8000/%2Fhello%2Fworld").body(Body::empty())?;
    dbg!(&req);
    let res = client.request(req).await?;
    dbg!(&res);
    assert_eq!(res.status(), StatusCode::NOT_FOUND);

    // Variant 2 path without slash between host:port and path
    // With `get()` there is an http::Error(InvalidUri(InvalidAuthority))
    // let req = hyper::Request::get("http://127.0.0.1:8000%2Fhello%2Fworld").body(Body::empty())?;

    // `uri::Builder` accepts an invalid path
    let uri = http::uri::Builder::new()
        .scheme("http")
        .authority("127.0.0.1:8000")
        .path_and_query("%2Fhello%2Fworld")
        .build()?;

    // this is an invalid request
    let req = hyper::Request::builder()
        .method(Method::GET)
        .uri(uri)
        .body(Body::empty())?;
    dbg!(&req);
    let res = client.request(req).await?;
    dbg!(&res);

    // With http/1.1...
    // Is it ok to be bad request? Or should it be not found?
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    Ok(())
}
