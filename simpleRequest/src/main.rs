#![feature(globs)]

extern crate http;
extern crate url;
use http::client::RequestWriter;
use http::method::Get;
use url::{Url, ParseError};

fn main() {
  let url = Url::parse("http://yahoo.com").unwrap();
  let request: RequestWriter  = match RequestWriter::new(Get, url) {
    Ok(request) => request,
    Err(error) => panic!("{}", error),
  };

  let mut response = match request.read_response() {
    Ok(response) => response,
    Err((_request, error)) => panic!("{}", error),
  };

  println!("{}", response.read_to_string());
}