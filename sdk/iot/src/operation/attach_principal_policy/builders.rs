// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_principal_policy::_attach_principal_policy_output::AttachPrincipalPolicyOutputBuilder;

pub use crate::operation::attach_principal_policy::_attach_principal_policy_input::AttachPrincipalPolicyInputBuilder;

impl AttachPrincipalPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::attach_principal_policy::AttachPrincipalPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::attach_principal_policy::AttachPrincipalPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.attach_principal_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AttachPrincipalPolicy`.
///
/// <p>Attaches the specified policy to the specified principal (certificate or other credential).</p>
/// <p> <b>Note:</b> This action is deprecated and works as expected for backward compatibility, but we won't add enhancements. Use <code>AttachPolicy</code> instead.</p>
/// <p>Requires permission to access the <a href="https://docs.aws.amazon.com/service-authorization/latest/reference/list_awsiot.html#awsiot-actions-as-permissions">AttachPrincipalPolicy</a> action.</p>
#[deprecated]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AttachPrincipalPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::attach_principal_policy::builders::AttachPrincipalPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::attach_principal_policy::AttachPrincipalPolicyOutput,
        crate::operation::attach_principal_policy::AttachPrincipalPolicyError,
    > for AttachPrincipalPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::attach_principal_policy::AttachPrincipalPolicyOutput,
            crate::operation::attach_principal_policy::AttachPrincipalPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AttachPrincipalPolicyFluentBuilder {
    /// Creates a new `AttachPrincipalPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AttachPrincipalPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::attach_principal_policy::builders::AttachPrincipalPolicyInputBuilder {
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
        crate::operation::attach_principal_policy::AttachPrincipalPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::attach_principal_policy::AttachPrincipalPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::attach_principal_policy::AttachPrincipalPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::attach_principal_policy::AttachPrincipalPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::attach_principal_policy::AttachPrincipalPolicyOutput,
        crate::operation::attach_principal_policy::AttachPrincipalPolicyError,
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
    /// <p>The policy name.</p>
    pub fn policy_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_name(input.into());
        self
    }
    /// <p>The policy name.</p>
    pub fn set_policy_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_name(input);
        self
    }
    /// <p>The policy name.</p>
    pub fn get_policy_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_name()
    }
    /// <p>The principal, which can be a certificate ARN (as returned from the CreateCertificate operation) or an Amazon Cognito ID.</p>
    pub fn principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal(input.into());
        self
    }
    /// <p>The principal, which can be a certificate ARN (as returned from the CreateCertificate operation) or an Amazon Cognito ID.</p>
    pub fn set_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_principal(input);
        self
    }
    /// <p>The principal, which can be a certificate ARN (as returned from the CreateCertificate operation) or an Amazon Cognito ID.</p>
    pub fn get_principal(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_principal()
    }
}
