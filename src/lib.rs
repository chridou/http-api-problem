//! # HTTP-API-PROBLEM
//!
//! [![crates.io](https://img.shields.io/crates/v/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
//! [![docs.rs](https://docs.rs/http-api-problem/badge.svg)](https://docs.rs/http-api-problem)
//! [![downloads](https://img.shields.io/crates/d/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
//! [![Build Status](https://travis-ci.org/chridou/http-api-problem.svg?branch=master)](https://travis-ci.org/chridou/http-api-problem)
//! [![license-mit](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/chridou/http-api-problem/blob/master/LICENSE-MIT)
//! [![license-apache](http://img.shields.io/badge/license-APACHE-blue.svg)](https://github.com/chridou/http-api-problem/blob/master/LICENSE-APACHE)
//!
//! A library to create HTTP response content for APIs based on
//! [RFC7807](https://tools.ietf.org/html/rfc7807).
//!
//! ## Usage
//!
//! Get the latest version for your `Cargo.toml` from
//! [crates.io](https://crates.io/crates/http-api-problem).
//!
//! Add this to your crate root:
//!
//! ```rust
//! extern crate http_api_problem;
//! ```
//!
//!  ## serde
//!
//! `HttpApiProblem` implements `Serialize` and `Deserialize` for
//! `HttpApiProblem`.
//!
//! ## Examples
//!
//! ```rust
//! use http_api_problem::*;
//!
//! let p = HttpApiProblem::with_title_and_type_from_status(StatusCode::NOT_FOUND)
//!     .set_detail("detailed explanation")
//!     .set_instance("/on/1234/do/something");
//!
//! assert_eq!(Some("https://httpstatuses.com/404".to_string()), p.type_url);
//! assert_eq!(Some(StatusCode::NOT_FOUND), p.status);
//! assert_eq!("Not Found".to_string(), p.title);
//! assert_eq!(Some("detailed explanation".to_string()), p.detail);
//! assert_eq!(Some("/on/1234/do/something".to_string()), p.instance);
//! ```
//!
//! There is also `From<u16>` implemented for `StatusCode`:
//!
//! ```rust
//! use http_api_problem::*;
//!
//! let p = HttpApiProblem::with_title_and_type_from_status(StatusCode::PRECONDITION_REQUIRED)
//!     .set_detail("detailed explanation")
//!     .set_instance("/on/1234/do/something");
//!
//! assert_eq!(Some("https://httpstatuses.com/428".to_string()), p.type_url);
//! assert_eq!(Some(StatusCode::PRECONDITION_REQUIRED), p.status);
//! assert_eq!("Precondition Required".to_string(), p.title);
//! assert_eq!(Some("detailed explanation".to_string()), p.detail);
//! assert_eq!(Some("/on/1234/do/something".to_string()), p.instance);
//! ```
//!
//! ## Features
//!
//!
//! ### with_iron
//!
//! There is a conversion between `iron`s StatusCode and `StatusCode` back
//! and forth.
//!
//! The `HttpApiProblem` provides a method `to_iron_response` which constructs
//! an iron `Response`. If the `status` field of the `HttpApiProblem` is `None`
//! `500 - Internal Server Error` is the default.
//!
//! `From<HttpApiProblem` for `iron::response::Response` will also be there. It
//! simply calls `to_iron_response`.
//!
//! Additionally there will be a function `into_iron_response` which converts
//! anything into an `iron::response::Response` that can be converted into a
//! `HttpApiProblem`.
//!
//! ### with_hyper
//!
//! There is a conversion between `hypers`s StatusCode and `StatusCode`
//! back and forth.
//!
//! The `HttpApiProblem` provides a method `to_hyper_response` which constructs
//! a hyper `Response`. If the `status` field of the `HttpApiProblem` is `None`
//! `500 - Internal Server Error` is the default.
//!
//! `From<HttpApiProblem` for `hyper::Response` will also be there. It simply
//! calls `to_hyper_response`.
//!
//! Additionally there will be a function `into_iron_response` which converts
//! anything into a `hyper::Response` that can be converted into a
//! `HttpApiProblem`.
//!
//! ### with_reqwest
//!
//! There is a conversion between `reqwest`s StatusCode and `StatusCode`
//! back and forth.
//!
//! ### with_rocket(nightly only)
//!
//! There is a conversion between `rocket`s Status and `StatusCode` back
//! and forth.
//!
//! `HttpApiProblem` implements `rocket::response::Responder`, allowing it to
//! be returned from rocket handlers directly (e.g. as `Result<T,
//! HttpApiProblem>`). It also provides a method `to_rocket_response` which
//! explicitly constructs a rocket `Response`. If the `status` field of the
//! `HttpApiProblem` is `None` `500 - Internal Server Error` is the default.
//!
//! `From<HttpApiProblem` for `rocket::Response` will also be there. It simply
//! calls `to_rocket_response`.
//!
//! Additionally there will be a function `into_rocket_response` which converts
//! anything into a `rocket::Response` that can be converted into a
//! `HttpApiProblem`.
//!
//!
//! ## Recent changes
//!
//! * 0.9.0
//!     * removed  feature `with-reqwest` since it was bumped to 0.9
//! * 0.7.0
//!     * Feature `with_reqwest` added
//!     * `HttpApiProblem` can now contain additional fields
//! * 0.6.2
//!     * Feature `with_hyper` returns Response<Body>
//! * 0.6.1
//!     * Feature `with_hyper` returns response Vec<u8>
//! * 0.6.0
//!     * Feature `with_hyper` uses hyper 0.12
//!
//! ## License
//!
//! `http-api-problem` is primarily distributed under the terms of both the MIT
//! license and the Apache License (Version 2.0).
//!
//! Copyright (c) 2017 Christian Douven.
#[macro_use]
extern crate serde;
extern crate http;
extern crate serde_json;

#[cfg(feature = "with_iron")]
extern crate iron;

#[cfg(feature = "with_hyper")]
extern crate hyper;

#[cfg(feature = "with_rocket")]
extern crate rocket;

use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

pub use http::StatusCode;

/// The recommended media type when serialized to JSON
pub static PROBLEM_JSON_MEDIA_TYPE: &'static str = "application/problem+json";

/// Description of a problem that can be returned by an HTTP API
/// based on [RFC7807](https://tools.ietf.org/html/rfc7807)
///
/// # Example
///
/// ```javascript
/// {
///    "type": "https://example.com/probs/out-of-credit",
///    "title": "You do not have enough credit.",
///    "detail": "Your current balance is 30, but that costs 50.",
///    "instance": "/account/12345/msgs/abc",
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq))]
pub struct HttpApiProblem {
    /// A URI reference [RFC3986](https://tools.ietf.org/html/rfc3986) that identifies the
    /// problem type.  This specification encourages that, when
    /// dereferenced, it provide human-readable documentation for the
    /// problem type (e.g., using HTML [W3C.REC-html5-20141028]).  When
    /// this member is not present, its value is assumed to be
    /// "about:blank".
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_url: Option<String>,
    /// The HTTP status code [RFC7231, Section 6](https://tools.ietf.org/html/rfc7231#section-6)
    /// generated by the origin server for this occurrence of the problem.
    #[serde(default)]
    #[serde(with = "custom_http_status_serialization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<StatusCode>,
    /// A short, human-readable summary of the problem
    /// type. It SHOULD NOT change from occurrence to occurrence of the
    /// problem, except for purposes of localization (e.g., using
    /// proactive content negotiation;
    /// see [RFC7231, Section 3.4](https://tools.ietf.org/html/rfc7231#section-3.4).
    ///
    /// This is the only mandatory field.
    pub title: String,
    /// A human-readable explanation specific to this
    /// occurrence of the problem.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// A URI reference that identifies the specific
    /// occurrence of the problem.  It may or may not yield further
    /// information if dereferenced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,

    /// Additional fields that must be JSON values
    #[serde(flatten)]
    additional_fields: HashMap<String, serde_json::Value>,
}

impl HttpApiProblem {
    /// Creates a new instance with the given `title`.
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::new("Internal Error");
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Internal Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn new<T: Into<String>>(title: T) -> HttpApiProblem {
        HttpApiProblem {
            type_url: None,
            status: None,
            title: title.into(),
            detail: None,
            instance: None,
            additional_fields: Default::default(),
        }
    }

    /// Creates a new instance with the `title` and `type_url` derived from the
    /// `status`.
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::with_title_and_type_from_status(StatusCode::SERVICE_UNAVAILABLE);
    ///
    /// assert_eq!(Some("https://httpstatuses.com/503".to_string()), p.type_url);
    /// assert_eq!(Some(StatusCode::SERVICE_UNAVAILABLE), p.status);
    /// assert_eq!("Service Unavailable", &p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn with_title_and_type_from_status<T: Into<StatusCode>>(status: T) -> HttpApiProblem {
        let status = status.into();
        HttpApiProblem {
            type_url: Some(format!("https://httpstatuses.com/{}", status.as_u16())),
            status: Some(status.into()),
            title: status.canonical_reason().unwrap_or("<unknown status code>").to_string(),
            detail: None,
            instance: None,
            additional_fields: Default::default(),
        }
    }

    /// Creates a new instance with `title` derived from `status`.
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::with_title_from_status(StatusCode::NOT_FOUND);
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(Some(StatusCode::NOT_FOUND), p.status);
    /// assert_eq!("Not Found", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn with_title_from_status<T: Into<StatusCode>>(status: T) -> HttpApiProblem {
        let status = status.into();
        HttpApiProblem {
            type_url: None,
            status: Some(status.into()),
            title: status.canonical_reason().unwrap_or("<unknown status code>").to_string(),
            detail: None,
            instance: None,
            additional_fields: Default::default(),
        }
    }

    /// Sets the `type_url`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::new("Error").set_type_url("http://example.com/my/real_error");
    ///
    /// assert_eq!(Some("http://example.com/my/real_error".to_string()), p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn set_type_url<T: Into<String>>(self, type_url: T) -> HttpApiProblem {
        let mut s = self;
        s.type_url = Some(type_url.into());
        s
    }

    /// Sets the `status`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::new("Error").set_status(StatusCode::NOT_FOUND);
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(Some(StatusCode::NOT_FOUND), p.status);
    /// assert_eq!("Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn set_status<T: Into<StatusCode>>(self, status: T) -> HttpApiProblem {
        let status = status.into();
        let mut s = self;
        s.status = Some(status.into());
        s
    }

    /// Sets the `title`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::new("Error").set_title("Another Error");
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Another Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn set_title<T: Into<String>>(self, title: T) -> HttpApiProblem {
        let mut s = self;
        s.title = title.into();
        s
    }

    /// Sets the `detail`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::new("Error").set_detail("a detailed description");
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Error", p.title);
    /// assert_eq!(Some("a detailed description".to_string()), p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn set_detail<T: Into<String>>(self, detail: T) -> HttpApiProblem {
        let mut s = self;
        s.detail = Some(detail.into());
        s
    }

    /// Add a value that must be serializable. The key must not be one of the
    /// field names of this struct.
    pub fn set_value<K, V>(&mut self, key: K, value: &V) -> Result<(), String>
    where
        V: Serialize,
        K: Into<String>,
    {
        let key: String = key.into();
        match key.as_ref() {
            "type" => return Err("'type' is a reserved field name".into()),
            "status" => return Err("'status' is a reserved field name".into()),
            "title" => return Err("'title' is a reserved field name".into()),
            "detail" => return Err("'detail' is a reserved field name".into()),
            "instance" => return Err("'instance' is a reserved field name".into()),
            "additional_fields" => return Err("'additional_fields' is a reserved field name".into()),
            _ => (),
        }
        let serialized = serde_json::to_value(value).map_err(|err| err.to_string())?;
        self.additional_fields.insert(key, serialized);
        Ok(())
    }

    /// Returns the deserialized field for the given key.
    ///
    /// If the key does not exist or the field is not deserializable to
    /// the target type `None` is returned
    pub fn value<K, V>(&self, key: &str) -> Option<V>
    where
        V: DeserializeOwned,
    {
        self.json_value(key)
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    /// Returns the `serde_json::Value` for the given key if the key exists.
    pub fn json_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.additional_fields.get(key)
    }

    pub fn keys<K, V>(&self) -> impl Iterator<Item = &String>
    where
        V: DeserializeOwned,
    {
        self.additional_fields.keys()
    }

    /// Sets the `instance`
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::new("Error").set_instance("/account/1234/withdraw");
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(None, p.status);
    /// assert_eq!("Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(Some("/account/1234/withdraw".to_string()), p.instance);
    /// ```
    pub fn set_instance<T: Into<String>>(self, instance: T) -> HttpApiProblem {
        let mut s = self;
        s.instance = Some(instance.into());
        s
    }

    /// Serialize to a JSON `Vec<u8>`
    pub fn json_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap()
    }

    /// Serialize to a JSON `String`
    pub fn json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    fn status(&self) -> StatusCode {
        self.status.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn status_code(&self) -> u16 {
        self.status().as_u16()
    }

    /// Creates an `iron` response.
    ///
    /// If status is `None` `500 - Internal Server Error` is the
    /// default.
    #[cfg(feature = "with_iron")]
    pub fn to_iron_response(&self) -> ::iron::response::Response {
        use iron::headers::ContentType;
        use iron::mime::Mime;
        use iron::status::Status;
        use iron::*;

        let mut response = Response::with((Status::from_u16(self.status_code()), self.json_bytes()));
        let mime: Mime = PROBLEM_JSON_MEDIA_TYPE.parse().unwrap();
        response.headers.set(ContentType(mime));

        response
    }

    /// Creates a `hyper` response.
    ///
    /// If status is `None` `500 - Internal Server Error` is the
    /// default.
    #[cfg(feature = "with_hyper")]
    pub fn to_hyper_response(self) -> hyper::Response<hyper::Body> {
        use hyper::header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
        use hyper::*;

        let json = self.json_bytes();
        let length = json.len() as u64;

        let (mut parts, body) = Response::new(json.into()).into_parts();

        parts
            .headers
            .insert(CONTENT_TYPE, HeaderValue::from_static(PROBLEM_JSON_MEDIA_TYPE));
        parts
            .headers
            .insert(CONTENT_LENGTH, HeaderValue::from_str(&length.to_string()).unwrap());
        parts.status = self.status();

        Response::from_parts(parts, body)
    }

    /// Creates a `rocket` response.
    ///
    /// If status is `None` `500 - Internal Server Error` is the
    /// default.
    #[cfg(feature = "with_rocket")]
    pub fn to_rocket_response(&self) -> rocket::Response<'static> {
        use rocket::http::ContentType;
        use rocket::http::Status;
        use rocket::Response;
        use std::io::Cursor;

        let content_type: ContentType = PROBLEM_JSON_MEDIA_TYPE.parse().unwrap();
        let json = self.json_bytes();
        let response = Response::build()
            .status(Status::raw(self.status_code()))
            .sized_body(Cursor::new(json))
            .header(content_type)
            .finalize();

        response
    }
}

impl From<StatusCode> for HttpApiProblem {
    fn from(status: StatusCode) -> HttpApiProblem {
        HttpApiProblem::with_title_from_status(status)
    }
}

/// Creates an `iron::response::Response` from something that can become an
/// `HttpApiProblem`.
///
/// If status is `None` `500 - Internal Server Error` is the
/// default.
#[cfg(feature = "with_iron")]
pub fn into_iron_response<T: Into<HttpApiProblem>>(what: T) -> ::iron::response::Response {
    let problem: HttpApiProblem = what.into();
    problem.to_iron_response()
}

#[cfg(feature = "with_iron")]
impl From<HttpApiProblem> for ::iron::response::Response {
    fn from(problem: HttpApiProblem) -> ::iron::response::Response {
        problem.to_iron_response()
    }
}

/// Creates an `hyper::Response` from something that can become an
/// `HttpApiProblem`.
///
/// If status is `None` `500 - Internal Server Error` is the
/// default.
#[cfg(feature = "with_hyper")]
pub fn into_hyper_response<T: Into<HttpApiProblem>>(what: T) -> hyper::Response<hyper::Body> {
    let problem: HttpApiProblem = what.into();
    problem.to_hyper_response()
}

#[cfg(feature = "with_hyper")]
impl From<HttpApiProblem> for hyper::Response<hyper::Body> {
    fn from(problem: HttpApiProblem) -> hyper::Response<hyper::Body> {
        problem.to_hyper_response()
    }
}

/// Creates an `rocket::Response` from something that can become an
/// `HttpApiProblem`.
///
/// If status is `None` `500 - Internal Server Error` is the
/// default.
#[cfg(feature = "with_rocket")]
pub fn into_rocket_response<T: Into<HttpApiProblem>>(what: T) -> ::rocket::Response<'static> {
    let problem: HttpApiProblem = what.into();
    problem.to_rocket_response()
}

#[cfg(feature = "with_rocket")]
impl From<HttpApiProblem> for ::rocket::Response<'static> {
    fn from(problem: HttpApiProblem) -> ::rocket::Response<'static> {
        problem.to_rocket_response()
    }
}

#[cfg(feature = "with_rocket")]
impl<'r> ::rocket::response::Responder<'r> for HttpApiProblem {
    fn respond_to(self, _request: &::rocket::Request) -> Result<::rocket::Response<'r>, ::rocket::http::Status> {
        Ok(self.into())
    }
}

mod custom_http_status_serialization {
    use http::{HttpTryFrom, StatusCode};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<StatusCode>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(ref status_code) = *date {
            return s.serialize_u16(status_code.as_u16());
        }
        s.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<StatusCode>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<u16> = Option::deserialize(deserializer)?;
        if let Some(numeric_status_code) = s {
            // The deserialization should not fail because of an invalid status code.
            let status_code = HttpTryFrom::try_from(numeric_status_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            return Ok(Some(status_code));
        }

        Ok(None)
    }
}

#[cfg(test)]
mod test;
