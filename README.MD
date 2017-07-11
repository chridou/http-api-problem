# HTTP-API-PROBLEM

[![Build Status](https://secure.travis-ci.org/chridou/http-api-problem.png)](http://travis-ci.org/chridou/http-api-problem) 

 A library to create HTTP response content for APIs based on
 [RFC7807](https://tools.ietf.org/html/rfc7807).

 This library depends on [serde](https://serde.rs/).

 The `HttpApiProblem` struct implements `Serialize` and erialize`.

 ## Usage
 
 Add this to your `Cargo.toml`:

 ```toml
 http-api-problem = "0.1"
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
