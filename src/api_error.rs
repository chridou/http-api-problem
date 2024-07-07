//! An error that should be returned from an HTTP API handler.
//!
//! It is able to carry typed data via [Extensions] to be used
//! in middlewares. These values will not become part of any response.
//!
//! # Things to know
//!
//! [ApiError] can be converted to an [HttpApiProblem] and
//! also has many conversions to responses of web framewors implemented.
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::{self, Display};
use std::io;

use std::error::Error;

use http::Extensions;
use serde::Serialize;
use serde_json::Value;

use super::*;
pub use http_api_problem_derive::IntoApiError;

pub struct ApiErrorBuilder {
    /// The suggested status code for the server to be returned to the client
    pub status: StatusCode,

    /// This is an optional title which can be used to create a valuable output
    /// for consumers.
    pub title: Option<String>,

    /// A message that describes the error in a human readable form.
    ///
    /// In an [HttpApiProblem] this becomes the `detail` in most cases.
    pub message: Option<String>,

    /// A URL that points to a detailed description of the error.
    pub type_url: Option<String>,

    /// A URI reference that identifies the specific
    /// occurrence of the problem.  It may or may not yield further
    /// information if dereferenced.
    pub instance: Option<String>,

    /// Additional JSON encodable information. It is up to the server how and if
    /// it adds the given information.
    pub fields: HashMap<String, Value>,

    /// Typed extensions for carrying processable data server side
    ///
    /// Can be used e.g. for middlewares
    ///
    /// Extensions will not be part of an [HttpApiProblem]
    pub extensions: Extensions,

    pub source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl ApiErrorBuilder {
    /// Set the [StatusCode]
    pub fn status<T: Into<StatusCode>>(mut self, status: T) -> Self {
        self.status = status.into();
        self
    }

    /// Try to set the [StatusCode]
    ///
    /// Fails if the `status` argument can not be converted to a [StatusCode]
    pub fn try_status<T: TryInto<StatusCode>>(self, status: T) -> Result<Self, InvalidStatusCode>
    where
        T::Error: Into<InvalidStatusCode>,
    {
        let status = status.try_into().map_err(|e| e.into())?;
        Ok(self.status(status))
    }

    /// This is an optional title which can be used to create a valuable output
    /// for consumers.
    pub fn title<T: Display>(mut self, title: T) -> Self {
        self.title = Some(title.to_string());
        self
    }

    /// A message that describes the error in a human readable form.
    ///
    /// In an [HttpApiProblem] this becomes the `detail` in most cases.
    pub fn message<M: Display>(mut self, message: M) -> Self {
        self.message = Some(message.to_string());
        self
    }

    /// A URL that points to a detailed description of the error.
    pub fn type_url<U: Display>(mut self, type_url: U) -> Self {
        self.type_url = Some(type_url.to_string());
        self
    }

    /// Sets the `instance`
    ///
    /// A URI reference that identifies the specific
    /// occurrence of the problem.  It may or may not yield further
    /// information if dereferenced.
    pub fn instance<T: Display>(mut self, instance: T) -> Self {
        self.instance = Some(instance.to_string());
        self
    }

    /// Adds a serializable field.
    ///
    /// If the serialization fails nothing will be added.
    /// An already present field with the same name will be replaced.
    pub fn field<T: Into<String>, V: Serialize>(mut self, name: T, value: V) -> Self {
        if let Ok(value) = serde_json::to_value(value) {
            self.fields.insert(name.into(), value);
        }

        self
    }

    /// Modify the fields values from within a closure
    pub fn with_fields<F>(mut self, f: F) -> Self
    where
        F: FnOnce(HashMap<String, Value>) -> HashMap<String, Value>,
    {
        self.fields = f(self.fields);

        self
    }

    /// Adds an extension value.
    ///
    /// Existing values will be overwritten
    ///
    /// Extensions will not be part of an [HttpApiProblem]
    pub fn extension<T: Send + Sync + Clone + 'static>(mut self, val: T) -> Self {
        let _ = self.extensions.insert(val);

        self
    }

    /// Modify the extension values from within a closure
    ///
    /// Extensions will not be part of an [HttpApiProblem]
    pub fn with_extensions<F>(mut self, f: F) -> Self
    where
        F: FnOnce(Extensions) -> Extensions,
    {
        self.extensions = f(self.extensions);

        self
    }

    pub fn source<E: Error + Send + Sync + 'static>(self, source: E) -> Self {
        self.source_in_a_box(Box::new(source))
    }

    pub fn source_in_a_box<E: Into<Box<dyn Error + Send + Sync + 'static>>>(
        mut self,
        source: E,
    ) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Build the [ApiError]
    pub fn finish(self) -> ApiError {
        ApiError {
            status: self.status,
            title: self.title,
            message: self.message,
            type_url: self.type_url,
            instance: self.instance,
            fields: self.fields,
            extensions: self.extensions,
            source: self.source,
        }
    }
}

/// An error that should be returned from an API handler of a web service.
///
/// This should be returned from a handler as an error instead of a response
/// or [HttpApiProblem]. Allows for logging etc. right before a response is generated.
///
/// Advantage over using an [HttpApiProblem] directly are that the [StatusCode] is
/// mandatory and that this struct can also capture a source error
/// which the [HttpApiProblem] does not since no error chains
/// should be transmitted to clients.
///
/// # Message on Display and converting to HttpApiProblem
///
/// When [Display::fmt] is invoked or when the details field of an [HttpApiProblem]
/// is filled, the `message` field is used if present. If no `message` is set
/// but there is a `source` error set, `to_string()` of the source will
/// be used instead. Otherwise nothing will be displayed or set.
///
/// `ApiError` requires the feature `api-error` to be enabled.
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    title: Option<String>,
    message: Option<String>,
    instance: Option<String>,
    type_url: Option<String>,
    fields: HashMap<String, Value>,
    extensions: Extensions,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl ApiError {
    /// Get an [ApiErrorBuilder] with the given [StatusCode] preset.
    pub fn builder<T: Into<StatusCode>>(status: T) -> ApiErrorBuilder {
        ApiErrorBuilder {
            status: status.into(),
            title: None,
            message: None,
            type_url: None,
            instance: None,
            fields: HashMap::default(),
            source: None,
            extensions: Extensions::default(),
        }
    }

    /// Try to get an [ApiErrorBuilder] with the given [StatusCode] preset.
    ///
    /// Fails if the `status` argument can not be converted to a [StatusCode]
    pub fn try_builder<S: TryInto<StatusCode>>(
        status: S,
    ) -> Result<ApiErrorBuilder, InvalidStatusCode>
    where
        S::Error: Into<InvalidStatusCode>,
    {
        let status = status.try_into().map_err(|e| e.into())?;
        Ok(Self::builder(status))
    }

    /// Create a new instance with the given [StatusCode]
    pub fn new<T: Into<StatusCode>>(status: T) -> Self {
        Self {
            status: status.into(),
            title: None,
            message: None,
            type_url: None,
            instance: None,
            fields: HashMap::new(),
            extensions: Extensions::default(),
            source: None,
        }
    }

    /// Try to create a new instance with the given [StatusCode]
    ///
    /// Fails if the `status` argument can not be converted to a [StatusCode]
    pub fn try_new<S: TryInto<StatusCode>>(status: S) -> Result<Self, InvalidStatusCode>
    where
        S::Error: Into<InvalidStatusCode>,
    {
        let status = status.try_into().map_err(|e| e.into())?;
        Ok(Self::new(status))
    }

    /// Set the [StatusCode].
    pub fn set_status<T: Into<StatusCode>>(&mut self, status: T) {
        self.status = status.into();
    }

    /// Get the [StatusCode].
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// This is an optional title which can be used to create a valuable output
    /// for consumers.
    pub fn set_title<T: Display>(&mut self, title: T) {
        self.title = Some(title.to_string())
    }

    /// This is an optional title which can be used to create a valuable output
    /// for consumers.
    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    /// Set a message that describes the error in a human readable form.
    pub fn set_message<T: Display>(&mut self, message: T) {
        self.message = Some(message.to_string())
    }

    /// A message that describes the error in a human readable form.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Set a URL that points to a detailed description of the error.
    ///
    /// If not set it will most probably become `httpstatus.es.com/XXX` when
    /// the problem response is generated.
    pub fn set_type_url<T: Display>(&mut self, type_url: T) {
        self.type_url = Some(type_url.to_string())
    }

    /// A URL that points to a detailed description of the error.
    pub fn type_url(&self) -> Option<&str> {
        self.type_url.as_deref()
    }

    pub fn set_instance<T: Display>(&mut self, instance: T) {
        self.instance = Some(instance.to_string())
    }

    /// A URL that points to a detailed description of the error.
    pub fn instance(&self) -> Option<&str> {
        self.instance.as_deref()
    }

    pub fn set_source<E: Error + Send + Sync + 'static>(&mut self, source: E) {
        self.set_source_in_a_box(Box::new(source))
    }

    pub fn set_source_in_a_box<E: Into<Box<dyn Error + Send + Sync + 'static>>>(
        &mut self,
        source: E,
    ) {
        self.source = Some(source.into());
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
        let name: String = name.into();

        match name.as_ref() {
            "type" => return Err("'type' is a reserved field name".into()),
            "status" => return Err("'status' is a reserved field name".into()),
            "title" => return Err("'title' is a reserved field name".into()),
            "detail" => return Err("'detail' is a reserved field name".into()),
            "instance" => return Err("'instance' is a reserved field name".into()),
            _ => (),
        }

        match serde_json::to_value(value) {
            Ok(value) => {
                self.fields.insert(name, value);
                Ok(())
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    /// Returns a reference to the serialized fields
    pub fn fields(&self) -> &HashMap<String, Value> {
        &self.fields
    }

    /// Returns a mutable reference to the serialized fields
    pub fn fields_mut(&mut self) -> &mut HashMap<String, Value> {
        &mut self.fields
    }

    /// Get a reference to the extensions
    ///
    /// Extensions will not be part of an [HttpApiProblem]
    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    /// Get a mutable reference to the extensions
    ///
    /// Extensions will not be part of an [HttpApiProblem]
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }

    /// Creates an [HttpApiProblem] from this.
    ///
    /// Note: If the status is [StatusCode]::UNAUTHORIZED fields will
    /// **not** be put into the problem.
    pub fn to_http_api_problem(&self) -> HttpApiProblem {
        let mut problem = HttpApiProblem::with_title_and_type(self.status);

        problem.title.clone_from(&self.title);

        if let Some(message) = self.detail_message() {
            problem.detail = Some(message.into())
        }

        problem.type_url.clone_from(&self.type_url);
        problem.instance.clone_from(&self.instance);

        if self.status != StatusCode::UNAUTHORIZED {
            for (key, value) in self.fields.iter() {
                problem.set_value(key.to_string(), value);
            }
        }

        problem
    }

    /// Turns this into an [HttpApiProblem].
    ///
    /// Note: If the status is [StatusCode]::UNAUTHORIZED fields will
    /// **not** be put into the problem.
    pub fn into_http_api_problem(self) -> HttpApiProblem {
        let mut problem = HttpApiProblem::with_title_and_type(self.status);

        if let Some(title) = self.title.as_ref() {
            problem.title = Some(title.to_owned());
        }

        if let Some(message) = self.detail_message() {
            problem.detail = Some(message.into())
        }

        if let Some(type_url) = self.type_url.as_ref() {
            problem.type_url = Some(type_url.to_owned())
        }

        if let Some(instance) = self.instance.as_ref() {
            problem.instance = Some(instance.to_owned())
        }

        if self.status != StatusCode::UNAUTHORIZED {
            for (key, value) in self.fields.iter() {
                problem.set_value(key.to_string(), value);
            }
        }

        problem
    }

    /// If there is a message it will be the message otherwise the source error stringified
    ///
    /// If none is present, `None` is returned
    pub fn detail_message(&self) -> Option<Cow<str>> {
        if let Some(message) = self.message.as_ref() {
            return Some(Cow::Borrowed(message));
        }

        if let Some(source) = self.source() {
            return Some(Cow::Owned(source.to_string()));
        }

        None
    }

    /// Creates a [hyper] response containing a problem JSON.
    ///
    /// Requires the `hyper` feature
    #[cfg(feature = "hyper")]
    pub fn into_hyper_response(self) -> hyper::Response<String> {
        let problem = self.into_http_api_problem();
        problem.to_hyper_response()
    }

    /// Creates an axum [Response](axum_core::response::response) containing a problem JSON.
    ///
    /// Requires the `axum` feature
    #[cfg(feature = "axum")]
    pub fn into_axum_response(self) -> axum_core::response::Response {
        let problem = self.into_http_api_problem();
        problem.to_axum_response()
    }

    /// Creates a `actix-web` response containing a problem JSON.
    ///
    /// Requires the `actix.web` feature
    #[cfg(feature = "actix-web")]
    pub fn into_actix_web_response(self) -> actix_web::HttpResponse {
        let problem = self.into_http_api_problem();
        problem.into()
    }

    /// Creates a [salvo] response containing a problem JSON.
    ///
    /// Requires the `salvo` feature
    #[cfg(feature = "salvo")]
    pub fn into_salvo_response(self) -> salvo::Response {
        let problem = self.into_http_api_problem();
        problem.to_salvo_response()
    }

    /// Creates a [tide] response containing a problem JSON.
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
        self.source.as_ref().map(|e| &**e as _)
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.status)?;

        match (self.title.as_ref(), self.detail_message()) {
            (Some(title), Some(detail)) => return write!(f, " - {} - {}", title, detail),
            (Some(title), None) => return write!(f, " - {}", title),
            (None, Some(detail)) => return write!(f, " - {}", detail),
            (None, None) => (),
        }

        if let Some(type_url) = self.type_url.as_ref() {
            return write!(f, " of type {}", type_url);
        }

        if let Some(instance) = self.instance.as_ref() {
            return write!(f, " on {}", instance);
        }

        Ok(())
    }
}

impl From<StatusCode> for ApiError {
    fn from(s: StatusCode) -> Self {
        Self::new(s)
    }
}

impl From<ApiErrorBuilder> for ApiError {
    fn from(builder: ApiErrorBuilder) -> Self {
        builder.finish()
    }
}

impl From<ApiError> for HttpApiProblem {
    fn from(error: ApiError) -> Self {
        error.into_http_api_problem()
    }
}

impl From<io::Error> for ApiError {
    fn from(error: io::Error) -> Self {
        ApiError::builder(StatusCode::INTERNAL_SERVER_ERROR)
            .title("An IO error occurred")
            .source(error)
            .finish()
    }
}

impl From<std::convert::Infallible> for ApiError {
    fn from(error: std::convert::Infallible) -> Self {
        match error {}
    }
}

pub trait IntoApiError {
    fn into_api_error(self) -> ApiError;
}

impl<T: IntoApiError> From<T> for ApiError {
    fn from(t: T) -> ApiError {
        t.into_api_error()
    }
}

#[cfg(feature = "hyper")]
impl From<hyper::Error> for ApiError {
    fn from(error: hyper::Error) -> Self {
        ApiError::builder(StatusCode::INTERNAL_SERVER_ERROR)
            .source(error)
            .finish()
    }
}

#[cfg(feature = "hyper")]
impl From<ApiError> for hyper::Response<String> {
    fn from(error: ApiError) -> hyper::Response<String> {
        error.into_hyper_response()
    }
}

#[cfg(feature = "axum")]
impl From<ApiError> for axum_core::response::Response {
    fn from(error: ApiError) -> axum_core::response::Response {
        error.into_axum_response()
    }
}

#[cfg(feature = "axum")]
impl axum_core::response::IntoResponse for ApiError {
    fn into_response(self) -> axum_core::response::Response {
        self.into()
    }
}

#[cfg(feature = "actix-web")]
impl From<actix::prelude::MailboxError> for ApiError {
    fn from(error: actix::prelude::MailboxError) -> Self {
        ApiError::builder(StatusCode::INTERNAL_SERVER_ERROR)
            .source(error)
            .finish()
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
            .append_header((
                actix_web::http::header::CONTENT_TYPE,
                PROBLEM_JSON_MEDIA_TYPE,
            ))
            .body(json)
    }
}

#[cfg(feature = "warp")]
impl warp::reject::Reject for ApiError {}

#[cfg(feature = "salvo")]
impl From<salvo::Error> for ApiError {
    fn from(error: salvo::Error) -> Self {
        ApiError::builder(StatusCode::INTERNAL_SERVER_ERROR)
            .source(error)
            .finish()
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
            .source_in_a_box(error.into_inner())
            .finish()
    }
}

#[cfg(feature = "tide")]
impl From<ApiError> for tide::Response {
    fn from(error: ApiError) -> tide::Response {
        error.into_tide_response()
    }
}
