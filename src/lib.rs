//! # HTTP-API-PROBLEM
//!
//! [![crates.io](https://img.shields.io/crates/v/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
//! [![docs.rs](https://docs.rs/http-api-problem/badge.svg)](https://docs.rs/http-api-problem)
//! [![downloads](https://img.shields.io/crates/d/http-api-problem.svg)](https://crates.io/crates/http-api-problem)
//! [![Build Status](https://travis-ci.org/chridou/http-api-problem.svg?branch=master)](https://travis-ci.
//! org/chridou/http-api-problem)
//! [![license-mit](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.
//! com/chridou/http-api-problem/blob/master/LICENSE-MIT)
//! [![license-apache](http://img.shields.io/badge/license-APACHE-blue.svg)]
//! (https://github.com/chridou/http-api-problem/blob/master/LICENSE-APACHE)
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
//! let p = HttpApiProblem::with_title_and_type_from_status(HttpStatusCode::NotFound)
//!     .set_detail("detailed explanation")
//!     .set_instance("/on/1234/do/something");
//!
//! assert_eq!(Some("https://httpstatuses.com/404".to_string()), p.type_url);
//! assert_eq!(Some(HttpStatusCode::NotFound), p.status);
//! assert_eq!("Not Found".to_string(), p.title);
//! assert_eq!(Some("detailed explanation".to_string()), p.detail);
//! assert_eq!(Some("/on/1234/do/something".to_string()), p.instance);
//! ```
//!
//! There is also `From<u16>` implemented for `HttpStatusCode`:
//!
//! ```rust
//! use http_api_problem::*;
//!
//! let p = HttpApiProblem::with_title_and_type_from_status(428)
//!     .set_detail("detailed explanation")
//!     .set_instance("/on/1234/do/something");
//!
//! assert_eq!(Some("https://httpstatuses.com/428".to_string()), p.type_url);
//! assert_eq!(Some(HttpStatusCode::PreconditionRequired), p.status);
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
//! There is a conversion between `iron`s StatusCode and `HttpStatusCode` back
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
//! There is a conversion between `hypers`s StatusCode and `HttpStatusCode`
//! back and forth.
//!
//! The `HttpApiProblem` provides a method `to_hyper_response` which constructs
//! an iron `Response`. If the `status` field of the `HttpApiProblem` is `None`
//! `500 - Internal Server Error` is the default.
//!
//! `From<HttpApiProblem` for `hyper::Response` will also be there. It simply
//! calls `to_hyper_response`.
//!
//! Additionally there will be a function `into_iron_response` which converts
//! anything into a `hyper::Response` that can be converted into a
//! `HttpApiProblem`.
//!
//! ### with_rocket(nightly only)
//!
//! There is a conversion between `rocket`s Status and `HttpStatusCode` back
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
//! * 0.6.1
//!     * Feature `with_hyper` returns response Vec<u8>
//! * 0.6.0
//!     * Feature `with_hyper` uses hyper 0.12
//! * 0.5.3
//!     * Fixed JSON mappings(serde attributes were not respected)
//! * 0.5.2
//!     * Added methods to status code to query its category
//! * 0.5.1
//!     * Support for `Rocket` (contributed by panicbit)
//! * 0.5.0
//!     * Breaking changes, features renamed to `with_iron` and `with_hyper`
//!     * `to_iron_response` now takes a ref insted of `Self`.
//!
//! ## License
//!
//! `http-api-problem` is primarily distributed under the terms of both the MIT
//! license and the Apache License (Version 2.0).
//!
//! Copyright (c) 2017 Christian Douven.
#[macro_use]
extern crate serde;
extern crate serde_json;

#[cfg(feature = "with_iron")]
extern crate iron;

#[cfg(feature = "with_hyper")]
extern crate hyper;

#[cfg(feature = "with_rocket")]
extern crate rocket;

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<HttpStatusCode>,
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
    /// let p = HttpApiProblem::with_title_and_type_from_status(503);
    ///
    /// assert_eq!(Some("https://httpstatuses.com/503".to_string()), p.type_url);
    /// assert_eq!(Some(HttpStatusCode::ServiceUnavailable), p.status);
    /// assert_eq!("Service Unavailable", &p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn with_title_and_type_from_status<T: Into<HttpStatusCode>>(status: T) -> HttpApiProblem {
        let status = status.into();
        HttpApiProblem {
            type_url: Some(format!("https://httpstatuses.com/{}", status.to_u16())),
            status: Some(status.into()),
            title: status.title().to_string(),
            detail: None,
            instance: None,
        }
    }

    /// Creates a new instance with `title` derived from `status`.
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// let p = HttpApiProblem::with_title_from_status(404);
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(Some(HttpStatusCode::NotFound), p.status);
    /// assert_eq!("Not Found", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn with_title_from_status<T: Into<HttpStatusCode>>(status: T) -> HttpApiProblem {
        let status = status.into();
        HttpApiProblem {
            type_url: None,
            status: Some(status.into()),
            title: status.title().to_string(),
            detail: None,
            instance: None,
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
    /// let p = HttpApiProblem::new("Error").set_status(404);
    ///
    /// assert_eq!(None, p.type_url);
    /// assert_eq!(Some(HttpStatusCode::NotFound), p.status);
    /// assert_eq!("Error", p.title);
    /// assert_eq!(None, p.detail);
    /// assert_eq!(None, p.instance);
    /// ```
    pub fn set_status<T: Into<HttpStatusCode>>(self, status: T) -> HttpApiProblem {
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

        let status: Status = self.status.unwrap_or(HttpStatusCode::InternalServerError).into();

        let mut response = Response::with((status, self.json_bytes()));
        let mime: Mime = PROBLEM_JSON_MEDIA_TYPE.parse().unwrap();
        response.headers.set(ContentType(mime));

        response
    }

    /// Creates a `hyper` response.
    ///
    /// If status is `None` `500 - Internal Server Error` is the
    /// default.
    #[cfg(feature = "with_hyper")]
    pub fn to_hyper_response(self) -> hyper::Response<Vec<u8>> {
        use hyper::header::{HeaderValue, CONTENT_LENGTH, CONTENT_TYPE};
        use hyper::StatusCode;
        use hyper::*;

        let status: StatusCode = self.status.unwrap_or(HttpStatusCode::InternalServerError).into();

        let json = self.json_bytes();
        let length = json.len() as u64;

        let builder = Response::builder()
            .status(status)
            .header(CONTENT_TYPE, HeaderValue::from_static(PROBLEM_JSON_MEDIA_TYPE))
            .header(CONTENT_LENGTH, HeaderValue::from_str(&length.to_string()).unwrap())
            .body(json);
        let response: Response<Vec<u8>> = builder.unwrap();

        response
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

        let status: Status = self.status.unwrap_or(HttpStatusCode::InternalServerError).into();

        let content_type: ContentType = PROBLEM_JSON_MEDIA_TYPE.parse().unwrap();
        let json = self.json_bytes();
        let response = Response::build()
            .status(status)
            .sized_body(Cursor::new(json))
            .header(content_type)
            .finalize();

        response
    }
}

impl From<HttpStatusCode> for HttpApiProblem {
    fn from(status: HttpStatusCode) -> HttpApiProblem {
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
pub fn into_hyper_response<T: Into<HttpApiProblem>>(what: T) -> hyper::Response<Vec<u8>> {
    let problem: HttpApiProblem = what.into();
    problem.to_hyper_response()
}

#[cfg(feature = "with_hyper")]
impl From<HttpApiProblem> for hyper::Response<Vec<u8>> {
    fn from(problem: HttpApiProblem) -> hyper::Response<Vec<u8>> {
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

/// An HTTP status code (`status-code` in RFC 7230 et al.).
///
/// This enum contains all common status codes and an Unregistered
/// extension variant. It allows status codes in the range [0, 65535], as any
/// `u16` integer may be used as a status code for XHR requests. It is
/// recommended to only use values between [100, 599], since only these are
/// defined as valid status codes with a status class by HTTP.
///
/// IANA maintain the [Hypertext Transfer Protocol (HTTP) Status Code
/// Registry](http://www.iana.org/assignments/http-status-codes/http-status-codes.xhtml) which is
/// the source for this enum (with one exception, 418 I'm a teapot, which is
/// inexplicably not in the register).
///
/// Shamelessly copied from [iron](http://ironframework.io/).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpStatusCode {
    Continue,
    SwitchingProtocols,
    Processing,
    Ok,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    ImUsed,
    MultipleChoices,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    UseProxy,
    TemporaryRedirect,
    PermanentRedirect,
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    UriTooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,
    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired,
    Unregistered(u16),
}

impl HttpStatusCode {
    ///  `HttpStatusCode` is within 100-199.
    #[inline]
    pub fn is_informational(&self) -> bool {
        self.to_u16() >= 100 && self.to_u16() < 200
    }

    /// `HttpStatusCode` is within 200-299.
    #[inline]
    pub fn is_success(&self) -> bool {
        self.to_u16() >= 200 && self.to_u16() < 300
    }

    /// `HttpStatusCode`  is within 300-399.
    #[inline]
    pub fn is_redirection(&self) -> bool {
        self.to_u16() >= 300 && self.to_u16() < 400
    }

    /// `HttpStatusCode`  is within 400-499.
    #[inline]
    pub fn is_client_error(&self) -> bool {
        self.to_u16() >= 400 && self.to_u16() < 500
    }

    /// `HttpStatusCode`  is within 500-599.
    #[inline]
    pub fn is_server_error(&self) -> bool {
        self.to_u16() >= 500 && self.to_u16() < 600
    }

    /// A descriptive title for the status code which does not contain
    /// the numeric code itself.
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// assert_eq!("Internal Server Error", HttpStatusCode::InternalServerError.title());
    /// ```
    pub fn title(&self) -> &'static str {
        use HttpStatusCode::*;
        match *self {
            Continue => "Continue",
            SwitchingProtocols => "Switching Protocols",
            Processing => "Processing",
            Ok => "Ok",
            Created => "Created",
            Accepted => "Accepted",
            NonAuthoritativeInformation => "Non Authoritative Information",
            NoContent => "No Content",
            ResetContent => "Reset Content",
            PartialContent => "Partial Content",
            MultiStatus => "Multi Status",
            AlreadyReported => "Already Reported",
            ImUsed => "Im Used",
            MultipleChoices => "Multiple Choices",
            MovedPermanently => "Moved Permanently",
            Found => "Found",
            SeeOther => "See Other",
            NotModified => "Not Modified",
            UseProxy => "Use Proxy",
            TemporaryRedirect => "Temporary Redirect",
            PermanentRedirect => "Permanent Redirect",
            BadRequest => "Bad Request",
            Unauthorized => "Unauthorized",
            PaymentRequired => "Payment Required",
            Forbidden => "Forbidden",
            NotFound => "Not Found",
            MethodNotAllowed => "Method Not Allowed",
            NotAcceptable => "Not Acceptable",
            ProxyAuthenticationRequired => "Proxy Authentication Required",
            RequestTimeout => "Request Timeout",
            Conflict => "Conflict",
            Gone => "Gone",
            LengthRequired => "Length Required",
            PreconditionFailed => "Precondition Failed",
            PayloadTooLarge => "Payload Too Large",
            UriTooLong => "Uri Too Long",
            UnsupportedMediaType => "Unsupported Media Type",
            RangeNotSatisfiable => "Range Not Satisfiable",
            ExpectationFailed => "Expectation Failed",
            ImATeapot => "Im A Teapot",
            MisdirectedRequest => "Misdirected Request",
            UnprocessableEntity => "Unprocessable Entity",
            Locked => "Locked",
            FailedDependency => "Failed Dependency",
            UpgradeRequired => "Upgrade Required",
            PreconditionRequired => "Precondition Required",
            TooManyRequests => "Too Many Requests",
            RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            InternalServerError => "Internal Server Error",
            NotImplemented => "Not Implemented",
            BadGateway => "Bad Gateway",
            ServiceUnavailable => "Service Unavailable",
            GatewayTimeout => "Gateway Timeout",
            HttpVersionNotSupported => "HTTP Version Not Supported",
            VariantAlsoNegotiates => "Variant Also Negotiates",
            InsufficientStorage => "Insufficient Storage",
            LoopDetected => "Loop Detected",
            NotExtended => "Not Extended",
            NetworkAuthenticationRequired => "Network Authentication Required",
            Unregistered(code) => {
                if code / 100 == 4 {
                    "<Unregistered Client Error>"
                } else if code / 100 == 5 {
                    "<Unregistered Server Error>"
                } else {
                    "<Unregistered Status Code>"
                }
            }
        }
    }

    /// The numeric status code
    ///
    /// #Example
    ///
    /// ```rust
    /// use http_api_problem::*;
    ///
    /// assert_eq!(500, HttpStatusCode::InternalServerError.to_u16());
    /// ```
    pub fn to_u16(&self) -> u16 {
        use HttpStatusCode::*;
        match *self {
            Continue => 100,
            SwitchingProtocols => 101,
            Processing => 102,
            Ok => 200,
            Created => 201,
            Accepted => 202,
            NonAuthoritativeInformation => 203,
            NoContent => 204,
            ResetContent => 205,
            PartialContent => 206,
            MultiStatus => 207,
            AlreadyReported => 208,
            ImUsed => 226,
            MultipleChoices => 300,
            MovedPermanently => 301,
            Found => 302,
            SeeOther => 303,
            NotModified => 304,
            UseProxy => 305,
            TemporaryRedirect => 307,
            PermanentRedirect => 308,
            BadRequest => 400,
            Unauthorized => 401,
            PaymentRequired => 402,
            Forbidden => 403,
            NotFound => 404,
            MethodNotAllowed => 405,
            NotAcceptable => 406,
            ProxyAuthenticationRequired => 407,
            RequestTimeout => 408,
            Conflict => 409,
            Gone => 410,
            LengthRequired => 411,
            PreconditionFailed => 412,
            PayloadTooLarge => 413,
            UriTooLong => 414,
            UnsupportedMediaType => 415,
            RangeNotSatisfiable => 416,
            ExpectationFailed => 417,
            ImATeapot => 418,
            MisdirectedRequest => 421,
            UnprocessableEntity => 422,
            Locked => 423,
            FailedDependency => 424,
            UpgradeRequired => 426,
            PreconditionRequired => 428,
            TooManyRequests => 429,
            RequestHeaderFieldsTooLarge => 431,
            UnavailableForLegalReasons => 451,
            InternalServerError => 500,
            NotImplemented => 501,
            BadGateway => 502,
            ServiceUnavailable => 503,
            GatewayTimeout => 504,
            HttpVersionNotSupported => 505,
            VariantAlsoNegotiates => 506,
            InsufficientStorage => 507,
            LoopDetected => 508,
            NotExtended => 510,
            NetworkAuthenticationRequired => 511,
            Unregistered(n) => n,
        }
    }
}

impl From<u16> for HttpStatusCode {
    fn from(n: u16) -> HttpStatusCode {
        use HttpStatusCode::*;
        match n {
            100 => Continue,
            101 => SwitchingProtocols,
            102 => Processing,
            200 => Ok,
            201 => Created,
            202 => Accepted,
            203 => NonAuthoritativeInformation,
            204 => NoContent,
            205 => ResetContent,
            206 => PartialContent,
            207 => MultiStatus,
            208 => AlreadyReported,
            226 => ImUsed,
            300 => MultipleChoices,
            301 => MovedPermanently,
            302 => Found,
            303 => SeeOther,
            304 => NotModified,
            305 => UseProxy,
            307 => TemporaryRedirect,
            308 => PermanentRedirect,
            400 => BadRequest,
            401 => Unauthorized,
            402 => PaymentRequired,
            403 => Forbidden,
            404 => NotFound,
            405 => MethodNotAllowed,
            406 => NotAcceptable,
            407 => ProxyAuthenticationRequired,
            408 => RequestTimeout,
            409 => Conflict,
            410 => Gone,
            411 => LengthRequired,
            412 => PreconditionFailed,
            413 => PayloadTooLarge,
            414 => UriTooLong,
            415 => UnsupportedMediaType,
            416 => RangeNotSatisfiable,
            417 => ExpectationFailed,
            418 => ImATeapot,
            421 => MisdirectedRequest,
            422 => UnprocessableEntity,
            423 => Locked,
            424 => FailedDependency,
            426 => UpgradeRequired,
            428 => PreconditionRequired,
            429 => TooManyRequests,
            431 => RequestHeaderFieldsTooLarge,
            451 => UnavailableForLegalReasons,
            500 => InternalServerError,
            501 => NotImplemented,
            502 => BadGateway,
            503 => ServiceUnavailable,
            504 => GatewayTimeout,
            505 => HttpVersionNotSupported,
            506 => VariantAlsoNegotiates,
            507 => InsufficientStorage,
            508 => LoopDetected,
            510 => NotExtended,
            511 => NetworkAuthenticationRequired,
            _ => Unregistered(n),
        }
    }
}

impl fmt::Display for HttpStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.to_u16(), self.title())
    }
}

impl Serialize for HttpStatusCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.to_u16())
    }
}

impl<'de> Deserialize<'de> for HttpStatusCode {
    fn deserialize<D>(deserializer: D) -> Result<HttpStatusCode, D::Error>
    where
        D: Deserializer<'de>,
    {
        u16::deserialize(deserializer).map(Into::into)
    }
}

#[cfg(feature = "with_hyper")]
impl From<::hyper::StatusCode> for HttpStatusCode {
    fn from(hyper_status: hyper::StatusCode) -> HttpStatusCode {
        hyper_status.as_u16().into()
    }
}

#[cfg(feature = "with_hyper")]
impl From<HttpStatusCode> for ::hyper::StatusCode {
    fn from(status: HttpStatusCode) -> ::hyper::StatusCode {
        ::hyper::StatusCode::from_u16(status.to_u16()).unwrap_or_else(|_| ::hyper::StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[cfg(feature = "with_iron")]
impl From<::iron::status::Status> for HttpStatusCode {
    fn from(iron_status: ::iron::status::Status) -> HttpStatusCode {
        iron_status.to_u16().into()
    }
}

#[cfg(feature = "with_iron")]
impl From<HttpStatusCode> for ::iron::status::Status {
    fn from(status: HttpStatusCode) -> ::iron::status::Status {
        ::iron::status::Status::from_u16(status.to_u16())
    }
}

#[cfg(feature = "with_rocket")]
impl From<::rocket::http::Status> for HttpStatusCode {
    fn from(rocket_status: ::rocket::http::Status) -> HttpStatusCode {
        rocket_status.code.into()
    }
}

#[cfg(feature = "with_rocket")]
impl From<HttpStatusCode> for ::rocket::http::Status {
    fn from(status: HttpStatusCode) -> ::rocket::http::Status {
        use rocket::http::Status;

        let code = status.to_u16();
        Status::from_code(code).unwrap_or(Status::new(code, ""))
    }
}
