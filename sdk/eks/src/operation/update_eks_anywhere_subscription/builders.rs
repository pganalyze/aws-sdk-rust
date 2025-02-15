// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_eks_anywhere_subscription::_update_eks_anywhere_subscription_output::UpdateEksAnywhereSubscriptionOutputBuilder;

pub use crate::operation::update_eks_anywhere_subscription::_update_eks_anywhere_subscription_input::UpdateEksAnywhereSubscriptionInputBuilder;

impl UpdateEksAnywhereSubscriptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_eks_anywhere_subscription();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateEksAnywhereSubscription`.
///
/// <p>Update an EKS Anywhere Subscription. Only auto renewal and tags can be updated after subscription creation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateEksAnywhereSubscriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_eks_anywhere_subscription::builders::UpdateEksAnywhereSubscriptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionOutput,
        crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionError,
    > for UpdateEksAnywhereSubscriptionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionOutput,
            crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateEksAnywhereSubscriptionFluentBuilder {
    /// Creates a new `UpdateEksAnywhereSubscription`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateEksAnywhereSubscription as a reference.
    pub fn as_input(&self) -> &crate::operation::update_eks_anywhere_subscription::builders::UpdateEksAnywhereSubscriptionInputBuilder {
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
        crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscription::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscription::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionOutput,
        crate::operation::update_eks_anywhere_subscription::UpdateEksAnywhereSubscriptionError,
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
    /// <p>The ID of the subscription.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id(input.into());
        self
    }
    /// <p>The ID of the subscription.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id(input);
        self
    }
    /// <p>The ID of the subscription.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id()
    }
    /// <p>A boolean indicating whether or not to automatically renew the subscription.</p>
    pub fn auto_renew(mut self, input: bool) -> Self {
        self.inner = self.inner.auto_renew(input);
        self
    }
    /// <p>A boolean indicating whether or not to automatically renew the subscription.</p>
    pub fn set_auto_renew(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_auto_renew(input);
        self
    }
    /// <p>A boolean indicating whether or not to automatically renew the subscription.</p>
    pub fn get_auto_renew(&self) -> &::std::option::Option<bool> {
        self.inner.get_auto_renew()
    }
    /// <p>Unique, case-sensitive identifier to ensure the idempotency of the request.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_request_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier to ensure the idempotency of the request.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_request_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier to ensure the idempotency of the request.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_request_token()
    }
}
