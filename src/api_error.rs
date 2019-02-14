//! An error that should be returned from an Http API handler
//!
//! # Purpose
//!
//! # Things to know
//!
//! There are multiple conversions in here:
//!
//! * `ApiError` can be converted to a `HttpApiProblem`
//! * `ApiError` can be converted to a `hyper::Response` containing
use std::collections::HashMap;
use std::fmt;
use std::io;

use failure::*;
use serde_json::Value;

use super::*;

/// An error that should be returned from an API handler of a web service.
///
/// This should be returned from a handler instead of a premature response
/// or `HttpApiProblem`. This gives the server a chance for better introspection
/// on handler errors and also a chance to create a customized response by himself
#[derive(Debug)]
pub struct ApiError {
    inner: Context<ApiErrorData>,
}

impl ApiError {
    pub fn new<S, M>(status: S, message: M) -> Self
    where
        S: Into<StatusCode>,
        M: Into<String>,
    {
        let data = (status, message).into();
        Self {
            inner: Context::new(data),
        }
    }

    pub fn with_data(data: ApiErrorData) -> Self {
        Self {
            inner: Context::new(data),
        }
    }

    pub fn with_failure<S, M, F>(status: S, message: M, err: F) -> Self
    where
        S: Into<StatusCode>,
        M: Into<String>,
        F: Fail,
    {
        (status, message, err).into()
    }

    /// A message to display. The default representation.
    pub fn message(&self) -> &str {
        &self.context().message
    }

    /// The HTTP status this error is associated with.
    pub fn status(&self) -> StatusCode {
        self.context().status.clone()
    }

    /// A URL pointing to a detailed explanation for a better
    /// user experience
    pub fn type_url(&self) -> Option<&str> {
        self.context().type_url.as_ref().map(|s| &**s)
    }

    /// Custom fields associated to this instance of an `ApiError`
    pub fn fields(&self) -> &HashMap<String, Value> {
        &self.context().fields
    }

    /// A short, human-readable summary of the problem
    /// type. It SHOULD NOT change from occurrence to occurrence of the
    /// problem, except for purposes of localization (e.g., using
    /// proactive content negotiation;
    /// see [RFC7231, Section 3.4](https://tools.ietf.org/html/rfc7231#section-3.4).
    pub fn title(&self) -> Option<&str> {
        self.context().title.as_ref().map(|s| &**s)
    }

    pub fn context(&self) -> &ApiErrorData {
        &self.inner.get_context()
    }

    pub fn to_http_api_problem(&self) -> HttpApiProblem {
        self.context().to_http_api_problem()
    }

           #[cfg(feature = "with_hyper")]
    pub fn to_hyper_response(&self) -> hyper::Response<hyper::Body>{
        let problem = self.to_http_api_problem();
        problem.to_hyper_response()
    }


          #[cfg(feature = "with_actix_web")]
    pub fn to_actix_web_response(&self) -> actix_web::HttpResponse{
         let problem = self.to_http_api_problem();
         problem.into()
    }
}

impl Fail for ApiError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }

    fn name(&self) -> Option<&str> {
        Some("http_api_problem::ApiError")
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// Data used to generate an `ApiError`
#[derive(Debug, Fail, Clone)]
#[fail(display = "An error with status code {} occured: {}", status, message)]
pub struct ApiErrorData {
    /// A message that describes the error in a human readable form.
    ///
    /// In an `HttpApiProblem` this becomes the `detail` in most cases.
    pub message: String,
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
}

impl ApiErrorData {
    pub fn new<S, M>(status: S, message: M) -> ApiErrorData
    where
        M: Into<String>,
        S: Into<StatusCode>,
    {
        ApiErrorData {
            status: status.into(),
            message: message.into(),
            type_url: None,
            fields: HashMap::new(),
            title: None,
        }
    }

    /// Add a custom type URL to point to a specific description of an error.
    ///
    /// The intention is to override the default which usually points to
    /// `httpstatus.es/{status_code}` when an `HttpApiProblem` is generated.
    ///
    /// See `HttpApiProblem#type_url`
    pub fn with_type_url<T: Into<String>>(mut self, type_url: T) -> Self {
        self.type_url = Some(type_url.into());
        self
    }

    /// Add a JSON value as a field
    pub fn with_field_value<T: Into<String>>(mut self, name: T, value: Value) -> Self {
        self.fields.insert(name.into(), value);
        self
    }

    /// Adds a serializable field. An error will be returned if adding the
    /// field failed
    ///
    /// An already present field with the same name will be replaced.
    pub fn with_field<T: Into<String>, V: Serialize>(
        mut self,
        name: T,
        value: V,
    ) -> Result<Self, Error> {
        if let Err(err) = self.try_add_field(name, value) {
            Err(err)
        } else {
            Ok(self)
        }
    }

    /// Adds a serializable field. If the serialization fails nothing will be
    /// added.
    ///
    /// An already present field with the same name will be replaced.
    ///
    /// Since this is a convinience builder method is fails silently.
    pub fn with_field_careless<T: Into<String>, V: Serialize>(mut self, name: T, value: V) -> Self {
        self.add_field(name, value);
        self
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
    ) -> Result<(), Error> {
        match serde_json::to_value(value) {
            Ok(value) => {
                self.fields.insert(name.into(), value);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Changes the status of this error
    pub fn change_status<T: Into<StatusCode>>(mut self, new_status: T) -> Self {
        self.status = new_status.into();
        self
    }

    /// Changes the message of this error
    pub fn change_message<T: Into<String>>(mut self, new_message: T) -> Self {
        self.message = new_message.into();
        self
    }

    /// Set the title for the Error
    pub fn with_title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.title = Some(title.into());
        self
    }

    pub fn to_http_api_problem(&self) -> HttpApiProblem {
        let mut problem = HttpApiProblem::with_title_and_type_from_status(self.status)
            .set_detail(self.message.clone());

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


}

impl<S, M, F> From<(S, M, F)> for ApiError
where
    S: Into<StatusCode>,
    M: Into<String>,
    F: Fail,
{
    fn from(v: (S, M, F)) -> Self {
        let (status, message, fail) = v;
        let data = (status, message).into();
        fail.context(data).into()
    }
}

impl From<Context<ApiErrorData>> for ApiError {
    fn from(context: Context<ApiErrorData>) -> Self {
        ApiError { inner: context }
    }
}

impl From<ApiErrorData> for ApiError {
    fn from(error_data: ApiErrorData) -> Self {
        Context::new(error_data).into()
    }
}

impl<S, M> From<(S, M)> for ApiErrorData
where
    S: Into<StatusCode>,
    M: Into<String>,
{
    fn from(v: (S, M)) -> Self {
        let (status, message) = v;
        Self::new(status, message)
    }
}

impl From<ApiError> for HttpApiProblem {
    fn from(error: ApiError) -> Self {
        let mut problem = HttpApiProblem::with_title_and_type_from_status(error.status())
            .set_detail(error.message());

        if let Some(custom_type_url) = error.type_url() {
            problem.type_url = Some(custom_type_url.to_string())
        }

        if error.status() != StatusCode::UNAUTHORIZED {
            for (key, value) in error.fields().iter() {
                let _ = problem.set_value(key.to_string(), value);
            }
        }

        if let Some(title) = error.title() {
            problem.title = title.to_string();
        }

        problem
    }
}

impl From<io::Error> for ApiError {
    fn from(error: io::Error) -> Self {
        let data = ApiErrorData::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal error IO error occurred",
        );
        error.context(data).into()
    }
}

#[cfg(feature = "with_hyper")]
impl From<hyper::error::Error> for ApiError {
    fn from(error: hyper::error::Error) -> Self {
        let data = ApiErrorData::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal error caused by hyper occurred",
        );
        error.context(data).into()
    }
}

#[cfg(feature = "with_actix_web")]
impl From<actix::prelude::MailboxError> for ApiError {
    fn from(error: actix::prelude::MailboxError) -> Self {
        let data = ApiErrorData::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An internal error caused by internal messaging occured",
        );
        error.context(data).into()
    }
}

#[cfg(feature = "with_hyper")]
impl From<ApiError> for hyper::Response<hyper::Body> {
    fn from(error: ApiError) -> hyper::Response<hyper::Body> {
        error.to_hyper_response()
    }
}

#[cfg(feature = "with_actix_web")]
impl From<ApiError> for actix_web::HttpResponse {
    fn from(error: ApiError) -> Self {
        error.to_actix_web_response()
    }
}

#[cfg(feature = "with_actix_web")]
impl actix_web::error::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let json = self.to_http_api_problem().json_bytes();
        let actix_status = actix_web::http::StatusCode::from_u16(self.status().as_u16())
            .unwrap_or(actix_web::http::StatusCode::INTERNAL_SERVER_ERROR);

        let response = actix_web::HttpResponse::build(actix_status)
            .header(
                actix_web::http::header::CONTENT_TYPE,
                PROBLEM_JSON_MEDIA_TYPE,
            )
            .body(json);
        response
    }
}
