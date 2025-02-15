// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_lag::_update_lag_output::UpdateLagOutputBuilder;

pub use crate::operation::update_lag::_update_lag_input::UpdateLagInputBuilder;

impl UpdateLagInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_lag::UpdateLagOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_lag::UpdateLagError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_lag();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateLag`.
///
/// <p>Updates the attributes of the specified link aggregation group (LAG).</p>
/// <p>You can update the following LAG attributes:</p>
/// <ul>
/// <li> <p>The name of the LAG.</p> </li>
/// <li> <p>The value for the minimum number of connections that must be operational for the LAG itself to be operational. </p> </li>
/// <li> <p>The LAG's MACsec encryption mode.</p> <p>Amazon Web Services assigns this value to each connection which is part of the LAG.</p> </li>
/// <li> <p>The tags</p> </li>
/// </ul> <note>
/// <p>If you adjust the threshold value for the minimum number of operational connections, ensure that the new value does not cause the LAG to fall below the threshold and become non-operational.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLagFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_lag::builders::UpdateLagInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::update_lag::UpdateLagOutput, crate::operation::update_lag::UpdateLagError>
    for UpdateLagFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::update_lag::UpdateLagOutput, crate::operation::update_lag::UpdateLagError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateLagFluentBuilder {
    /// Creates a new `UpdateLag`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateLag as a reference.
    pub fn as_input(&self) -> &crate::operation::update_lag::builders::UpdateLagInputBuilder {
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
        crate::operation::update_lag::UpdateLagOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_lag::UpdateLagError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_lag::UpdateLag::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_lag::UpdateLag::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_lag::UpdateLagOutput,
        crate::operation::update_lag::UpdateLagError,
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
    /// <p>The ID of the LAG.</p>
    pub fn lag_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lag_id(input.into());
        self
    }
    /// <p>The ID of the LAG.</p>
    pub fn set_lag_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lag_id(input);
        self
    }
    /// <p>The ID of the LAG.</p>
    pub fn get_lag_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lag_id()
    }
    /// <p>The name of the LAG.</p>
    pub fn lag_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.lag_name(input.into());
        self
    }
    /// <p>The name of the LAG.</p>
    pub fn set_lag_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_lag_name(input);
        self
    }
    /// <p>The name of the LAG.</p>
    pub fn get_lag_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_lag_name()
    }
    /// <p>The minimum number of physical connections that must be operational for the LAG itself to be operational.</p>
    pub fn minimum_links(mut self, input: i32) -> Self {
        self.inner = self.inner.minimum_links(input);
        self
    }
    /// <p>The minimum number of physical connections that must be operational for the LAG itself to be operational.</p>
    pub fn set_minimum_links(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_minimum_links(input);
        self
    }
    /// <p>The minimum number of physical connections that must be operational for the LAG itself to be operational.</p>
    pub fn get_minimum_links(&self) -> &::std::option::Option<i32> {
        self.inner.get_minimum_links()
    }
    /// <p>The LAG MAC Security (MACsec) encryption mode.</p>
    /// <p>Amazon Web Services applies the value to all connections which are part of the LAG.</p>
    pub fn encryption_mode(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.encryption_mode(input.into());
        self
    }
    /// <p>The LAG MAC Security (MACsec) encryption mode.</p>
    /// <p>Amazon Web Services applies the value to all connections which are part of the LAG.</p>
    pub fn set_encryption_mode(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_encryption_mode(input);
        self
    }
    /// <p>The LAG MAC Security (MACsec) encryption mode.</p>
    /// <p>Amazon Web Services applies the value to all connections which are part of the LAG.</p>
    pub fn get_encryption_mode(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_encryption_mode()
    }
}
