# HTTP-API-PROBLEM

[![crates.io](https://img.shields.io/crates/v/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
[![docs.rs](https://docs.rs/http-api-problem/badge.svg)](https://docs.rs/http-api-problem)
[![downloads](https://img.shields.io/crates/d/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
[![Build Status](https://travis-ci.org/chridou/http-api-problem.svg?branch=master)](https://travis-ci.org/chridou/http-api-problem)
[![license-mit](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/chridou/http-api-problem/blob/master/LICENSE-MIT)
[![license-apache](http://img.shields.io/badge/license-APACHE-blue.svg)](https://github.com/chridou/http-api-problem/blob/master/LICENSE-APACHE)

A library to create HTTP response content for APIs based on
[RFC7807](https://tools.ietf.org/html/rfc7807).

** Breaking changes! This crate now uses `http::StatusCode` instead of the own custom one **

## Usage

Get the latest version for your `Cargo.toml` from
[crates.io](https://crates.io/crates/http-api-problem).

Add this to your crate root:

```rust
extern crate http_api_problem;
```

 ## serde

`HttpApiProblem` implements `Serialize` and `Deserialize` for
`HttpApiProblem`.

## Examples

```rust
use http_api_problem::*;

let p = HttpApiProblem::with_title_and_type_from_status(HttpStatusCode::NotFound)
    .set_detail("detailed explanation")
    .set_instance("/on/1234/do/something");

assert_eq!(Some("https://httpstatuses.com/404".to_string()), p.type_url);
assert_eq!(Some(HttpStatusCode::NotFound), p.status);
assert_eq!("Not Found".to_string(), p.title);
assert_eq!(Some("detailed explanation".to_string()), p.detail);
assert_eq!(Some("/on/1234/do/something".to_string()), p.instance);
```

There is also `From<u16>` implemented for `HttpStatusCode`:

```rust
use http_api_problem::*;

let p = HttpApiProblem::with_title_and_type_from_status(428)
    .set_detail("detailed explanation")
    .set_instance("/on/1234/do/something");

assert_eq!(Some("https://httpstatuses.com/428".to_string()), p.type_url);
assert_eq!(Some(HttpStatusCode::PreconditionRequired), p.status);
assert_eq!("Precondition Required".to_string(), p.title);
assert_eq!(Some("detailed explanation".to_string()), p.detail);
assert_eq!(Some("/on/1234/do/something".to_string()), p.instance);
```

## Features


### with_iron

There is a conversion between `iron`s StatusCode and `HttpStatusCode` back
and forth.

The `HttpApiProblem` provides a method `to_iron_response` which constructs
an iron `Response`. If the `status` field of the `HttpApiProblem` is `None`
`500 - Internal Server Error` is the default.

`From<HttpApiProblem` for `iron::response::Response` will also be there. It
simply calls `to_iron_response`.

Additionally there will be a function `into_iron_response` which converts
anything into an `iron::response::Response` that can be converted into a
`HttpApiProblem`.

### with_hyper

There is a conversion between `hypers`s StatusCode and `HttpStatusCode`
back and forth.

The `HttpApiProblem` provides a method `to_hyper_response` which constructs
a hyper `Response`. If the `status` field of the `HttpApiProblem` is `None`
`500 - Internal Server Error` is the default.

`From<HttpApiProblem` for `hyper::Response` will also be there. It simply
calls `to_hyper_response`.

Additionally there will be a function `into_iron_response` which converts
anything into a `hyper::Response` that can be converted into a
`HttpApiProblem`.


## Recent changes

* 0.13.1 
    * Impl `std::error::Error` for HttpApiProblem

* 0.13.0 
    * Temporarely disable rocket
    * update actix to 1.0
    * do not support `iron` any longer
* 0.12.0 Added experimental API Error type
* 0.11.0 Added `actix_web` support
* 0.10.0 Use `http::StatusCode` **Breaking change**

## Thank you
A big "thank you" for contributions and inspirations goes to the
following GitHub users:

* panicbit
* thomaseizinger

## License

`http-api-problem` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

Copyright (c) 2017 Christian Douven.

License: Apache-2.0/MIT
