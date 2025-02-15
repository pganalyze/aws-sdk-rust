// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_kx_cluster_databases::_update_kx_cluster_databases_output::UpdateKxClusterDatabasesOutputBuilder;

pub use crate::operation::update_kx_cluster_databases::_update_kx_cluster_databases_input::UpdateKxClusterDatabasesInputBuilder;

impl UpdateKxClusterDatabasesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_kx_cluster_databases();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateKxClusterDatabases`.
///
/// <p>Updates the databases mounted on a kdb cluster, which includes the <code>changesetId</code> and all the dbPaths to be cached. This API does not allow you to change a database name or add a database if you created a cluster without one. </p>
/// <p>Using this API you can point a cluster to a different changeset and modify a list of partitions being cached.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateKxClusterDatabasesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_kx_cluster_databases::builders::UpdateKxClusterDatabasesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesOutput,
        crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesError,
    > for UpdateKxClusterDatabasesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesOutput,
            crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateKxClusterDatabasesFluentBuilder {
    /// Creates a new `UpdateKxClusterDatabases`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateKxClusterDatabases as a reference.
    pub fn as_input(&self) -> &crate::operation::update_kx_cluster_databases::builders::UpdateKxClusterDatabasesInputBuilder {
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
        crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabases::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabases::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesOutput,
        crate::operation::update_kx_cluster_databases::UpdateKxClusterDatabasesError,
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
    /// <p>The unique identifier of a kdb environment.</p>
    pub fn environment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.environment_id(input.into());
        self
    }
    /// <p>The unique identifier of a kdb environment.</p>
    pub fn set_environment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_environment_id(input);
        self
    }
    /// <p>The unique identifier of a kdb environment.</p>
    pub fn get_environment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_environment_id()
    }
    /// <p>A unique name for the cluster that you want to modify.</p>
    pub fn cluster_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cluster_name(input.into());
        self
    }
    /// <p>A unique name for the cluster that you want to modify.</p>
    pub fn set_cluster_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cluster_name(input);
        self
    }
    /// <p>A unique name for the cluster that you want to modify.</p>
    pub fn get_cluster_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cluster_name()
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A token that ensures idempotency. This token expires in 10 minutes.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Appends an item to `databases`.
    ///
    /// To override the contents of this collection use [`set_databases`](Self::set_databases).
    ///
    /// <p> The structure of databases mounted on the cluster.</p>
    pub fn databases(mut self, input: crate::types::KxDatabaseConfiguration) -> Self {
        self.inner = self.inner.databases(input);
        self
    }
    /// <p> The structure of databases mounted on the cluster.</p>
    pub fn set_databases(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::KxDatabaseConfiguration>>) -> Self {
        self.inner = self.inner.set_databases(input);
        self
    }
    /// <p> The structure of databases mounted on the cluster.</p>
    pub fn get_databases(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::KxDatabaseConfiguration>> {
        self.inner.get_databases()
    }
    /// <p> The configuration that allows you to choose how you want to update the databases on a cluster. </p>
    pub fn deployment_configuration(mut self, input: crate::types::KxDeploymentConfiguration) -> Self {
        self.inner = self.inner.deployment_configuration(input);
        self
    }
    /// <p> The configuration that allows you to choose how you want to update the databases on a cluster. </p>
    pub fn set_deployment_configuration(mut self, input: ::std::option::Option<crate::types::KxDeploymentConfiguration>) -> Self {
        self.inner = self.inner.set_deployment_configuration(input);
        self
    }
    /// <p> The configuration that allows you to choose how you want to update the databases on a cluster. </p>
    pub fn get_deployment_configuration(&self) -> &::std::option::Option<crate::types::KxDeploymentConfiguration> {
        self.inner.get_deployment_configuration()
    }
}
