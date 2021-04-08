//! An error that should be returned from an Http API handler
//!
//! # Things to know
//!
//! `ApiError` can be converted to a `HttpApiProblem`
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::io;

use std::error::Error;

use serde::Serialize;
use serde_json::Value;

use super::*;

pub struct ApiErrorBuilder {
    /// A message that describes the error in a human readable form.
    ///
    /// In an `HttpApiProblem` this becomes the `detail` in most cases.
    ///
    /// If None:
    ///
    /// * If there is a cause, this will become the details
    /// * Otherwise there will be no details
    pub message: Option<String>,
    /// The suggested status code for the server to be returned to the client
    ///
    /// Defaults to 500.
    pub status: StatusCode,
    /// A URL that points to a detailed description of the error. If not
    /// set it will most probably become `httpstatus.es.com/XXX` when
    /// the problem response is generated.
    pub type_url: Option<String>,
    /// Additional JSON encodable information. It is up to the server how and if
    /// it adds the given information.
    pub fields: HashMap<String, Value>,
    /// This is an optional title which can be used to create a valuable output
    /// for consumers.
    pub title: Option<String>,

    pub source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl Default for ApiErrorBuilder {
    fn default() -> Self {
        ApiErrorBuilder {
            message: None,
            status: StatusCode::INTERNAL_SERVER_ERROR,
            type_url: None,
            fields: HashMap::default(),
            title: None,
            source: None,
        }
    }
}

impl ApiErrorBuilder {
    pub fn status<S: Into<StatusCode>>(mut self, status: S) -> Self {
        self.status = status.into();
        self
    }

    pub fn try_status<T: TryInto<StatusCode>, E>(mut self, status: T) -> Result<Self, E>
    where
        E: From<T::Error>,
    {
        self.status = status.try_into()?;
        Ok(self)
    }

    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn message<M: Into<String>>(mut self, message: M) -> Self {
        self.message = Some(message.into());
        self
    }

    pub fn type_url<U: Into<String>>(mut self, type_url: U) -> Self {
        self.type_url = Some(type_url.into());
        self
    }

    /// Adds a serializable field. If the serialization fails nothing will be
    /// added.
    ///
    /// An already present field with the same name will be replaced.
    pub fn field<T: Into<String>, V: Serialize>(mut self, name: T, value: V) -> Self {
        if let Ok(value) = serde_json::to_value(value) {
            self.fields.insert(name.into(), value);
        }

        self
    }

    pub fn source<E: Error + Send + Sync + 'static>(self, cause: E) -> Self {
        self.source_boxed(Box::new(cause))
    }

    pub fn source_boxed<E: Into< Box< dyn Error + Send + Sync + 'static>>>(mut self, cause: E) -> Self {
        self.source = Some(cause.into());
        self
    }

    pub fn finish(self) -> ApiError {
        ApiError {
            status: self.status,
            message: self.message,
            type_url: self.type_url,
            fields: self.fields,
            title: self.title,
            cause: self.source,
        }
    }
}

/// An error that should be returned from an API handler of a web service.
///
/// This should be returned from a handler instead of a premature response
/// or `HttpApiProblem`. This gives the server a chance for better introspection
/// on handler errors and also a chance to create a customized response by himself
///
/// `ApiError` requires the feature "api-error" to be enabled.
#[derive(Debug)]
pub struct ApiError {
    /// A message that describes the error in a human readable form.
    ///
    /// In an `HttpApiProblem` this becomes the `detail` in most cases.
    ///
    /// If None:
    ///
    /// * If there is a cause, this will become the details
    /// * Otherwise there will be no details
    pub message: Option<String>,
    /// The suggested status code for the server to be returned to the client
    pub status: StatusCode,
    /// A URL that points to a detailed description of the error. If not
    /// set it will most probably become `httpstatus.es.com/XXX` when
    /// the problem response is generated.
    pub type_url: Option<String>,
    /// Additional JSON encodable information. It is up to the server how and if
    /// it adds the given information.
    pub fields: HashMap<String, Value>,
    /// This is an optional title which can be used to create a valuable output
    /// for consumers.
    pub title: Option<String>,

    cause: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl ApiError {
    /// Get an [ApiErrorBuilder] with the given [StatusCode] preset.
    pub fn builder(status: StatusCode) -> ApiErrorBuilder {
        ApiErrorBuilder::default().status(status)
    }

    /// Create a new instance with the given [StatusCode]
    pub fn new<S>(status: S) -> Self
    where
        S: Into<StatusCode>,
    {
        Self {
            status: status.into(),
            message: None,
            type_url: None,
            fields: HashMap::new(),
            title: None,
            cause: None,
        }
    }

    /// Create a new instance with the given [StatusCode] and a message
    pub fn with_message<M>(status: StatusCode, message: M) -> Self
    where
        M: Into<String>,
     {
        Self {
            status: status.into(),
            message: Some(message.into()),
            type_url: None,
            fields: HashMap::new(),
            title: None,
            cause: None,
        }
    }


    /// Create a new instance with the given [StatusCode] and a cause
    pub fn with_cause<E>(status: StatusCode, err: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            status,
            message: None,
            type_url: None,
            fields: HashMap::new(),
            title: None,
            cause: Some(Box::new(err)),
        }
    }

    /// Create a new instance with the given [StatusCode], message and a cause
    pub fn with_message_and_cause<M, E>(status: StatusCode, message: M, err: E) -> Self
    where
        M: Into<String>,
        E: Error + Send + Sync + 'static,
    {
        Self {
            status: status,
            message: Some(message.into()),
            type_url: None,
            fields: HashMap::new(),
            title: None,
            cause: Some(Box::new(err)),
        }
    }

    /// Set the [StatusCode] for this error
    pub fn set_cause<E: Error + Send + Sync + 'static>(&mut self, cause: E) {
        self.cause = Some(Box::new(cause))
    }

    /// Adds a serializable field. If the serialization fails nothing will be
    /// added. This method returns `true` if the field was added and `false` if
    /// the field could not be added.
    ///
    /// An already present field with the same name will be replaced.
    pub fn add_field<T: Into<String>, V: Serialize>(&mut self, name: T, value: V) -> bool {
        self.try_add_field(name, value).is_ok()
    }

    /// Adds a serializable field. If the serialization fails nothing will be
    /// added. This fails if a failure occurred while adding the field.
    ///
    /// An already present field with the same name will be replaced.
    pub fn try_add_field<T: Into<String>, V: Serialize>(
        &mut self,
        name: T,
        value: V,
    ) -> Result<(), Box<dyn Error + 'static>> {
        match serde_json::to_value(value) {
            Ok(value) => {
                self.fields.insert(name.into(), value);
                Ok(())
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn display_message(&self) -> Cow<str> {
        if let Some(message) = self.detail_message() {
            return message;
        }

        if let Some(canonical_reason) = self.status.canonical_reason() {
            return Cow::Borrowed(canonical_reason);
        }

        Cow::Borrowed(self.status.as_str())
    }

    /// Creates an [HttpApiProblem] from this.
    ///
    /// Note: If the status is [StatusCode::UNAUTHORIZED] fields will
    /// **not** be put into the problem.
    pub fn to_http_api_problem(&self) -> HttpApiProblem {
        let mut problem = HttpApiProblem::with_title_and_type_from_status(self.status);

        if let Some(detail_message) = self.detail_message() {
            problem.detail = Some(detail_message.to_string())
        }

        if let Some(custom_type_url) = self.type_url.as_ref() {
            problem.type_url = Some(custom_type_url.to_string())
        }

        if self.status != StatusCode::UNAUTHORIZED {
            for (key, value) in self.fields.iter() {
                let _ = problem.set_value(key.to_string(), value);
            }
        }

        if let Some(title) = self.title.as_ref() {
            problem.title = title.to_owned();
        }

        problem
    }

    /// Turns this into an [HttpApiProblem].
    ///
    /// Note: If the status is [StatusCode::UNAUTHORIZED] fields will
    /// **not** be put into the problem.
    pub fn into_http_api_problem(self) -> HttpApiProblem {
        let mut problem = HttpApiProblem::with_title_and_type_from_status(self.status);

        if let Some(detail_message) = self.detail_message() {
            problem.detail = Some(detail_message.to_string())
        }

        if let Some(custom_type_url) = self.type_url {
            problem.type_url = Some(custom_type_url)
        }

        if self.status != StatusCode::UNAUTHORIZED {
            for (key, value) in self.fields.into_iter() {
                let _ = problem.set_value(key, &value);
            }
        }

        if let Some(title) = self.title {
            problem.title = title;
        }

        problem
    }

    fn detail_message(&self) -> Option<Cow<str>> {
        if let Some(message) = self.message.as_ref() {
            return Some(Cow::Borrowed(message));
        }

        if let Some(cause) = self.source() {
            return Some(Cow::Owned(cause.to_string()));
        }

        None
    }

    /// Creates a `hyper` response containing a problem JSON.
    ///
    /// Requires the `hyper` feature
    #[cfg(feature = "hyper")]
    pub fn into_hyper_response(self) -> hyper::Response<hyper::Body> {
        let problem = self.into_http_api_problem();
        problem.to_hyper_response()
    }

    /// Creates a `actix-web` response containing a problem JSON.
    ///
    /// Requires the `actix.web` feature
    #[cfg(feature = "actix-web")]
    pub fn into_actix_web_response(self) -> actix_web::HttpResponse {
        let problem = self.into_http_api_problem();
        problem.into()
    }

    /// Creates a `salvo` response containing a problem JSON.
    ///
    /// Requires the `tide` feature
    #[cfg(feature = "salvo")]
    pub fn into_salvo_response(self) -> salvo::Response {
        let problem = self.into_http_api_problem();
        problem.to_salvo_response()
    }

    /// Creates a `tide` response containing a problem JSON.
    ///
    /// Requires the `tide` feature
    #[cfg(feature = "tide")]
    pub fn into_tide_response(self) -> tide::Response {
        let problem = self.into_http_api_problem();
        problem.to_tide_response()
    }
}

impl Error for ApiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.cause.as_ref().map(|e| &**e as _)
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(detail_message) = self.detail_message() {
            write!(f, "{}: {}", self.status, detail_message)
        } else {
            write!(f, "{}", self.status)
        }
    }
}

impl From<StatusCode> for ApiError {
    fn from(s: StatusCode) -> Self {
        Self::new(s)
    }
}

impl From<ApiError> for HttpApiProblem {
    fn from(error: ApiError) -> Self {
        error.into_http_api_problem()
    }
}

impl From<io::Error> for ApiError {
    fn from(error: io::Error) -> Self {
        ApiError::with_message_and_cause(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal IO error occurred",
            error,
        )
    }
}

#[cfg(feature = "hyper")]
impl From<hyper::Error> for ApiError {
    fn from(error: hyper::Error) -> Self {
        ApiError::with_message_and_cause(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal error caused by hyper occurred",
            error,
        )
    }
}

#[cfg(feature = "hyper")]
impl From<ApiError> for hyper::Response<hyper::Body> {
    fn from(error: ApiError) -> hyper::Response<hyper::Body> {
        error.into_hyper_response()
    }
}

#[cfg(feature = "actix-web")]
impl From<actix::prelude::MailboxError> for ApiError {
    fn from(error: actix::prelude::MailboxError) -> Self {
        ApiError::with_message_and_cause(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal error caused by internal messaging occured",
            error,
        )
    }
}

#[cfg(feature = "actix-web")]
impl From<ApiError> for actix_web::HttpResponse {
    fn from(error: ApiError) -> Self {
        error.into_actix_web_response()
    }
}

#[cfg(feature = "actix-web")]
impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let json = self.to_http_api_problem().json_bytes();
        let actix_status = actix_web::http::StatusCode::from_u16(self.status.as_u16())
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

        actix_web::HttpResponse::build(actix_status)
            .header(
                actix_web::http::header::CONTENT_TYPE,
                PROBLEM_JSON_MEDIA_TYPE,
            )
            .body(json)
    }
}

#[cfg(feature = "warp")]
impl warp::reject::Reject for ApiError {}

#[cfg(feature = "salvo")]
impl From<salvo::Error> for ApiError {
    fn from(error: salvo::Error) -> Self {
        ApiError::with_message_and_cause(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal error caused by salvo occurred",
            error,
        )
    }
}

#[cfg(feature = "salvo")]
impl From<ApiError> for salvo::Response {
    fn from(error: ApiError) -> salvo::Response {
        error.into_salvo_response()
    }
}

#[cfg(feature = "tide")]
impl From<tide::Error> for ApiError {
    fn from(error: tide::Error) -> Self {
        // tide also has its version of status which should always be 
        // convertible without an error.
        let status: StatusCode = u16::from(error.status())
            .try_into()
            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR); 
        ApiError::builder(status)
            .title(status.canonical_reason().unwrap_or("<unknown status code>"))
            .source_boxed(error.into_inner())
            .finish()
    }
}

#[cfg(feature = "tide")]
impl From<ApiError> for tide::Response {
    fn from(error: ApiError) -> tide::Response {
        error.into_tide_response()
    }
}
