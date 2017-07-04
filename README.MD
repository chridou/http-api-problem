# HTTP-API-PROBLEM

A library to create HTTP response content for APIs based on
[RFC7807](https://tools.ietf.org/html/rfc7807)

This library depends on [serde](https://serde.rs/).

The ```HttpApiProblem``` struct implements `Serialize'.

## Documentation

The [documentation](https://docs.rs/http-api-problem) can be found on [docs.rs](https://docs.rs)

## Usage

Add this to your `Cargo.toml`:
```toml
http-api-problem = { version="0.1.0" }
```

Add this crate root:

```rust
extern crate http_api_problem;
```

## Example

```rust
use http_api_problem::*;

let p = 
    HttpApiProblem::with_type_and_title_from_status(428)
    .set_detail("detailed explanation")
    .set_instance("/on/1234/do/something");

assert_eq!("https://httpstatuses.com/428", p.type_url);
assert_eq!(Some(428), p.status);
assert_eq!(Some("Precondition Required".to_string()), p.title);
assert_eq!(Some("detailed explanation".to_string()), p.detail);
assert_eq!(Some("/on/1234/do/something".to_string()), p.instance);
```

## License

`http-api-problem` is primarily distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.

Copyright (c) 2017 Christian Douven.
