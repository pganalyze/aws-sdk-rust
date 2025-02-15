// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_media_insights_pipeline_configuration::_update_media_insights_pipeline_configuration_output::UpdateMediaInsightsPipelineConfigurationOutputBuilder;

pub use crate::operation::update_media_insights_pipeline_configuration::_update_media_insights_pipeline_configuration_input::UpdateMediaInsightsPipelineConfigurationInputBuilder;

impl UpdateMediaInsightsPipelineConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_media_insights_pipeline_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateMediaInsightsPipelineConfiguration`.
///
/// <p>Updates the media insights pipeline's configuration settings.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateMediaInsightsPipelineConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_media_insights_pipeline_configuration::builders::UpdateMediaInsightsPipelineConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationOutput,
        crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationError,
    > for UpdateMediaInsightsPipelineConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationOutput,
            crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateMediaInsightsPipelineConfigurationFluentBuilder {
    /// Creates a new `UpdateMediaInsightsPipelineConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateMediaInsightsPipelineConfiguration as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::update_media_insights_pipeline_configuration::builders::UpdateMediaInsightsPipelineConfigurationInputBuilder {
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
        crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfiguration::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfiguration::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationOutput,
        crate::operation::update_media_insights_pipeline_configuration::UpdateMediaInsightsPipelineConfigurationError,
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
    /// <p>The unique identifier for the resource to be updated. Valid values include the name and ARN of the media insights pipeline configuration.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>The unique identifier for the resource to be updated. Valid values include the name and ARN of the media insights pipeline configuration.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// <p>The unique identifier for the resource to be updated. Valid values include the name and ARN of the media insights pipeline configuration.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
    /// <p>The ARN of the role used by the service to access Amazon Web Services resources.</p>
    pub fn resource_access_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_access_role_arn(input.into());
        self
    }
    /// <p>The ARN of the role used by the service to access Amazon Web Services resources.</p>
    pub fn set_resource_access_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_access_role_arn(input);
        self
    }
    /// <p>The ARN of the role used by the service to access Amazon Web Services resources.</p>
    pub fn get_resource_access_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_access_role_arn()
    }
    /// <p>The configuration settings for real-time alerts for the media insights pipeline.</p>
    pub fn real_time_alert_configuration(mut self, input: crate::types::RealTimeAlertConfiguration) -> Self {
        self.inner = self.inner.real_time_alert_configuration(input);
        self
    }
    /// <p>The configuration settings for real-time alerts for the media insights pipeline.</p>
    pub fn set_real_time_alert_configuration(mut self, input: ::std::option::Option<crate::types::RealTimeAlertConfiguration>) -> Self {
        self.inner = self.inner.set_real_time_alert_configuration(input);
        self
    }
    /// <p>The configuration settings for real-time alerts for the media insights pipeline.</p>
    pub fn get_real_time_alert_configuration(&self) -> &::std::option::Option<crate::types::RealTimeAlertConfiguration> {
        self.inner.get_real_time_alert_configuration()
    }
    /// Appends an item to `Elements`.
    ///
    /// To override the contents of this collection use [`set_elements`](Self::set_elements).
    ///
    /// <p>The elements in the request, such as a processor for Amazon Transcribe or a sink for a Kinesis Data Stream..</p>
    pub fn elements(mut self, input: crate::types::MediaInsightsPipelineConfigurationElement) -> Self {
        self.inner = self.inner.elements(input);
        self
    }
    /// <p>The elements in the request, such as a processor for Amazon Transcribe or a sink for a Kinesis Data Stream..</p>
    pub fn set_elements(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MediaInsightsPipelineConfigurationElement>>) -> Self {
        self.inner = self.inner.set_elements(input);
        self
    }
    /// <p>The elements in the request, such as a processor for Amazon Transcribe or a sink for a Kinesis Data Stream..</p>
    pub fn get_elements(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MediaInsightsPipelineConfigurationElement>> {
        self.inner.get_elements()
    }
}
