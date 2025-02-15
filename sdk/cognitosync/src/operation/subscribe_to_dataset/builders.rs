// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::subscribe_to_dataset::_subscribe_to_dataset_output::SubscribeToDatasetOutputBuilder;

pub use crate::operation::subscribe_to_dataset::_subscribe_to_dataset_input::SubscribeToDatasetInputBuilder;

impl SubscribeToDatasetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::subscribe_to_dataset::SubscribeToDatasetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::subscribe_to_dataset::SubscribeToDatasetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.subscribe_to_dataset();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SubscribeToDataset`.
///
/// <p>Subscribes to receive notifications when a dataset is modified by another device.</p>
/// <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p> <examples>
/// <example>
/// <name>
/// SubscribeToDataset
/// </name>
/// <description>
/// The following examples have been edited for readability.
/// </description>
/// <request>
/// POST / HTTP/1.1 CONTENT-TYPE: application/json X-AMZN-REQUESTID: 8b9932b7-201d-4418-a960-0a470e11de9f X-AMZ-TARGET: com.amazonaws.cognito.sync.model.AWSCognitoSyncService.SubscribeToDataset HOST: cognito-sync.us-east-1.amazonaws.com X-AMZ-DATE: 20141004T195350Z X-AMZ-SECURITY-TOKEN:
/// <securitytoken>
/// AUTHORIZATION: AWS4-HMAC-SHA256 Credential=
/// <credential>
/// , SignedHeaders=content-type;content-length;host;x-amz-date;x-amz-target, Signature=
/// <signature>
/// { "Operation": "com.amazonaws.cognito.sync.model#SubscribeToDataset", "Service": "com.amazonaws.cognito.sync.model#AWSCognitoSyncService", "Input": { "IdentityPoolId": "ID_POOL_ID", "IdentityId": "IDENTITY_ID", "DatasetName": "Rufus", "DeviceId": "5cd28fbe-dd83-47ab-9f83-19093a5fb014" } }
/// </signature>
/// </credential>
/// </securitytoken>
/// </request>
/// <response>
/// 1.1 200 OK x-amzn-requestid: 8b9932b7-201d-4418-a960-0a470e11de9f date: Sat, 04 Oct 2014 19:53:50 GMT content-type: application/json content-length: 99 { "Output": { "__type": "com.amazonaws.cognito.sync.model#SubscribeToDatasetResponse" }, "Version": "1.0" }
/// </response>
/// </example>
/// </examples>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SubscribeToDatasetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::subscribe_to_dataset::builders::SubscribeToDatasetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::subscribe_to_dataset::SubscribeToDatasetOutput,
        crate::operation::subscribe_to_dataset::SubscribeToDatasetError,
    > for SubscribeToDatasetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::subscribe_to_dataset::SubscribeToDatasetOutput,
            crate::operation::subscribe_to_dataset::SubscribeToDatasetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SubscribeToDatasetFluentBuilder {
    /// Creates a new `SubscribeToDataset`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SubscribeToDataset as a reference.
    pub fn as_input(&self) -> &crate::operation::subscribe_to_dataset::builders::SubscribeToDatasetInputBuilder {
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
        crate::operation::subscribe_to_dataset::SubscribeToDatasetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::subscribe_to_dataset::SubscribeToDatasetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::subscribe_to_dataset::SubscribeToDataset::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::subscribe_to_dataset::SubscribeToDataset::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::subscribe_to_dataset::SubscribeToDatasetOutput,
        crate::operation::subscribe_to_dataset::SubscribeToDatasetError,
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
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. The ID of the pool to which the identity belongs.</p>
    pub fn identity_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity_pool_id(input.into());
        self
    }
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. The ID of the pool to which the identity belongs.</p>
    pub fn set_identity_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity_pool_id(input);
        self
    }
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. The ID of the pool to which the identity belongs.</p>
    pub fn get_identity_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identity_pool_id()
    }
    /// <p>Unique ID for this identity.</p>
    pub fn identity_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identity_id(input.into());
        self
    }
    /// <p>Unique ID for this identity.</p>
    pub fn set_identity_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identity_id(input);
        self
    }
    /// <p>Unique ID for this identity.</p>
    pub fn get_identity_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identity_id()
    }
    /// <p>The name of the dataset to subcribe to.</p>
    pub fn dataset_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.dataset_name(input.into());
        self
    }
    /// <p>The name of the dataset to subcribe to.</p>
    pub fn set_dataset_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_dataset_name(input);
        self
    }
    /// <p>The name of the dataset to subcribe to.</p>
    pub fn get_dataset_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_dataset_name()
    }
    /// <p>The unique ID generated for this device by Cognito.</p>
    pub fn device_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.device_id(input.into());
        self
    }
    /// <p>The unique ID generated for this device by Cognito.</p>
    pub fn set_device_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_device_id(input);
        self
    }
    /// <p>The unique ID generated for this device by Cognito.</p>
    pub fn get_device_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_device_id()
    }
}
