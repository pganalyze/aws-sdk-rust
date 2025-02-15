// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_tenant_database::_modify_tenant_database_output::ModifyTenantDatabaseOutputBuilder;

pub use crate::operation::modify_tenant_database::_modify_tenant_database_input::ModifyTenantDatabaseInputBuilder;

impl ModifyTenantDatabaseInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_tenant_database::ModifyTenantDatabaseOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_tenant_database::ModifyTenantDatabaseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_tenant_database();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyTenantDatabase`.
///
/// <p>Modifies an existing tenant database in a DB instance. You can change the tenant database name or the master user password. This operation is supported only for RDS for Oracle CDB instances using the multi-tenant configuration.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyTenantDatabaseFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_tenant_database::builders::ModifyTenantDatabaseInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_tenant_database::ModifyTenantDatabaseOutput,
        crate::operation::modify_tenant_database::ModifyTenantDatabaseError,
    > for ModifyTenantDatabaseFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_tenant_database::ModifyTenantDatabaseOutput,
            crate::operation::modify_tenant_database::ModifyTenantDatabaseError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyTenantDatabaseFluentBuilder {
    /// Creates a new `ModifyTenantDatabase`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyTenantDatabase as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_tenant_database::builders::ModifyTenantDatabaseInputBuilder {
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
        crate::operation::modify_tenant_database::ModifyTenantDatabaseOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_tenant_database::ModifyTenantDatabaseError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_tenant_database::ModifyTenantDatabase::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_tenant_database::ModifyTenantDatabase::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_tenant_database::ModifyTenantDatabaseOutput,
        crate::operation::modify_tenant_database::ModifyTenantDatabaseError,
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
    /// <p>The identifier of the DB instance that contains the tenant database that you are modifying. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing DB instance.</p> </li>
    /// </ul>
    pub fn db_instance_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_instance_identifier(input.into());
        self
    }
    /// <p>The identifier of the DB instance that contains the tenant database that you are modifying. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing DB instance.</p> </li>
    /// </ul>
    pub fn set_db_instance_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_instance_identifier(input);
        self
    }
    /// <p>The identifier of the DB instance that contains the tenant database that you are modifying. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing DB instance.</p> </li>
    /// </ul>
    pub fn get_db_instance_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_instance_identifier()
    }
    /// <p>The user-supplied name of the tenant database that you want to modify. This parameter isn’t case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing tenant database.</p> </li>
    /// </ul>
    pub fn tenant_db_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tenant_db_name(input.into());
        self
    }
    /// <p>The user-supplied name of the tenant database that you want to modify. This parameter isn’t case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing tenant database.</p> </li>
    /// </ul>
    pub fn set_tenant_db_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_tenant_db_name(input);
        self
    }
    /// <p>The user-supplied name of the tenant database that you want to modify. This parameter isn’t case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Must match the identifier of an existing tenant database.</p> </li>
    /// </ul>
    pub fn get_tenant_db_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_tenant_db_name()
    }
    /// <p>The new password for the master user of the specified tenant database in your DB instance.</p> <note>
    /// <p>Amazon RDS operations never return the password, so this action provides a way to regain access to a tenant database user if the password is lost. This includes restoring privileges that might have been accidentally revoked.</p>
    /// </note>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Can include any printable ASCII character except <code>/</code>, <code>"</code> (double quote), <code>@</code>, <code>&amp;</code> (ampersand), and <code>'</code> (single quote).</p> </li>
    /// </ul>
    /// <p>Length constraints:</p>
    /// <ul>
    /// <li> <p>Must contain between 8 and 30 characters. </p> </li>
    /// </ul>
    pub fn master_user_password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.master_user_password(input.into());
        self
    }
    /// <p>The new password for the master user of the specified tenant database in your DB instance.</p> <note>
    /// <p>Amazon RDS operations never return the password, so this action provides a way to regain access to a tenant database user if the password is lost. This includes restoring privileges that might have been accidentally revoked.</p>
    /// </note>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Can include any printable ASCII character except <code>/</code>, <code>"</code> (double quote), <code>@</code>, <code>&amp;</code> (ampersand), and <code>'</code> (single quote).</p> </li>
    /// </ul>
    /// <p>Length constraints:</p>
    /// <ul>
    /// <li> <p>Must contain between 8 and 30 characters. </p> </li>
    /// </ul>
    pub fn set_master_user_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_master_user_password(input);
        self
    }
    /// <p>The new password for the master user of the specified tenant database in your DB instance.</p> <note>
    /// <p>Amazon RDS operations never return the password, so this action provides a way to regain access to a tenant database user if the password is lost. This includes restoring privileges that might have been accidentally revoked.</p>
    /// </note>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Can include any printable ASCII character except <code>/</code>, <code>"</code> (double quote), <code>@</code>, <code>&amp;</code> (ampersand), and <code>'</code> (single quote).</p> </li>
    /// </ul>
    /// <p>Length constraints:</p>
    /// <ul>
    /// <li> <p>Must contain between 8 and 30 characters. </p> </li>
    /// </ul>
    pub fn get_master_user_password(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_master_user_password()
    }
    /// <p>The new name of the tenant database when renaming a tenant database. This parameter isn’t case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Can't be the string null or any other reserved word.</p> </li>
    /// <li> <p>Can't be longer than 8 characters.</p> </li>
    /// </ul>
    pub fn new_tenant_db_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.new_tenant_db_name(input.into());
        self
    }
    /// <p>The new name of the tenant database when renaming a tenant database. This parameter isn’t case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Can't be the string null or any other reserved word.</p> </li>
    /// <li> <p>Can't be longer than 8 characters.</p> </li>
    /// </ul>
    pub fn set_new_tenant_db_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_new_tenant_db_name(input);
        self
    }
    /// <p>The new name of the tenant database when renaming a tenant database. This parameter isn’t case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li> <p>Can't be the string null or any other reserved word.</p> </li>
    /// <li> <p>Can't be longer than 8 characters.</p> </li>
    /// </ul>
    pub fn get_new_tenant_db_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_new_tenant_db_name()
    }
}
