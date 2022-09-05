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
let p = HttpApiProblem::new(StatusCode::UNPROCESSABLE_ENTITY)
    .title("You do not have enough credit.")
    .detail("Your current balance is 30, but that costs 50.")
    .type_url("https://example.com/probs/out-of-credit")
    .instance("/account/12345/msgs/abc");

assert_eq!(Some(StatusCode::UNPROCESSABLE_ENTITY), p.status);
assert_eq!(Some("You do not have enough credit."), p.title.as_deref());
assert_eq!(Some("Your current balance is 30, but that costs 50."), p.detail.as_deref());
assert_eq!(Some("https://example.com/probs/out-of-credit"), p.type_url.as_deref());
assert_eq!(Some("/account/12345/msgs/abc"), p.instance.as_deref());
```

There is also `TryFrom<u16>` implemented for [StatusCode]:

```rust
use http_api_problem::*;
let p = HttpApiProblem::try_new(422).unwrap()
    .title("You do not have enough credit.")
    .detail("Your current balance is 30, but that costs 50.")
    .type_url("https://example.com/probs/out-of-credit")
    .instance("/account/12345/msgs/abc");

assert_eq!(Some(StatusCode::UNPROCESSABLE_ENTITY), p.status);
assert_eq!(Some("You do not have enough credit."), p.title.as_deref());
assert_eq!(Some("Your current balance is 30, but that costs 50."), p.detail.as_deref());
assert_eq!(Some("https://example.com/probs/out-of-credit"), p.type_url.as_deref());
assert_eq!(Some("/account/12345/msgs/abc"), p.instance.as_deref());
```

## Features

### Web Frameworks

There are multiple features to integrate with web frameworks:

+ `axum`
* `warp`
* `hyper`
* `actix-web`
* `salvo`
* `tide`
* `rocket`

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
* SohumB

## License

`http-api-problem` is primarily distributed under the terms of both the MIT
license and the Apache License (Version 2.0).

Copyright (c) 2017 Christian Douven.

License: Apache-2.0/MIT
