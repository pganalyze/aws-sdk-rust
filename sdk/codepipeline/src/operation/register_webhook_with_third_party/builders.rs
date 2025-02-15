// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_webhook_with_third_party::_register_webhook_with_third_party_output::RegisterWebhookWithThirdPartyOutputBuilder;

pub use crate::operation::register_webhook_with_third_party::_register_webhook_with_third_party_input::RegisterWebhookWithThirdPartyInputBuilder;

impl RegisterWebhookWithThirdPartyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.register_webhook_with_third_party();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RegisterWebhookWithThirdParty`.
///
/// <p>Configures a connection between the webhook that was created and the external tool with events to be detected.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterWebhookWithThirdPartyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_webhook_with_third_party::builders::RegisterWebhookWithThirdPartyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyOutput,
        crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError,
    > for RegisterWebhookWithThirdPartyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyOutput,
            crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RegisterWebhookWithThirdPartyFluentBuilder {
    /// Creates a new `RegisterWebhookWithThirdParty`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RegisterWebhookWithThirdParty as a reference.
    pub fn as_input(&self) -> &crate::operation::register_webhook_with_third_party::builders::RegisterWebhookWithThirdPartyInputBuilder {
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
        crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdParty::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdParty::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyOutput,
        crate::operation::register_webhook_with_third_party::RegisterWebhookWithThirdPartyError,
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
    /// <p>The name of an existing webhook created with PutWebhook to register with a supported third party. </p>
    pub fn webhook_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.webhook_name(input.into());
        self
    }
    /// <p>The name of an existing webhook created with PutWebhook to register with a supported third party. </p>
    pub fn set_webhook_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_webhook_name(input);
        self
    }
    /// <p>The name of an existing webhook created with PutWebhook to register with a supported third party. </p>
    pub fn get_webhook_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_webhook_name()
    }
}
