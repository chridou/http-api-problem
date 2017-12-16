# HTTP-API-PROBLEM

[![crates.io](https://img.shields.io/crates/v/http-api-problem.svg)](https://crates.io/crates/http-api-problem) 
[![docs.rs](https://docs.rs/http-api-problem/badge.svg)](https://docs.rs/http-api-problem) 
[![downloads](https://img.shields.io/crates/d/http-api-problem.svg)](https://crates.io/crates/http-api-problem) 
[![Build Status](https://travis-ci.org/chridou/http-api-problem.svg?branch=master)](https://travis-ci.org/chridou/http-api-problem) 
[![license-mit](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/chridou/http-api-problem/blob/master/LICENSE-MIT) 
[![license-apache](http://img.shields.io/badge/license-APACHE-blue.svg)](https://github.com/chridou/http-api-problem/blob/master/LICENSE-APACHE)

A library to create HTTP response content for APIs based on [RFC7807](https://tools.ietf.org/html/rfc7807).

This library depends on [serde](https://serde.rs/).

The `HttpApiProblem` struct implements `Serialize` and `Deserialize`.

## Usage

Add this to your `Cargo.toml`:

```toml
http-api-problem = "0.2"
```

Add this crate root:

```rust
extern crate http_api_problem;
```

## Example

```rust
use http_api_problem::*;
let p = 
    HttpApiProblem::with_title_and_type_from_status(428)
    .set_detail("detailed explanation")
    .set_instance("/on/1234/do/something");
assert_eq!(Some("https://httpstatuses.com/428".to_string()), ype_url);
assert_eq!(Some(428), p.status);
assert_eq!("Precondition Required".to_string(), p.title);
assert_eq!(Some("detailed explanation".to_string()), p.detail);
assert_eq!(Some("/on/1234/do/something".to_string()), nstance);
```

## License

`http-api-problem` is primarily distributed under the terms of h the MIT license and the
Apache License (Version 2.0).

Copyright (c) 2017 Christian Douven.
