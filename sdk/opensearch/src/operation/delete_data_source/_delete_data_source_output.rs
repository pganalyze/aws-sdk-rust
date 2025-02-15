// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The result of a <code>GetDataSource</code> operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteDataSourceOutput {
    /// <p>A message associated with the initiated request.</p>
    pub message: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteDataSourceOutput {
    /// <p>A message associated with the initiated request.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DeleteDataSourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteDataSourceOutput {
    /// Creates a new builder-style object to manufacture [`DeleteDataSourceOutput`](crate::operation::delete_data_source::DeleteDataSourceOutput).
    pub fn builder() -> crate::operation::delete_data_source::builders::DeleteDataSourceOutputBuilder {
        crate::operation::delete_data_source::builders::DeleteDataSourceOutputBuilder::default()
    }
}

/// A builder for [`DeleteDataSourceOutput`](crate::operation::delete_data_source::DeleteDataSourceOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteDataSourceOutputBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DeleteDataSourceOutputBuilder {
    /// <p>A message associated with the initiated request.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A message associated with the initiated request.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>A message associated with the initiated request.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteDataSourceOutput`](crate::operation::delete_data_source::DeleteDataSourceOutput).
    pub fn build(self) -> crate::operation::delete_data_source::DeleteDataSourceOutput {
        crate::operation::delete_data_source::DeleteDataSourceOutput {
            message: self.message,
            _request_id: self._request_id,
        }
    }
}
