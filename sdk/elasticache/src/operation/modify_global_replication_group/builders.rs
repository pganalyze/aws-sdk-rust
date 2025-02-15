// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_global_replication_group::_modify_global_replication_group_output::ModifyGlobalReplicationGroupOutputBuilder;

pub use crate::operation::modify_global_replication_group::_modify_global_replication_group_input::ModifyGlobalReplicationGroupInputBuilder;

impl ModifyGlobalReplicationGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_global_replication_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyGlobalReplicationGroup`.
///
/// <p>Modifies the settings for a Global datastore.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyGlobalReplicationGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_global_replication_group::builders::ModifyGlobalReplicationGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupOutput,
        crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupError,
    > for ModifyGlobalReplicationGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupOutput,
            crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyGlobalReplicationGroupFluentBuilder {
    /// Creates a new `ModifyGlobalReplicationGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyGlobalReplicationGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_global_replication_group::builders::ModifyGlobalReplicationGroupInputBuilder {
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
        crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupOutput,
        crate::operation::modify_global_replication_group::ModifyGlobalReplicationGroupError,
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
    /// <p>The name of the Global datastore</p>
    pub fn global_replication_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_replication_group_id(input.into());
        self
    }
    /// <p>The name of the Global datastore</p>
    pub fn set_global_replication_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_replication_group_id(input);
        self
    }
    /// <p>The name of the Global datastore</p>
    pub fn get_global_replication_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_replication_group_id()
    }
    /// <p>This parameter causes the modifications in this request and any pending modifications to be applied, asynchronously and as soon as possible. Modifications to Global Replication Groups cannot be requested to be applied in PreferredMaintenceWindow. </p>
    pub fn apply_immediately(mut self, input: bool) -> Self {
        self.inner = self.inner.apply_immediately(input);
        self
    }
    /// <p>This parameter causes the modifications in this request and any pending modifications to be applied, asynchronously and as soon as possible. Modifications to Global Replication Groups cannot be requested to be applied in PreferredMaintenceWindow. </p>
    pub fn set_apply_immediately(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_apply_immediately(input);
        self
    }
    /// <p>This parameter causes the modifications in this request and any pending modifications to be applied, asynchronously and as soon as possible. Modifications to Global Replication Groups cannot be requested to be applied in PreferredMaintenceWindow. </p>
    pub fn get_apply_immediately(&self) -> &::std::option::Option<bool> {
        self.inner.get_apply_immediately()
    }
    /// <p>A valid cache node type that you want to scale this Global datastore to.</p>
    pub fn cache_node_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cache_node_type(input.into());
        self
    }
    /// <p>A valid cache node type that you want to scale this Global datastore to.</p>
    pub fn set_cache_node_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cache_node_type(input);
        self
    }
    /// <p>A valid cache node type that you want to scale this Global datastore to.</p>
    pub fn get_cache_node_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cache_node_type()
    }
    /// <p>The upgraded version of the cache engine to be run on the clusters in the Global datastore. </p>
    pub fn engine_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.engine_version(input.into());
        self
    }
    /// <p>The upgraded version of the cache engine to be run on the clusters in the Global datastore. </p>
    pub fn set_engine_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_engine_version(input);
        self
    }
    /// <p>The upgraded version of the cache engine to be run on the clusters in the Global datastore. </p>
    pub fn get_engine_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_engine_version()
    }
    /// <p>The name of the cache parameter group to use with the Global datastore. It must be compatible with the major engine version used by the Global datastore.</p>
    pub fn cache_parameter_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cache_parameter_group_name(input.into());
        self
    }
    /// <p>The name of the cache parameter group to use with the Global datastore. It must be compatible with the major engine version used by the Global datastore.</p>
    pub fn set_cache_parameter_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cache_parameter_group_name(input);
        self
    }
    /// <p>The name of the cache parameter group to use with the Global datastore. It must be compatible with the major engine version used by the Global datastore.</p>
    pub fn get_cache_parameter_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cache_parameter_group_name()
    }
    /// <p>A description of the Global datastore</p>
    pub fn global_replication_group_description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.global_replication_group_description(input.into());
        self
    }
    /// <p>A description of the Global datastore</p>
    pub fn set_global_replication_group_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_global_replication_group_description(input);
        self
    }
    /// <p>A description of the Global datastore</p>
    pub fn get_global_replication_group_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_global_replication_group_description()
    }
    /// <p>Determines whether a read replica is automatically promoted to read/write primary if the existing primary encounters a failure. </p>
    pub fn automatic_failover_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.automatic_failover_enabled(input);
        self
    }
    /// <p>Determines whether a read replica is automatically promoted to read/write primary if the existing primary encounters a failure. </p>
    pub fn set_automatic_failover_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_automatic_failover_enabled(input);
        self
    }
    /// <p>Determines whether a read replica is automatically promoted to read/write primary if the existing primary encounters a failure. </p>
    pub fn get_automatic_failover_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_automatic_failover_enabled()
    }
}
