// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_rule::_create_rule_output::CreateRuleOutputBuilder;

pub use crate::operation::create_rule::_create_rule_input::CreateRuleInputBuilder;

impl CreateRuleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_rule::CreateRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_rule::CreateRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_rule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateRule`.
///
/// <p>Creates a rule for use with the specified detector. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_rule::builders::CreateRuleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_rule::CreateRuleOutput,
        crate::operation::create_rule::CreateRuleError,
    > for CreateRuleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_rule::CreateRuleOutput,
            crate::operation::create_rule::CreateRuleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateRuleFluentBuilder {
    /// Creates a new `CreateRule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateRule as a reference.
    pub fn as_input(&self) -> &crate::operation::create_rule::builders::CreateRuleInputBuilder {
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
        crate::operation::create_rule::CreateRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_rule::CreateRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_rule::CreateRule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_rule::CreateRule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_rule::CreateRuleOutput,
        crate::operation::create_rule::CreateRuleError,
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
    /// <p>The rule ID.</p>
    pub fn rule_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_id(input.into());
        self
    }
    /// <p>The rule ID.</p>
    pub fn set_rule_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rule_id(input);
        self
    }
    /// <p>The rule ID.</p>
    pub fn get_rule_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rule_id()
    }
    /// <p>The detector ID for the rule's parent detector.</p>
    pub fn detector_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.detector_id(input.into());
        self
    }
    /// <p>The detector ID for the rule's parent detector.</p>
    pub fn set_detector_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_detector_id(input);
        self
    }
    /// <p>The detector ID for the rule's parent detector.</p>
    pub fn get_detector_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_detector_id()
    }
    /// <p>The rule description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The rule description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The rule description.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The rule expression.</p>
    pub fn expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.expression(input.into());
        self
    }
    /// <p>The rule expression.</p>
    pub fn set_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_expression(input);
        self
    }
    /// <p>The rule expression.</p>
    pub fn get_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_expression()
    }
    /// <p>The language of the rule.</p>
    pub fn language(mut self, input: crate::types::Language) -> Self {
        self.inner = self.inner.language(input);
        self
    }
    /// <p>The language of the rule.</p>
    pub fn set_language(mut self, input: ::std::option::Option<crate::types::Language>) -> Self {
        self.inner = self.inner.set_language(input);
        self
    }
    /// <p>The language of the rule.</p>
    pub fn get_language(&self) -> &::std::option::Option<crate::types::Language> {
        self.inner.get_language()
    }
    /// Appends an item to `outcomes`.
    ///
    /// To override the contents of this collection use [`set_outcomes`](Self::set_outcomes).
    ///
    /// <p>The outcome or outcomes returned when the rule expression matches.</p>
    pub fn outcomes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.outcomes(input.into());
        self
    }
    /// <p>The outcome or outcomes returned when the rule expression matches.</p>
    pub fn set_outcomes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_outcomes(input);
        self
    }
    /// <p>The outcome or outcomes returned when the rule expression matches.</p>
    pub fn get_outcomes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_outcomes()
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A collection of key and value pairs.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A collection of key and value pairs.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A collection of key and value pairs.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
