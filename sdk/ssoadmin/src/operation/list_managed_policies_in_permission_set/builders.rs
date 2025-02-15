// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_managed_policies_in_permission_set::_list_managed_policies_in_permission_set_output::ListManagedPoliciesInPermissionSetOutputBuilder;

pub use crate::operation::list_managed_policies_in_permission_set::_list_managed_policies_in_permission_set_input::ListManagedPoliciesInPermissionSetInputBuilder;

impl ListManagedPoliciesInPermissionSetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_managed_policies_in_permission_set();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListManagedPoliciesInPermissionSet`.
///
/// <p>Lists the Amazon Web Services managed policy that is attached to a specified permission set.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListManagedPoliciesInPermissionSetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_managed_policies_in_permission_set::builders::ListManagedPoliciesInPermissionSetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetOutput,
        crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetError,
    > for ListManagedPoliciesInPermissionSetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetOutput,
            crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListManagedPoliciesInPermissionSetFluentBuilder {
    /// Creates a new `ListManagedPoliciesInPermissionSet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListManagedPoliciesInPermissionSet as a reference.
    pub fn as_input(&self) -> &crate::operation::list_managed_policies_in_permission_set::builders::ListManagedPoliciesInPermissionSetInputBuilder {
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
        crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSet::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetOutput,
        crate::operation::list_managed_policies_in_permission_set::ListManagedPoliciesInPermissionSetError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_managed_policies_in_permission_set::paginator::ListManagedPoliciesInPermissionSetPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_managed_policies_in_permission_set::paginator::ListManagedPoliciesInPermissionSetPaginator {
        crate::operation::list_managed_policies_in_permission_set::paginator::ListManagedPoliciesInPermissionSetPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn instance_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_arn(input.into());
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn set_instance_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_arn(input);
        self
    }
    /// <p>The ARN of the IAM Identity Center instance under which the operation will be executed. For more information about ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and Amazon Web Services Service Namespaces</a> in the <i>Amazon Web Services General Reference</i>.</p>
    pub fn get_instance_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_arn()
    }
    /// <p>The ARN of the <code>PermissionSet</code> whose managed policies will be listed.</p>
    pub fn permission_set_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.permission_set_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>PermissionSet</code> whose managed policies will be listed.</p>
    pub fn set_permission_set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_permission_set_arn(input);
        self
    }
    /// <p>The ARN of the <code>PermissionSet</code> whose managed policies will be listed.</p>
    pub fn get_permission_set_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_permission_set_arn()
    }
    /// <p>The maximum number of results to display for the <code>PermissionSet</code>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to display for the <code>PermissionSet</code>.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to display for the <code>PermissionSet</code>.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The pagination token for the list API. Initially the value is null. Use the output of previous API calls to make subsequent calls.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token for the list API. Initially the value is null. Use the output of previous API calls to make subsequent calls.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token for the list API. Initially the value is null. Use the output of previous API calls to make subsequent calls.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
