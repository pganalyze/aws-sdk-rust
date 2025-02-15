// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_configuration_sets::_describe_configuration_sets_output::DescribeConfigurationSetsOutputBuilder;

pub use crate::operation::describe_configuration_sets::_describe_configuration_sets_input::DescribeConfigurationSetsInputBuilder;

impl DescribeConfigurationSetsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_configuration_sets::DescribeConfigurationSetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_configuration_sets::DescribeConfigurationSetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_configuration_sets();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeConfigurationSets`.
///
/// <p>Describes the specified configuration sets or all in your account.</p>
/// <p>If you specify configuration set names, the output includes information for only the specified configuration sets. If you specify filters, the output includes information for only those configuration sets that meet the filter criteria. If you don't specify configuration set names or filters, the output includes information for all configuration sets.</p>
/// <p>If you specify a configuration set name that isn't valid, an error is returned.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeConfigurationSetsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_configuration_sets::builders::DescribeConfigurationSetsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_configuration_sets::DescribeConfigurationSetsOutput,
        crate::operation::describe_configuration_sets::DescribeConfigurationSetsError,
    > for DescribeConfigurationSetsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_configuration_sets::DescribeConfigurationSetsOutput,
            crate::operation::describe_configuration_sets::DescribeConfigurationSetsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeConfigurationSetsFluentBuilder {
    /// Creates a new `DescribeConfigurationSets`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeConfigurationSets as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_configuration_sets::builders::DescribeConfigurationSetsInputBuilder {
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
        crate::operation::describe_configuration_sets::DescribeConfigurationSetsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_configuration_sets::DescribeConfigurationSetsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_configuration_sets::DescribeConfigurationSets::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_configuration_sets::DescribeConfigurationSets::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_configuration_sets::DescribeConfigurationSetsOutput,
        crate::operation::describe_configuration_sets::DescribeConfigurationSetsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_configuration_sets::paginator::DescribeConfigurationSetsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_configuration_sets::paginator::DescribeConfigurationSetsPaginator {
        crate::operation::describe_configuration_sets::paginator::DescribeConfigurationSetsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `ConfigurationSetNames`.
    ///
    /// To override the contents of this collection use [`set_configuration_set_names`](Self::set_configuration_set_names).
    ///
    /// <p>An array of strings. Each element can be either a ConfigurationSetName or ConfigurationSetArn.</p>
    pub fn configuration_set_names(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.configuration_set_names(input.into());
        self
    }
    /// <p>An array of strings. Each element can be either a ConfigurationSetName or ConfigurationSetArn.</p>
    pub fn set_configuration_set_names(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_configuration_set_names(input);
        self
    }
    /// <p>An array of strings. Each element can be either a ConfigurationSetName or ConfigurationSetArn.</p>
    pub fn get_configuration_set_names(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_configuration_set_names()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>An array of filters to apply to the results that are returned.</p>
    pub fn filters(mut self, input: crate::types::ConfigurationSetFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>An array of filters to apply to the results that are returned.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ConfigurationSetFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>An array of filters to apply to the results that are returned.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ConfigurationSetFilter>> {
        self.inner.get_filters()
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to be used for the next set of paginated results. You don't need to supply a value for this field in the initial request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return per each request.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
}
