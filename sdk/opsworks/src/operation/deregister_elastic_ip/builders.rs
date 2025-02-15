// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deregister_elastic_ip::_deregister_elastic_ip_output::DeregisterElasticIpOutputBuilder;

pub use crate::operation::deregister_elastic_ip::_deregister_elastic_ip_input::DeregisterElasticIpInputBuilder;

impl DeregisterElasticIpInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::deregister_elastic_ip::DeregisterElasticIpOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deregister_elastic_ip::DeregisterElasticIpError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.deregister_elastic_ip();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeregisterElasticIp`.
///
/// <p>Deregisters a specified Elastic IP address. The address can then be registered by another stack. For more information, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/resources.html">Resource Management</a>.</p>
/// <p> <b>Required Permissions</b>: To use this action, an IAM user must have a Manage permissions level for the stack, or an attached policy that explicitly grants permissions. For more information on user permissions, see <a href="https://docs.aws.amazon.com/opsworks/latest/userguide/opsworks-security-users.html">Managing User Permissions</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeregisterElasticIpFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deregister_elastic_ip::builders::DeregisterElasticIpInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::deregister_elastic_ip::DeregisterElasticIpOutput,
        crate::operation::deregister_elastic_ip::DeregisterElasticIpError,
    > for DeregisterElasticIpFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::deregister_elastic_ip::DeregisterElasticIpOutput,
            crate::operation::deregister_elastic_ip::DeregisterElasticIpError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeregisterElasticIpFluentBuilder {
    /// Creates a new `DeregisterElasticIp`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeregisterElasticIp as a reference.
    pub fn as_input(&self) -> &crate::operation::deregister_elastic_ip::builders::DeregisterElasticIpInputBuilder {
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
        crate::operation::deregister_elastic_ip::DeregisterElasticIpOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deregister_elastic_ip::DeregisterElasticIpError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::deregister_elastic_ip::DeregisterElasticIp::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::deregister_elastic_ip::DeregisterElasticIp::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::deregister_elastic_ip::DeregisterElasticIpOutput,
        crate::operation::deregister_elastic_ip::DeregisterElasticIpError,
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
    /// <p>The Elastic IP address.</p>
    pub fn elastic_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.elastic_ip(input.into());
        self
    }
    /// <p>The Elastic IP address.</p>
    pub fn set_elastic_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_elastic_ip(input);
        self
    }
    /// <p>The Elastic IP address.</p>
    pub fn get_elastic_ip(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_elastic_ip()
    }
}
