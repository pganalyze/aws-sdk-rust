// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_annotation_import_job::_start_annotation_import_job_output::StartAnnotationImportJobOutputBuilder;

pub use crate::operation::start_annotation_import_job::_start_annotation_import_job_input::StartAnnotationImportJobInputBuilder;

impl StartAnnotationImportJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_annotation_import_job::StartAnnotationImportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_annotation_import_job::StartAnnotationImportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_annotation_import_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartAnnotationImportJob`.
///
/// <p>Starts an annotation import job.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartAnnotationImportJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_annotation_import_job::builders::StartAnnotationImportJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_annotation_import_job::StartAnnotationImportJobOutput,
        crate::operation::start_annotation_import_job::StartAnnotationImportJobError,
    > for StartAnnotationImportJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_annotation_import_job::StartAnnotationImportJobOutput,
            crate::operation::start_annotation_import_job::StartAnnotationImportJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartAnnotationImportJobFluentBuilder {
    /// Creates a new `StartAnnotationImportJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartAnnotationImportJob as a reference.
    pub fn as_input(&self) -> &crate::operation::start_annotation_import_job::builders::StartAnnotationImportJobInputBuilder {
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
        crate::operation::start_annotation_import_job::StartAnnotationImportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_annotation_import_job::StartAnnotationImportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_annotation_import_job::StartAnnotationImportJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_annotation_import_job::StartAnnotationImportJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_annotation_import_job::StartAnnotationImportJobOutput,
        crate::operation::start_annotation_import_job::StartAnnotationImportJobError,
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
    /// <p>A destination annotation store for the job.</p>
    pub fn destination_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_name(input.into());
        self
    }
    /// <p>A destination annotation store for the job.</p>
    pub fn set_destination_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_name(input);
        self
    }
    /// <p>A destination annotation store for the job.</p>
    pub fn get_destination_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_name()
    }
    /// <p>A service role for the job.</p>
    pub fn role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.role_arn(input.into());
        self
    }
    /// <p>A service role for the job.</p>
    pub fn set_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_role_arn(input);
        self
    }
    /// <p>A service role for the job.</p>
    pub fn get_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_role_arn()
    }
    /// Appends an item to `items`.
    ///
    /// To override the contents of this collection use [`set_items`](Self::set_items).
    ///
    /// <p>Items to import.</p>
    pub fn items(mut self, input: crate::types::AnnotationImportItemSource) -> Self {
        self.inner = self.inner.items(input);
        self
    }
    /// <p>Items to import.</p>
    pub fn set_items(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AnnotationImportItemSource>>) -> Self {
        self.inner = self.inner.set_items(input);
        self
    }
    /// <p>Items to import.</p>
    pub fn get_items(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AnnotationImportItemSource>> {
        self.inner.get_items()
    }
    /// <p> The name of the annotation store version. </p>
    pub fn version_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_name(input.into());
        self
    }
    /// <p> The name of the annotation store version. </p>
    pub fn set_version_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_name(input);
        self
    }
    /// <p> The name of the annotation store version. </p>
    pub fn get_version_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_name()
    }
    /// <p>Formatting options for the annotation file.</p>
    pub fn format_options(mut self, input: crate::types::FormatOptions) -> Self {
        self.inner = self.inner.format_options(input);
        self
    }
    /// <p>Formatting options for the annotation file.</p>
    pub fn set_format_options(mut self, input: ::std::option::Option<crate::types::FormatOptions>) -> Self {
        self.inner = self.inner.set_format_options(input);
        self
    }
    /// <p>Formatting options for the annotation file.</p>
    pub fn get_format_options(&self) -> &::std::option::Option<crate::types::FormatOptions> {
        self.inner.get_format_options()
    }
    /// <p>The job's left normalization setting.</p>
    pub fn run_left_normalization(mut self, input: bool) -> Self {
        self.inner = self.inner.run_left_normalization(input);
        self
    }
    /// <p>The job's left normalization setting.</p>
    pub fn set_run_left_normalization(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_run_left_normalization(input);
        self
    }
    /// <p>The job's left normalization setting.</p>
    pub fn get_run_left_normalization(&self) -> &::std::option::Option<bool> {
        self.inner.get_run_left_normalization()
    }
    /// Adds a key-value pair to `annotationFields`.
    ///
    /// To override the contents of this collection use [`set_annotation_fields`](Self::set_annotation_fields).
    ///
    /// <p> The annotation schema generated by the parsed annotation data. </p>
    pub fn annotation_fields(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.annotation_fields(k.into(), v.into());
        self
    }
    /// <p> The annotation schema generated by the parsed annotation data. </p>
    pub fn set_annotation_fields(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_annotation_fields(input);
        self
    }
    /// <p> The annotation schema generated by the parsed annotation data. </p>
    pub fn get_annotation_fields(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_annotation_fields()
    }
}
