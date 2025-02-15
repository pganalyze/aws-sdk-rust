// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_hub::_create_hub_output::CreateHubOutputBuilder;

pub use crate::operation::create_hub::_create_hub_input::CreateHubInputBuilder;

impl CreateHubInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_hub::CreateHubOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_hub::CreateHubError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_hub();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateHub`.
///
/// <p>Create a hub.</p> <note>
/// <p>Hub APIs are only callable through SageMaker Studio.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateHubFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_hub::builders::CreateHubInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::create_hub::CreateHubOutput, crate::operation::create_hub::CreateHubError>
    for CreateHubFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::create_hub::CreateHubOutput, crate::operation::create_hub::CreateHubError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateHubFluentBuilder {
    /// Creates a new `CreateHub`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateHub as a reference.
    pub fn as_input(&self) -> &crate::operation::create_hub::builders::CreateHubInputBuilder {
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
        crate::operation::create_hub::CreateHubOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_hub::CreateHubError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_hub::CreateHub::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_hub::CreateHub::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_hub::CreateHubOutput,
        crate::operation::create_hub::CreateHubError,
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
    /// <p>The name of the hub to create.</p>
    pub fn hub_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.hub_name(input.into());
        self
    }
    /// <p>The name of the hub to create.</p>
    pub fn set_hub_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_hub_name(input);
        self
    }
    /// <p>The name of the hub to create.</p>
    pub fn get_hub_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_hub_name()
    }
    /// <p>A description of the hub.</p>
    pub fn hub_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.hub_description(input.into());
        self
    }
    /// <p>A description of the hub.</p>
    pub fn set_hub_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_hub_description(input);
        self
    }
    /// <p>A description of the hub.</p>
    pub fn get_hub_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_hub_description()
    }
    /// <p>The display name of the hub.</p>
    pub fn hub_display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.hub_display_name(input.into());
        self
    }
    /// <p>The display name of the hub.</p>
    pub fn set_hub_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_hub_display_name(input);
        self
    }
    /// <p>The display name of the hub.</p>
    pub fn get_hub_display_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_hub_display_name()
    }
    /// Appends an item to `HubSearchKeywords`.
    ///
    /// To override the contents of this collection use [`set_hub_search_keywords`](Self::set_hub_search_keywords).
    ///
    /// <p>The searchable keywords for the hub.</p>
    pub fn hub_search_keywords(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.hub_search_keywords(input.into());
        self
    }
    /// <p>The searchable keywords for the hub.</p>
    pub fn set_hub_search_keywords(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_hub_search_keywords(input);
        self
    }
    /// <p>The searchable keywords for the hub.</p>
    pub fn get_hub_search_keywords(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_hub_search_keywords()
    }
    /// <p>The Amazon S3 storage configuration for the hub.</p>
    pub fn s3_storage_config(mut self, input: crate::types::HubS3StorageConfig) -> Self {
        self.inner = self.inner.s3_storage_config(input);
        self
    }
    /// <p>The Amazon S3 storage configuration for the hub.</p>
    pub fn set_s3_storage_config(mut self, input: ::std::option::Option<crate::types::HubS3StorageConfig>) -> Self {
        self.inner = self.inner.set_s3_storage_config(input);
        self
    }
    /// <p>The Amazon S3 storage configuration for the hub.</p>
    pub fn get_s3_storage_config(&self) -> &::std::option::Option<crate::types::HubS3StorageConfig> {
        self.inner.get_s3_storage_config()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags to associate with the hub.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Any tags to associate with the hub.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Any tags to associate with the hub.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
