// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::release_sender_id::_release_sender_id_output::ReleaseSenderIdOutputBuilder;

pub use crate::operation::release_sender_id::_release_sender_id_input::ReleaseSenderIdInputBuilder;

impl ReleaseSenderIdInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::release_sender_id::ReleaseSenderIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::release_sender_id::ReleaseSenderIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.release_sender_id();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ReleaseSenderId`.
///
/// <p>Releases an existing sender ID in your account.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ReleaseSenderIdFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::release_sender_id::builders::ReleaseSenderIdInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::release_sender_id::ReleaseSenderIdOutput,
        crate::operation::release_sender_id::ReleaseSenderIdError,
    > for ReleaseSenderIdFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::release_sender_id::ReleaseSenderIdOutput,
            crate::operation::release_sender_id::ReleaseSenderIdError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ReleaseSenderIdFluentBuilder {
    /// Creates a new `ReleaseSenderId`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ReleaseSenderId as a reference.
    pub fn as_input(&self) -> &crate::operation::release_sender_id::builders::ReleaseSenderIdInputBuilder {
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
        crate::operation::release_sender_id::ReleaseSenderIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::release_sender_id::ReleaseSenderIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::release_sender_id::ReleaseSenderId::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::release_sender_id::ReleaseSenderId::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::release_sender_id::ReleaseSenderIdOutput,
        crate::operation::release_sender_id::ReleaseSenderIdError,
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
    /// <p>The sender ID to release.</p>
    pub fn sender_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sender_id(input.into());
        self
    }
    /// <p>The sender ID to release.</p>
    pub fn set_sender_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sender_id(input);
        self
    }
    /// <p>The sender ID to release.</p>
    pub fn get_sender_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sender_id()
    }
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.</p>
    pub fn iso_country_code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.iso_country_code(input.into());
        self
    }
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.</p>
    pub fn set_iso_country_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_iso_country_code(input);
        self
    }
    /// <p>The two-character code, in ISO 3166-1 alpha-2 format, for the country or region.</p>
    pub fn get_iso_country_code(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_iso_country_code()
    }
}
