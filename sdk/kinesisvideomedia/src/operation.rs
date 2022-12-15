// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `GetMedia`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_media`](crate::client::Client::get_media).
///
/// See [`crate::client::fluent_builders::GetMedia`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetMedia {
    _private: (),
}
impl GetMedia {
    /// Creates a new builder-style object to manufacture [`GetMediaInput`](crate::input::GetMediaInput).
    pub fn builder() -> crate::input::get_media_input::Builder {
        crate::input::get_media_input::Builder::default()
    }
    /// Creates a new `GetMedia` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for GetMedia {
    type Output = std::result::Result<crate::output::GetMediaOutput, crate::error::GetMediaError>;
    fn parse_unloaded(
        &self,
        response: &mut aws_smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_get_media(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_get_media_error(response)
    }
}

/// Operation customization and supporting types
pub mod customize;
