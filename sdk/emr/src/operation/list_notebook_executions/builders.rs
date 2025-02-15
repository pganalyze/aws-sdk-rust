// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_notebook_executions::_list_notebook_executions_output::ListNotebookExecutionsOutputBuilder;

pub use crate::operation::list_notebook_executions::_list_notebook_executions_input::ListNotebookExecutionsInputBuilder;

impl ListNotebookExecutionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_notebook_executions::ListNotebookExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_notebook_executions::ListNotebookExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_notebook_executions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListNotebookExecutions`.
///
/// <p>Provides summaries of all notebook executions. You can filter the list based on multiple criteria such as status, time range, and editor id. Returns a maximum of 50 notebook executions and a marker to track the paging of a longer notebook execution list across multiple <code>ListNotebookExecutions</code> calls.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListNotebookExecutionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_notebook_executions::builders::ListNotebookExecutionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_notebook_executions::ListNotebookExecutionsOutput,
        crate::operation::list_notebook_executions::ListNotebookExecutionsError,
    > for ListNotebookExecutionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_notebook_executions::ListNotebookExecutionsOutput,
            crate::operation::list_notebook_executions::ListNotebookExecutionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListNotebookExecutionsFluentBuilder {
    /// Creates a new `ListNotebookExecutions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListNotebookExecutions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_notebook_executions::builders::ListNotebookExecutionsInputBuilder {
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
        crate::operation::list_notebook_executions::ListNotebookExecutionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_notebook_executions::ListNotebookExecutionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_notebook_executions::ListNotebookExecutions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_notebook_executions::ListNotebookExecutions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_notebook_executions::ListNotebookExecutionsOutput,
        crate::operation::list_notebook_executions::ListNotebookExecutionsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_notebook_executions::paginator::ListNotebookExecutionsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_notebook_executions::paginator::ListNotebookExecutionsPaginator {
        crate::operation::list_notebook_executions::paginator::ListNotebookExecutionsPaginator::new(self.handle, self.inner)
    }
    /// <p>The unique ID of the editor associated with the notebook execution.</p>
    pub fn editor_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.editor_id(input.into());
        self
    }
    /// <p>The unique ID of the editor associated with the notebook execution.</p>
    pub fn set_editor_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_editor_id(input);
        self
    }
    /// <p>The unique ID of the editor associated with the notebook execution.</p>
    pub fn get_editor_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_editor_id()
    }
    /// <p>The status filter for listing notebook executions.</p>
    /// <ul>
    /// <li> <p> <code>START_PENDING</code> indicates that the cluster has received the execution request but execution has not begun.</p> </li>
    /// <li> <p> <code>STARTING</code> indicates that the execution is starting on the cluster.</p> </li>
    /// <li> <p> <code>RUNNING</code> indicates that the execution is being processed by the cluster.</p> </li>
    /// <li> <p> <code>FINISHING</code> indicates that execution processing is in the final stages.</p> </li>
    /// <li> <p> <code>FINISHED</code> indicates that the execution has completed without error.</p> </li>
    /// <li> <p> <code>FAILING</code> indicates that the execution is failing and will not finish successfully.</p> </li>
    /// <li> <p> <code>FAILED</code> indicates that the execution failed.</p> </li>
    /// <li> <p> <code>STOP_PENDING</code> indicates that the cluster has received a <code>StopNotebookExecution</code> request and the stop is pending.</p> </li>
    /// <li> <p> <code>STOPPING</code> indicates that the cluster is in the process of stopping the execution as a result of a <code>StopNotebookExecution</code> request.</p> </li>
    /// <li> <p> <code>STOPPED</code> indicates that the execution stopped because of a <code>StopNotebookExecution</code> request.</p> </li>
    /// </ul>
    pub fn status(mut self, input: crate::types::NotebookExecutionStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The status filter for listing notebook executions.</p>
    /// <ul>
    /// <li> <p> <code>START_PENDING</code> indicates that the cluster has received the execution request but execution has not begun.</p> </li>
    /// <li> <p> <code>STARTING</code> indicates that the execution is starting on the cluster.</p> </li>
    /// <li> <p> <code>RUNNING</code> indicates that the execution is being processed by the cluster.</p> </li>
    /// <li> <p> <code>FINISHING</code> indicates that execution processing is in the final stages.</p> </li>
    /// <li> <p> <code>FINISHED</code> indicates that the execution has completed without error.</p> </li>
    /// <li> <p> <code>FAILING</code> indicates that the execution is failing and will not finish successfully.</p> </li>
    /// <li> <p> <code>FAILED</code> indicates that the execution failed.</p> </li>
    /// <li> <p> <code>STOP_PENDING</code> indicates that the cluster has received a <code>StopNotebookExecution</code> request and the stop is pending.</p> </li>
    /// <li> <p> <code>STOPPING</code> indicates that the cluster is in the process of stopping the execution as a result of a <code>StopNotebookExecution</code> request.</p> </li>
    /// <li> <p> <code>STOPPED</code> indicates that the execution stopped because of a <code>StopNotebookExecution</code> request.</p> </li>
    /// </ul>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::NotebookExecutionStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>The status filter for listing notebook executions.</p>
    /// <ul>
    /// <li> <p> <code>START_PENDING</code> indicates that the cluster has received the execution request but execution has not begun.</p> </li>
    /// <li> <p> <code>STARTING</code> indicates that the execution is starting on the cluster.</p> </li>
    /// <li> <p> <code>RUNNING</code> indicates that the execution is being processed by the cluster.</p> </li>
    /// <li> <p> <code>FINISHING</code> indicates that execution processing is in the final stages.</p> </li>
    /// <li> <p> <code>FINISHED</code> indicates that the execution has completed without error.</p> </li>
    /// <li> <p> <code>FAILING</code> indicates that the execution is failing and will not finish successfully.</p> </li>
    /// <li> <p> <code>FAILED</code> indicates that the execution failed.</p> </li>
    /// <li> <p> <code>STOP_PENDING</code> indicates that the cluster has received a <code>StopNotebookExecution</code> request and the stop is pending.</p> </li>
    /// <li> <p> <code>STOPPING</code> indicates that the cluster is in the process of stopping the execution as a result of a <code>StopNotebookExecution</code> request.</p> </li>
    /// <li> <p> <code>STOPPED</code> indicates that the execution stopped because of a <code>StopNotebookExecution</code> request.</p> </li>
    /// </ul>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::NotebookExecutionStatus> {
        self.inner.get_status()
    }
    /// <p>The beginning of time range filter for listing notebook executions. The default is the timestamp of 30 days ago.</p>
    pub fn from(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.from(input);
        self
    }
    /// <p>The beginning of time range filter for listing notebook executions. The default is the timestamp of 30 days ago.</p>
    pub fn set_from(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_from(input);
        self
    }
    /// <p>The beginning of time range filter for listing notebook executions. The default is the timestamp of 30 days ago.</p>
    pub fn get_from(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_from()
    }
    /// <p>The end of time range filter for listing notebook executions. The default is the current timestamp.</p>
    pub fn to(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.to(input);
        self
    }
    /// <p>The end of time range filter for listing notebook executions. The default is the current timestamp.</p>
    pub fn set_to(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_to(input);
        self
    }
    /// <p>The end of time range filter for listing notebook executions. The default is the current timestamp.</p>
    pub fn get_to(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_to()
    }
    /// <p>The pagination token, returned by a previous <code>ListNotebookExecutions</code> call, that indicates the start of the list for this <code>ListNotebookExecutions</code> call.</p>
    pub fn marker(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.marker(input.into());
        self
    }
    /// <p>The pagination token, returned by a previous <code>ListNotebookExecutions</code> call, that indicates the start of the list for this <code>ListNotebookExecutions</code> call.</p>
    pub fn set_marker(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_marker(input);
        self
    }
    /// <p>The pagination token, returned by a previous <code>ListNotebookExecutions</code> call, that indicates the start of the list for this <code>ListNotebookExecutions</code> call.</p>
    pub fn get_marker(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_marker()
    }
    /// <p>The unique ID of the execution engine.</p>
    pub fn execution_engine_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.execution_engine_id(input.into());
        self
    }
    /// <p>The unique ID of the execution engine.</p>
    pub fn set_execution_engine_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_execution_engine_id(input);
        self
    }
    /// <p>The unique ID of the execution engine.</p>
    pub fn get_execution_engine_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_execution_engine_id()
    }
}
