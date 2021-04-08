# HTTP-API-PROBLEM

[![crates.io](https://img.shields.io/crates/v/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
[![docs.rs](https://docs.rs/http-api-problem/badge.svg)](https://docs.rs/http-api-problem)
[![downloads](https://img.shields.io/crates/d/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
![CI](https://github.com/chridou/http-api-problem/workflows/CI/badge.svg)
[![license-mit](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/chridou/http-api-problem/blob/master/LICENSE-MIT)
[![license-apache](http://img.shields.io/badge/license-APACHE-blue.svg)](https://github.com/chridou/http-api-problem/blob/master/LICENSE-APACHE)

A library to create HTTP response content for APIs based on
[RFC7807](https://tools.ietf.org/html/rfc7807).

## Usage

Get the latest version for your `Cargo.toml` from
[crates.io](https://crates.io/crates/http-api-problem).

## Serde

`HttpApiProblem` implements `Serialize` and `Deserialize`.

## Examples
```rust
use http_api_problem::*;
let p = HttpApiProblem::with_title_and_type(StatusCode::NOT_FOUND)
    .detail("detailed explanation")
    .instance("/on/1234/do/something");
assert_eq!(Some("https://httpstatuses.com/404".to_string()), p.type_url);
assert_eq!(Some(StatusCode::NOT_FOUND), p.status);
assert_eq!("Not Found".to_string(), p.title);
assert_eq!(Some("detailed explanation".to_string()), p.detail);
assert_eq!(Some("/on/1234/do/something".to_string()), p.instance);
```

There is also `TryFrom<u16>` implemented for `StatusCode`:

```rust
use http_api_problem::*;
let p = HttpApiProblem::try_with_title_and_type(428)
    .detail("detailed explanation")
    .instance("/on/1234/do/something")
    .unwrap();
assert_eq!(Some("https://httpstatuses.com/428".to_string()), p.type_url);
assert_eq!(Some(StatusCode::PRECONDITION_REQUIRED), p.status);
assert_eq!("Precondition Required".to_string(), p.title);
assert_eq!(Some("detailed explanation".to_string()), p.detail);
assert_eq!(Some("/on/1234/do/something".to_string()), p.instance);
```

## Features

### Web Frameworks

There are multiple features to integrate with web frameworks:

* `warp`
* `hyper`
* `actix-web`
* `salvo`
* `tide`

These mainly convert the `HttpApiProblem` to response types of
the frameworks and implement traits to integrate with the frameworks
error handling

### ApiError

The feature `api-error` enables a structure which can be
return from "api handlers" that generate responses and can be 
converted into an `HttpApiProblem`.

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
