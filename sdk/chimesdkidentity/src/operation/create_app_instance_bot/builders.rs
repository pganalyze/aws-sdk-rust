// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_app_instance_bot::_create_app_instance_bot_output::CreateAppInstanceBotOutputBuilder;

pub use crate::operation::create_app_instance_bot::_create_app_instance_bot_input::CreateAppInstanceBotInputBuilder;

impl CreateAppInstanceBotInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_app_instance_bot::CreateAppInstanceBotOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_app_instance_bot::CreateAppInstanceBotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_app_instance_bot();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAppInstanceBot`.
///
/// <p>Creates a bot under an Amazon Chime <code>AppInstance</code>. The request consists of a unique <code>Configuration</code> and <code>Name</code> for that bot.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAppInstanceBotFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_app_instance_bot::CreateAppInstanceBotOutput,
        crate::operation::create_app_instance_bot::CreateAppInstanceBotError,
    > for CreateAppInstanceBotFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_app_instance_bot::CreateAppInstanceBotOutput,
            crate::operation::create_app_instance_bot::CreateAppInstanceBotError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateAppInstanceBotFluentBuilder {
    /// Creates a new `CreateAppInstanceBot`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateAppInstanceBot as a reference.
    pub fn as_input(&self) -> &crate::operation::create_app_instance_bot::builders::CreateAppInstanceBotInputBuilder {
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
        crate::operation::create_app_instance_bot::CreateAppInstanceBotOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_app_instance_bot::CreateAppInstanceBotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_app_instance_bot::CreateAppInstanceBot::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_app_instance_bot::CreateAppInstanceBot::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_app_instance_bot::CreateAppInstanceBotOutput,
        crate::operation::create_app_instance_bot::CreateAppInstanceBotError,
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
    /// <p>The ARN of the <code>AppInstance</code> request.</p>
    pub fn app_instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.app_instance_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>AppInstance</code> request.</p>
    pub fn set_app_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_app_instance_arn(input);
        self
    }
    /// <p>The ARN of the <code>AppInstance</code> request.</p>
    pub fn get_app_instance_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_app_instance_arn()
    }
    /// <p>The user's name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The user's name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The user's name.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The request metadata. Limited to a 1KB string in UTF-8.</p>
    pub fn metadata(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metadata(input.into());
        self
    }
    /// <p>The request metadata. Limited to a 1KB string in UTF-8.</p>
    pub fn set_metadata(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_metadata(input);
        self
    }
    /// <p>The request metadata. Limited to a 1KB string in UTF-8.</p>
    pub fn get_metadata(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_metadata()
    }
    /// <p>The unique ID for the client making the request. Use different tokens for different <code>AppInstanceBots</code>.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>The unique ID for the client making the request. Use different tokens for different <code>AppInstanceBots</code>.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>The unique ID for the client making the request. Use different tokens for different <code>AppInstanceBots</code>.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags assigned to the <code>AppInstanceBot</code>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags assigned to the <code>AppInstanceBot</code>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags assigned to the <code>AppInstanceBot</code>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>Configuration information about the Amazon Lex V2 V2 bot.</p>
    pub fn configuration(mut self, input: crate::types::Configuration) -> Self {
        self.inner = self.inner.configuration(input);
        self
    }
    /// <p>Configuration information about the Amazon Lex V2 V2 bot.</p>
    pub fn set_configuration(mut self, input: ::std::option::Option<crate::types::Configuration>) -> Self {
        self.inner = self.inner.set_configuration(input);
        self
    }
    /// <p>Configuration information about the Amazon Lex V2 V2 bot.</p>
    pub fn get_configuration(&self) -> &::std::option::Option<crate::types::Configuration> {
        self.inner.get_configuration()
    }
}
