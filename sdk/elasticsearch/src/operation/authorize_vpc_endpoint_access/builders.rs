// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::authorize_vpc_endpoint_access::_authorize_vpc_endpoint_access_output::AuthorizeVpcEndpointAccessOutputBuilder;

pub use crate::operation::authorize_vpc_endpoint_access::_authorize_vpc_endpoint_access_input::AuthorizeVpcEndpointAccessInputBuilder;

impl AuthorizeVpcEndpointAccessInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.authorize_vpc_endpoint_access();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AuthorizeVpcEndpointAccess`.
///
/// <p>Provides access to an Amazon OpenSearch Service domain through the use of an interface VPC endpoint.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AuthorizeVpcEndpointAccessFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::authorize_vpc_endpoint_access::builders::AuthorizeVpcEndpointAccessInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessOutput,
        crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessError,
    > for AuthorizeVpcEndpointAccessFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessOutput,
            crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AuthorizeVpcEndpointAccessFluentBuilder {
    /// Creates a new `AuthorizeVpcEndpointAccess`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AuthorizeVpcEndpointAccess as a reference.
    pub fn as_input(&self) -> &crate::operation::authorize_vpc_endpoint_access::builders::AuthorizeVpcEndpointAccessInputBuilder {
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
        crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccess::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccess::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessOutput,
        crate::operation::authorize_vpc_endpoint_access::AuthorizeVpcEndpointAccessError,
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
    /// <p>The name of the OpenSearch Service domain to provide access to.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_name(input.into());
        self
    }
    /// <p>The name of the OpenSearch Service domain to provide access to.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_name(input);
        self
    }
    /// <p>The name of the OpenSearch Service domain to provide access to.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_name()
    }
    /// <p>The account ID to grant access to.</p>
    pub fn account(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account(input.into());
        self
    }
    /// <p>The account ID to grant access to.</p>
    pub fn set_account(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account(input);
        self
    }
    /// <p>The account ID to grant access to.</p>
    pub fn get_account(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account()
    }
}
