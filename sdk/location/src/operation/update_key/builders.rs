// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_key::_update_key_output::UpdateKeyOutputBuilder;

pub use crate::operation::update_key::_update_key_input::UpdateKeyInputBuilder;

impl UpdateKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_key::UpdateKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_key::UpdateKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateKey`.
///
/// <p>Updates the specified properties of a given API key resource.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateKeyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_key::builders::UpdateKeyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::update_key::UpdateKeyOutput, crate::operation::update_key::UpdateKeyError>
    for UpdateKeyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::update_key::UpdateKeyOutput, crate::operation::update_key::UpdateKeyError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateKeyFluentBuilder {
    /// Creates a new `UpdateKey`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateKey as a reference.
    pub fn as_input(&self) -> &crate::operation::update_key::builders::UpdateKeyInputBuilder {
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
        crate::operation::update_key::UpdateKeyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_key::UpdateKeyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_key::UpdateKey::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_key::UpdateKey::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_key::UpdateKeyOutput,
        crate::operation::update_key::UpdateKeyError,
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
    /// <p>The name of the API key resource to update.</p>
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.key_name(input.into());
        self
    }
    /// <p>The name of the API key resource to update.</p>
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_key_name(input);
        self
    }
    /// <p>The name of the API key resource to update.</p>
    pub fn get_key_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_key_name()
    }
    /// <p>Updates the description for the API key resource.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>Updates the description for the API key resource.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>Updates the description for the API key resource.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Updates the timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    pub fn expire_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.expire_time(input);
        self
    }
    /// <p>Updates the timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    pub fn set_expire_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_expire_time(input);
        self
    }
    /// <p>Updates the timestamp for when the API key resource will expire in <a href="https://www.iso.org/iso-8601-date-and-time-format.html"> ISO 8601</a> format: <code>YYYY-MM-DDThh:mm:ss.sssZ</code>. </p>
    pub fn get_expire_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_expire_time()
    }
    /// <p>Whether the API key should expire. Set to <code>true</code> to set the API key to have no expiration time.</p>
    pub fn no_expiry(mut self, input: bool) -> Self {
        self.inner = self.inner.no_expiry(input);
        self
    }
    /// <p>Whether the API key should expire. Set to <code>true</code> to set the API key to have no expiration time.</p>
    pub fn set_no_expiry(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_no_expiry(input);
        self
    }
    /// <p>Whether the API key should expire. Set to <code>true</code> to set the API key to have no expiration time.</p>
    pub fn get_no_expiry(&self) -> &::std::option::Option<bool> {
        self.inner.get_no_expiry()
    }
    /// <p>The boolean flag to be included for updating <code>ExpireTime</code> or <code>Restrictions</code> details.</p>
    /// <p>Must be set to <code>true</code> to update an API key resource that has been used in the past 7 days.</p>
    /// <p> <code>False</code> if force update is not preferred</p>
    /// <p>Default value: <code>False</code> </p>
    pub fn force_update(mut self, input: bool) -> Self {
        self.inner = self.inner.force_update(input);
        self
    }
    /// <p>The boolean flag to be included for updating <code>ExpireTime</code> or <code>Restrictions</code> details.</p>
    /// <p>Must be set to <code>true</code> to update an API key resource that has been used in the past 7 days.</p>
    /// <p> <code>False</code> if force update is not preferred</p>
    /// <p>Default value: <code>False</code> </p>
    pub fn set_force_update(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_update(input);
        self
    }
    /// <p>The boolean flag to be included for updating <code>ExpireTime</code> or <code>Restrictions</code> details.</p>
    /// <p>Must be set to <code>true</code> to update an API key resource that has been used in the past 7 days.</p>
    /// <p> <code>False</code> if force update is not preferred</p>
    /// <p>Default value: <code>False</code> </p>
    pub fn get_force_update(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_update()
    }
    /// <p>Updates the API key restrictions for the API key resource.</p>
    pub fn restrictions(mut self, input: crate::types::ApiKeyRestrictions) -> Self {
        self.inner = self.inner.restrictions(input);
        self
    }
    /// <p>Updates the API key restrictions for the API key resource.</p>
    pub fn set_restrictions(mut self, input: ::std::option::Option<crate::types::ApiKeyRestrictions>) -> Self {
        self.inner = self.inner.set_restrictions(input);
        self
    }
    /// <p>Updates the API key restrictions for the API key resource.</p>
    pub fn get_restrictions(&self) -> &::std::option::Option<crate::types::ApiKeyRestrictions> {
        self.inner.get_restrictions()
    }
}
