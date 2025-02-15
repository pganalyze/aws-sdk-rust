// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_shared_directories::_describe_shared_directories_output::DescribeSharedDirectoriesOutputBuilder;

pub use crate::operation::describe_shared_directories::_describe_shared_directories_input::DescribeSharedDirectoriesInputBuilder;

impl DescribeSharedDirectoriesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_shared_directories::DescribeSharedDirectoriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_shared_directories::DescribeSharedDirectoriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_shared_directories();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeSharedDirectories`.
///
/// <p>Returns the shared directories in your account. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeSharedDirectoriesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_shared_directories::DescribeSharedDirectoriesOutput,
        crate::operation::describe_shared_directories::DescribeSharedDirectoriesError,
    > for DescribeSharedDirectoriesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_shared_directories::DescribeSharedDirectoriesOutput,
            crate::operation::describe_shared_directories::DescribeSharedDirectoriesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeSharedDirectoriesFluentBuilder {
    /// Creates a new `DescribeSharedDirectories`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeSharedDirectories as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_shared_directories::builders::DescribeSharedDirectoriesInputBuilder {
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
        crate::operation::describe_shared_directories::DescribeSharedDirectoriesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_shared_directories::DescribeSharedDirectoriesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_shared_directories::DescribeSharedDirectories::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_shared_directories::DescribeSharedDirectories::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_shared_directories::DescribeSharedDirectoriesOutput,
        crate::operation::describe_shared_directories::DescribeSharedDirectoriesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_shared_directories::paginator::DescribeSharedDirectoriesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_shared_directories::paginator::DescribeSharedDirectoriesPaginator {
        crate::operation::describe_shared_directories::paginator::DescribeSharedDirectoriesPaginator::new(self.handle, self.inner)
    }
    /// <p>Returns the identifier of the directory in the directory owner account. </p>
    pub fn owner_directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.owner_directory_id(input.into());
        self
    }
    /// <p>Returns the identifier of the directory in the directory owner account. </p>
    pub fn set_owner_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_owner_directory_id(input);
        self
    }
    /// <p>Returns the identifier of the directory in the directory owner account. </p>
    pub fn get_owner_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_owner_directory_id()
    }
    /// Appends an item to `SharedDirectoryIds`.
    ///
    /// To override the contents of this collection use [`set_shared_directory_ids`](Self::set_shared_directory_ids).
    ///
    /// <p>A list of identifiers of all shared directories in your account. </p>
    pub fn shared_directory_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.shared_directory_ids(input.into());
        self
    }
    /// <p>A list of identifiers of all shared directories in your account. </p>
    pub fn set_shared_directory_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_shared_directory_ids(input);
        self
    }
    /// <p>A list of identifiers of all shared directories in your account. </p>
    pub fn get_shared_directory_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_shared_directory_ids()
    }
    /// <p>The <code>DescribeSharedDirectoriesResult.NextToken</code> value from a previous call to <code>DescribeSharedDirectories</code>. Pass null if this is the first call. </p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The <code>DescribeSharedDirectoriesResult.NextToken</code> value from a previous call to <code>DescribeSharedDirectories</code>. Pass null if this is the first call. </p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The <code>DescribeSharedDirectoriesResult.NextToken</code> value from a previous call to <code>DescribeSharedDirectories</code>. Pass null if this is the first call. </p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The number of shared directories to return in the response object.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The number of shared directories to return in the response object.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The number of shared directories to return in the response object.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
}
