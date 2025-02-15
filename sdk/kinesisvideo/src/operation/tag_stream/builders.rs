// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::tag_stream::_tag_stream_output::TagStreamOutputBuilder;

pub use crate::operation::tag_stream::_tag_stream_input::TagStreamInputBuilder;

impl TagStreamInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::tag_stream::TagStreamOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::tag_stream::TagStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.tag_stream();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `TagStream`.
///
/// <p>Adds one or more tags to a stream. A <i>tag</i> is a key-value pair (the value is optional) that you can define and assign to Amazon Web Services resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>Billing and Cost Management and Cost Management User Guide</i>. </p>
/// <p>You must provide either the <code>StreamName</code> or the <code>StreamARN</code>.</p>
/// <p>This operation requires permission for the <code>KinesisVideo:TagStream</code> action.</p>
/// <p>A Kinesis video stream can support up to 50 tags.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct TagStreamFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::tag_stream::builders::TagStreamInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl crate::client::customize::internal::CustomizableSend<crate::operation::tag_stream::TagStreamOutput, crate::operation::tag_stream::TagStreamError>
    for TagStreamFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<crate::operation::tag_stream::TagStreamOutput, crate::operation::tag_stream::TagStreamError>,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl TagStreamFluentBuilder {
    /// Creates a new `TagStream`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the TagStream as a reference.
    pub fn as_input(&self) -> &crate::operation::tag_stream::builders::TagStreamInputBuilder {
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
        crate::operation::tag_stream::TagStreamOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::tag_stream::TagStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::tag_stream::TagStream::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::tag_stream::TagStream::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::tag_stream::TagStreamOutput,
        crate::operation::tag_stream::TagStreamError,
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
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to add the tag or tags to.</p>
    pub fn stream_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to add the tag or tags to.</p>
    pub fn set_stream_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to add the tag or tags to.</p>
    pub fn get_stream_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_arn()
    }
    /// <p>The name of the stream that you want to add the tag or tags to.</p>
    pub fn stream_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.stream_name(input.into());
        self
    }
    /// <p>The name of the stream that you want to add the tag or tags to.</p>
    pub fn set_stream_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_stream_name(input);
        self
    }
    /// <p>The name of the stream that you want to add the tag or tags to.</p>
    pub fn get_stream_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_stream_name()
    }
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tags to associate with the specified stream. Each tag is a key-value pair (the value is optional).</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A list of tags to associate with the specified stream. Each tag is a key-value pair (the value is optional).</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of tags to associate with the specified stream. Each tag is a key-value pair (the value is optional).</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
