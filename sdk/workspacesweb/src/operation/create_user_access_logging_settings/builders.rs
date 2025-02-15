// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_user_access_logging_settings::_create_user_access_logging_settings_output::CreateUserAccessLoggingSettingsOutputBuilder;

pub use crate::operation::create_user_access_logging_settings::_create_user_access_logging_settings_input::CreateUserAccessLoggingSettingsInputBuilder;

impl CreateUserAccessLoggingSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_user_access_logging_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateUserAccessLoggingSettings`.
///
/// <p>Creates a user access logging settings resource that can be associated with a web portal.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateUserAccessLoggingSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_user_access_logging_settings::builders::CreateUserAccessLoggingSettingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsOutput,
        crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsError,
    > for CreateUserAccessLoggingSettingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsOutput,
            crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateUserAccessLoggingSettingsFluentBuilder {
    /// Creates a new `CreateUserAccessLoggingSettings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateUserAccessLoggingSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::create_user_access_logging_settings::builders::CreateUserAccessLoggingSettingsInputBuilder {
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
        crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsOutput,
        crate::operation::create_user_access_logging_settings::CreateUserAccessLoggingSettingsError,
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
    /// <p>The ARN of the Kinesis stream.</p>
    pub fn kinesis_stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kinesis_stream_arn(input.into());
        self
    }
    /// <p>The ARN of the Kinesis stream.</p>
    pub fn set_kinesis_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kinesis_stream_arn(input);
        self
    }
    /// <p>The ARN of the Kinesis stream.</p>
    pub fn get_kinesis_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kinesis_stream_arn()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to add to the user settings resource. A tag is a key-value pair.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to add to the user settings resource. A tag is a key-value pair.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to add to the user settings resource. A tag is a key-value pair.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully, subsequent retries with the same client token returns the result from the original successful request. </p>
    /// <p>If you do not specify a client token, one is automatically generated by the AWS SDK.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully, subsequent retries with the same client token returns the result from the original successful request. </p>
    /// <p>If you do not specify a client token, one is automatically generated by the AWS SDK.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully, subsequent retries with the same client token returns the result from the original successful request. </p>
    /// <p>If you do not specify a client token, one is automatically generated by the AWS SDK.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
