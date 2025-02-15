// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_theme_permissions::_update_theme_permissions_output::UpdateThemePermissionsOutputBuilder;

pub use crate::operation::update_theme_permissions::_update_theme_permissions_input::UpdateThemePermissionsInputBuilder;

impl UpdateThemePermissionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_theme_permissions::UpdateThemePermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_theme_permissions::UpdateThemePermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_theme_permissions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateThemePermissions`.
///
/// <p>Updates the resource permissions for a theme. Permissions apply to the action to grant or revoke permissions on, for example <code>"quicksight:DescribeTheme"</code>.</p>
/// <p>Theme permissions apply in groupings. Valid groupings include the following for the three levels of permissions, which are user, owner, or no permissions: </p>
/// <ul>
/// <li> <p>User</p>
/// <ul>
/// <li> <p> <code>"quicksight:DescribeTheme"</code> </p> </li>
/// <li> <p> <code>"quicksight:DescribeThemeAlias"</code> </p> </li>
/// <li> <p> <code>"quicksight:ListThemeAliases"</code> </p> </li>
/// <li> <p> <code>"quicksight:ListThemeVersions"</code> </p> </li>
/// </ul> </li>
/// <li> <p>Owner</p>
/// <ul>
/// <li> <p> <code>"quicksight:DescribeTheme"</code> </p> </li>
/// <li> <p> <code>"quicksight:DescribeThemeAlias"</code> </p> </li>
/// <li> <p> <code>"quicksight:ListThemeAliases"</code> </p> </li>
/// <li> <p> <code>"quicksight:ListThemeVersions"</code> </p> </li>
/// <li> <p> <code>"quicksight:DeleteTheme"</code> </p> </li>
/// <li> <p> <code>"quicksight:UpdateTheme"</code> </p> </li>
/// <li> <p> <code>"quicksight:CreateThemeAlias"</code> </p> </li>
/// <li> <p> <code>"quicksight:DeleteThemeAlias"</code> </p> </li>
/// <li> <p> <code>"quicksight:UpdateThemeAlias"</code> </p> </li>
/// <li> <p> <code>"quicksight:UpdateThemePermissions"</code> </p> </li>
/// <li> <p> <code>"quicksight:DescribeThemePermissions"</code> </p> </li>
/// </ul> </li>
/// <li> <p>To specify no permissions, omit the permissions list.</p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateThemePermissionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_theme_permissions::builders::UpdateThemePermissionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_theme_permissions::UpdateThemePermissionsOutput,
        crate::operation::update_theme_permissions::UpdateThemePermissionsError,
    > for UpdateThemePermissionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_theme_permissions::UpdateThemePermissionsOutput,
            crate::operation::update_theme_permissions::UpdateThemePermissionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateThemePermissionsFluentBuilder {
    /// Creates a new `UpdateThemePermissions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateThemePermissions as a reference.
    pub fn as_input(&self) -> &crate::operation::update_theme_permissions::builders::UpdateThemePermissionsInputBuilder {
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
        crate::operation::update_theme_permissions::UpdateThemePermissionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_theme_permissions::UpdateThemePermissionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_theme_permissions::UpdateThemePermissions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_theme_permissions::UpdateThemePermissions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_theme_permissions::UpdateThemePermissionsOutput,
        crate::operation::update_theme_permissions::UpdateThemePermissionsError,
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
    /// <p>The ID of the Amazon Web Services account that contains the theme.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the theme.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account that contains the theme.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The ID for the theme.</p>
    pub fn theme_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.theme_id(input.into());
        self
    }
    /// <p>The ID for the theme.</p>
    pub fn set_theme_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_theme_id(input);
        self
    }
    /// <p>The ID for the theme.</p>
    pub fn get_theme_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_theme_id()
    }
    /// Appends an item to `GrantPermissions`.
    ///
    /// To override the contents of this collection use [`set_grant_permissions`](Self::set_grant_permissions).
    ///
    /// <p>A list of resource permissions to be granted for the theme.</p>
    pub fn grant_permissions(mut self, input: crate::types::ResourcePermission) -> Self {
        self.inner = self.inner.grant_permissions(input);
        self
    }
    /// <p>A list of resource permissions to be granted for the theme.</p>
    pub fn set_grant_permissions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>) -> Self {
        self.inner = self.inner.set_grant_permissions(input);
        self
    }
    /// <p>A list of resource permissions to be granted for the theme.</p>
    pub fn get_grant_permissions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>> {
        self.inner.get_grant_permissions()
    }
    /// Appends an item to `RevokePermissions`.
    ///
    /// To override the contents of this collection use [`set_revoke_permissions`](Self::set_revoke_permissions).
    ///
    /// <p>A list of resource permissions to be revoked from the theme.</p>
    pub fn revoke_permissions(mut self, input: crate::types::ResourcePermission) -> Self {
        self.inner = self.inner.revoke_permissions(input);
        self
    }
    /// <p>A list of resource permissions to be revoked from the theme.</p>
    pub fn set_revoke_permissions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>>) -> Self {
        self.inner = self.inner.set_revoke_permissions(input);
        self
    }
    /// <p>A list of resource permissions to be revoked from the theme.</p>
    pub fn get_revoke_permissions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ResourcePermission>> {
        self.inner.get_revoke_permissions()
    }
}
