// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_data_quality_ruleset::_delete_data_quality_ruleset_output::DeleteDataQualityRulesetOutputBuilder;

pub use crate::operation::delete_data_quality_ruleset::_delete_data_quality_ruleset_input::DeleteDataQualityRulesetInputBuilder;

impl DeleteDataQualityRulesetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_data_quality_ruleset();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDataQualityRuleset`.
///
/// <p>Deletes a data quality ruleset.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDataQualityRulesetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_data_quality_ruleset::builders::DeleteDataQualityRulesetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetOutput,
        crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetError,
    > for DeleteDataQualityRulesetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetOutput,
            crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteDataQualityRulesetFluentBuilder {
    /// Creates a new `DeleteDataQualityRuleset`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDataQualityRuleset as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_data_quality_ruleset::builders::DeleteDataQualityRulesetInputBuilder {
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
        crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_data_quality_ruleset::DeleteDataQualityRuleset::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_data_quality_ruleset::DeleteDataQualityRuleset::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetOutput,
        crate::operation::delete_data_quality_ruleset::DeleteDataQualityRulesetError,
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
    /// <p>A name for the data quality ruleset.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>A name for the data quality ruleset.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>A name for the data quality ruleset.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}
