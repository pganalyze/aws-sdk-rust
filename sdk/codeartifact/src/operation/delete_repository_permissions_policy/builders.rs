// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_repository_permissions_policy::_delete_repository_permissions_policy_output::DeleteRepositoryPermissionsPolicyOutputBuilder;

pub use crate::operation::delete_repository_permissions_policy::_delete_repository_permissions_policy_input::DeleteRepositoryPermissionsPolicyInputBuilder;

impl DeleteRepositoryPermissionsPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_repository_permissions_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteRepositoryPermissionsPolicy`.
///
/// <p> Deletes the resource policy that is set on a repository. After a resource policy is deleted, the permissions allowed and denied by the deleted policy are removed. The effect of deleting a resource policy might not be immediate. </p> <important>
/// <p> Use <code>DeleteRepositoryPermissionsPolicy</code> with caution. After a policy is deleted, Amazon Web Services users, roles, and accounts lose permissions to perform the repository actions granted by the deleted policy. </p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteRepositoryPermissionsPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_repository_permissions_policy::builders::DeleteRepositoryPermissionsPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyOutput,
        crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError,
    > for DeleteRepositoryPermissionsPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyOutput,
            crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteRepositoryPermissionsPolicyFluentBuilder {
    /// Creates a new `DeleteRepositoryPermissionsPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteRepositoryPermissionsPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_repository_permissions_policy::builders::DeleteRepositoryPermissionsPolicyInputBuilder {
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
        crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyOutput,
        crate::operation::delete_repository_permissions_policy::DeleteRepositoryPermissionsPolicyError,
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
    /// <p> The name of the domain that contains the repository associated with the resource policy to be deleted. </p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain(input.into());
        self
    }
    /// <p> The name of the domain that contains the repository associated with the resource policy to be deleted. </p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain(input);
        self
    }
    /// <p> The name of the domain that contains the repository associated with the resource policy to be deleted. </p>
    pub fn get_domain(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain()
    }
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    pub fn domain_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_owner(input.into());
        self
    }
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    pub fn set_domain_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_owner(input);
        self
    }
    /// <p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p>
    pub fn get_domain_owner(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_owner()
    }
    /// <p> The name of the repository that is associated with the resource policy to be deleted </p>
    pub fn repository(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository(input.into());
        self
    }
    /// <p> The name of the repository that is associated with the resource policy to be deleted </p>
    pub fn set_repository(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_repository(input);
        self
    }
    /// <p> The name of the repository that is associated with the resource policy to be deleted </p>
    pub fn get_repository(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_repository()
    }
    /// <p> The revision of the repository's resource policy to be deleted. This revision is used for optimistic locking, which prevents others from accidentally overwriting your changes to the repository's resource policy. </p>
    pub fn policy_revision(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_revision(input.into());
        self
    }
    /// <p> The revision of the repository's resource policy to be deleted. This revision is used for optimistic locking, which prevents others from accidentally overwriting your changes to the repository's resource policy. </p>
    pub fn set_policy_revision(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_revision(input);
        self
    }
    /// <p> The revision of the repository's resource policy to be deleted. This revision is used for optimistic locking, which prevents others from accidentally overwriting your changes to the repository's resource policy. </p>
    pub fn get_policy_revision(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_revision()
    }
}
