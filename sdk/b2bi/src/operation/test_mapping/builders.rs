// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::test_mapping::_test_mapping_output::TestMappingOutputBuilder;

pub use crate::operation::test_mapping::_test_mapping_input::TestMappingInputBuilder;

impl TestMappingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::test_mapping::TestMappingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::test_mapping::TestMappingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.test_mapping();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TestMapping`.
///
/// <p>Maps the input file according to the provided template file.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TestMappingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::test_mapping::builders::TestMappingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::test_mapping::TestMappingOutput,
        crate::operation::test_mapping::TestMappingError,
    > for TestMappingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::test_mapping::TestMappingOutput,
            crate::operation::test_mapping::TestMappingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl TestMappingFluentBuilder {
    /// Creates a new `TestMapping`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TestMapping as a reference.
    pub fn as_input(&self) -> &crate::operation::test_mapping::builders::TestMappingInputBuilder {
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
        crate::operation::test_mapping::TestMappingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::test_mapping::TestMappingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::test_mapping::TestMapping::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::test_mapping::TestMapping::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::test_mapping::TestMappingOutput,
        crate::operation::test_mapping::TestMappingError,
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
    /// <p>Specify the EDI (electronic data interchange) file that is used as input for the transform.</p>
    pub fn input_file_content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.input_file_content(input.into());
        self
    }
    /// <p>Specify the EDI (electronic data interchange) file that is used as input for the transform.</p>
    pub fn set_input_file_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_input_file_content(input);
        self
    }
    /// <p>Specify the EDI (electronic data interchange) file that is used as input for the transform.</p>
    pub fn get_input_file_content(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_input_file_content()
    }
    /// <p>Specifies the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub fn mapping_template(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.mapping_template(input.into());
        self
    }
    /// <p>Specifies the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub fn set_mapping_template(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_mapping_template(input);
        self
    }
    /// <p>Specifies the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub fn get_mapping_template(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_mapping_template()
    }
    /// <p>Specifies that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub fn file_format(mut self, input: crate::types::FileFormat) -> Self {
        self.inner = self.inner.file_format(input);
        self
    }
    /// <p>Specifies that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub fn set_file_format(mut self, input: ::std::option::Option<crate::types::FileFormat>) -> Self {
        self.inner = self.inner.set_file_format(input);
        self
    }
    /// <p>Specifies that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub fn get_file_format(&self) -> &::std::option::Option<crate::types::FileFormat> {
        self.inner.get_file_format()
    }
}
