// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_get_collaboration_analysis_template::_batch_get_collaboration_analysis_template_output::BatchGetCollaborationAnalysisTemplateOutputBuilder;

pub use crate::operation::batch_get_collaboration_analysis_template::_batch_get_collaboration_analysis_template_input::BatchGetCollaborationAnalysisTemplateInputBuilder;

impl BatchGetCollaborationAnalysisTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_get_collaboration_analysis_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchGetCollaborationAnalysisTemplate`.
///
/// <p>Retrieves multiple analysis templates within a collaboration by their Amazon Resource Names (ARNs).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchGetCollaborationAnalysisTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_get_collaboration_analysis_template::builders::BatchGetCollaborationAnalysisTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateOutput,
        crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateError,
    > for BatchGetCollaborationAnalysisTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateOutput,
            crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchGetCollaborationAnalysisTemplateFluentBuilder {
    /// Creates a new `BatchGetCollaborationAnalysisTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchGetCollaborationAnalysisTemplate as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::batch_get_collaboration_analysis_template::builders::BatchGetCollaborationAnalysisTemplateInputBuilder {
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
        crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplate::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateOutput,
        crate::operation::batch_get_collaboration_analysis_template::BatchGetCollaborationAnalysisTemplateError,
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
    /// <p>A unique identifier for the collaboration that the analysis templates belong to. Currently accepts collaboration ID.</p>
    pub fn collaboration_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.collaboration_identifier(input.into());
        self
    }
    /// <p>A unique identifier for the collaboration that the analysis templates belong to. Currently accepts collaboration ID.</p>
    pub fn set_collaboration_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_collaboration_identifier(input);
        self
    }
    /// <p>A unique identifier for the collaboration that the analysis templates belong to. Currently accepts collaboration ID.</p>
    pub fn get_collaboration_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_collaboration_identifier()
    }
    /// Appends an item to `analysisTemplateArns`.
    ///
    /// To override the contents of this collection use [`set_analysis_template_arns`](Self::set_analysis_template_arns).
    ///
    /// <p>The Amazon Resource Name (ARN) associated with the analysis template within a collaboration.</p>
    pub fn analysis_template_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.analysis_template_arns(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the analysis template within a collaboration.</p>
    pub fn set_analysis_template_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_analysis_template_arns(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the analysis template within a collaboration.</p>
    pub fn get_analysis_template_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_analysis_template_arns()
    }
}
