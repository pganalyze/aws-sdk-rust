// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_integration_response::_create_integration_response_output::CreateIntegrationResponseOutputBuilder;

pub use crate::operation::create_integration_response::_create_integration_response_input::CreateIntegrationResponseInputBuilder;

impl CreateIntegrationResponseInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_integration_response::CreateIntegrationResponseOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_integration_response::CreateIntegrationResponseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_integration_response();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateIntegrationResponse`.
///
/// <p>Creates an IntegrationResponses.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateIntegrationResponseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_integration_response::builders::CreateIntegrationResponseInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_integration_response::CreateIntegrationResponseOutput,
        crate::operation::create_integration_response::CreateIntegrationResponseError,
    > for CreateIntegrationResponseFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_integration_response::CreateIntegrationResponseOutput,
            crate::operation::create_integration_response::CreateIntegrationResponseError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateIntegrationResponseFluentBuilder {
    /// Creates a new `CreateIntegrationResponse`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateIntegrationResponse as a reference.
    pub fn as_input(&self) -> &crate::operation::create_integration_response::builders::CreateIntegrationResponseInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_integration_response::CreateIntegrationResponseOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_integration_response::CreateIntegrationResponseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_integration_response::CreateIntegrationResponse::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_integration_response::CreateIntegrationResponse::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_integration_response::CreateIntegrationResponseOutput,
        crate::operation::create_integration_response::CreateIntegrationResponseError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The API identifier.</p>
    pub fn api_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.api_id(input.into());
        self
    }
    /// <p>The API identifier.</p>
    pub fn set_api_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_api_id(input);
        self
    }
    /// <p>The API identifier.</p>
    pub fn get_api_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_api_id()
    }
    /// <p>Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p>
    /// <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p>
    /// <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p>
    /// <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    pub fn content_handling_strategy(mut self, input: crate::types::ContentHandlingStrategy) -> Self {
        self.inner = self.inner.content_handling_strategy(input);
        self
    }
    /// <p>Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p>
    /// <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p>
    /// <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p>
    /// <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    pub fn set_content_handling_strategy(mut self, input: ::std::option::Option<crate::types::ContentHandlingStrategy>) -> Self {
        self.inner = self.inner.set_content_handling_strategy(input);
        self
    }
    /// <p>Specifies how to handle response payload content type conversions. Supported values are CONVERT_TO_BINARY and CONVERT_TO_TEXT, with the following behaviors:</p>
    /// <p>CONVERT_TO_BINARY: Converts a response payload from a Base64-encoded string to the corresponding binary blob.</p>
    /// <p>CONVERT_TO_TEXT: Converts a response payload from a binary blob to a Base64-encoded string.</p>
    /// <p>If this property is not defined, the response payload will be passed through from the integration response to the route response or method response without modification.</p>
    pub fn get_content_handling_strategy(&self) -> &::std::option::Option<crate::types::ContentHandlingStrategy> {
        self.inner.get_content_handling_strategy()
    }
    /// <p>The integration ID.</p>
    pub fn integration_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.integration_id(input.into());
        self
    }
    /// <p>The integration ID.</p>
    pub fn set_integration_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_integration_id(input);
        self
    }
    /// <p>The integration ID.</p>
    pub fn get_integration_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_integration_id()
    }
    /// <p>The integration response key.</p>
    pub fn integration_response_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.integration_response_key(input.into());
        self
    }
    /// <p>The integration response key.</p>
    pub fn set_integration_response_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_integration_response_key(input);
        self
    }
    /// <p>The integration response key.</p>
    pub fn get_integration_response_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_integration_response_key()
    }
    /// Adds a key-value pair to `ResponseParameters`.
    ///
    /// To override the contents of this collection use [`set_response_parameters`](Self::set_response_parameters).
    ///
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where {name} is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where {name} is a valid and unique response header name and {JSON-expression} is a valid JSON expression without the $ prefix.</p>
    pub fn response_parameters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.response_parameters(k.into(), v.into());
        self
    }
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where {name} is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where {name} is a valid and unique response header name and {JSON-expression} is a valid JSON expression without the $ prefix.</p>
    pub fn set_response_parameters(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_response_parameters(input);
        self
    }
    /// <p>A key-value map specifying response parameters that are passed to the method response from the backend. The key is a method response header parameter name and the mapped value is an integration response header value, a static value enclosed within a pair of single quotes, or a JSON expression from the integration response body. The mapping key must match the pattern of method.response.header.{name}, where {name} is a valid and unique header name. The mapped non-static value must match the pattern of integration.response.header.{name} or integration.response.body.{JSON-expression}, where {name} is a valid and unique response header name and {JSON-expression} is a valid JSON expression without the $ prefix.</p>
    pub fn get_response_parameters(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_response_parameters()
    }
    /// Adds a key-value pair to `ResponseTemplates`.
    ///
    /// To override the contents of this collection use [`set_response_templates`](Self::set_response_templates).
    ///
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    pub fn response_templates(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.response_templates(k.into(), v.into());
        self
    }
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    pub fn set_response_templates(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_response_templates(input);
        self
    }
    /// <p>The collection of response templates for the integration response as a string-to-string map of key-value pairs. Response templates are represented as a key/value map, with a content-type as the key and a template as the value.</p>
    pub fn get_response_templates(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_response_templates()
    }
    /// <p>The template selection expression for the integration response. Supported only for WebSocket APIs.</p>
    pub fn template_selection_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.template_selection_expression(input.into());
        self
    }
    /// <p>The template selection expression for the integration response. Supported only for WebSocket APIs.</p>
    pub fn set_template_selection_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_template_selection_expression(input);
        self
    }
    /// <p>The template selection expression for the integration response. Supported only for WebSocket APIs.</p>
    pub fn get_template_selection_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_template_selection_expression()
    }
}
