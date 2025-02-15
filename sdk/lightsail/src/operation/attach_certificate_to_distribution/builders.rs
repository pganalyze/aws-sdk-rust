// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::attach_certificate_to_distribution::_attach_certificate_to_distribution_output::AttachCertificateToDistributionOutputBuilder;

pub use crate::operation::attach_certificate_to_distribution::_attach_certificate_to_distribution_input::AttachCertificateToDistributionInputBuilder;

impl AttachCertificateToDistributionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.attach_certificate_to_distribution();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AttachCertificateToDistribution`.
///
/// <p>Attaches an SSL/TLS certificate to your Amazon Lightsail content delivery network (CDN) distribution.</p>
/// <p>After the certificate is attached, your distribution accepts HTTPS traffic for all of the domains that are associated with the certificate.</p>
/// <p>Use the <code>CreateCertificate</code> action to create a certificate that you can attach to your distribution.</p> <important>
/// <p>Only certificates created in the <code>us-east-1</code> Amazon Web Services Region can be attached to Lightsail distributions. Lightsail distributions are global resources that can reference an origin in any Amazon Web Services Region, and distribute its content globally. However, all distributions are located in the <code>us-east-1</code> Region.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AttachCertificateToDistributionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::attach_certificate_to_distribution::builders::AttachCertificateToDistributionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionOutput,
        crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionError,
    > for AttachCertificateToDistributionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionOutput,
            crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AttachCertificateToDistributionFluentBuilder {
    /// Creates a new `AttachCertificateToDistribution`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AttachCertificateToDistribution as a reference.
    pub fn as_input(&self) -> &crate::operation::attach_certificate_to_distribution::builders::AttachCertificateToDistributionInputBuilder {
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
        crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::attach_certificate_to_distribution::AttachCertificateToDistribution::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::attach_certificate_to_distribution::AttachCertificateToDistribution::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionOutput,
        crate::operation::attach_certificate_to_distribution::AttachCertificateToDistributionError,
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
    /// <p>The name of the distribution that the certificate will be attached to.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn distribution_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.distribution_name(input.into());
        self
    }
    /// <p>The name of the distribution that the certificate will be attached to.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn set_distribution_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_distribution_name(input);
        self
    }
    /// <p>The name of the distribution that the certificate will be attached to.</p>
    /// <p>Use the <code>GetDistributions</code> action to get a list of distribution names that you can specify.</p>
    pub fn get_distribution_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_distribution_name()
    }
    /// <p>The name of the certificate to attach to a distribution.</p>
    /// <p>Only certificates with a status of <code>ISSUED</code> can be attached to a distribution.</p>
    /// <p>Use the <code>GetCertificates</code> action to get a list of certificate names that you can specify.</p> <note>
    /// <p>This is the name of the certificate resource type and is used only to reference the certificate in other API actions. It can be different than the domain name of the certificate. For example, your certificate name might be <code>WordPress-Blog-Certificate</code> and the domain name of the certificate might be <code>example.com</code>.</p>
    /// </note>
    pub fn certificate_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.certificate_name(input.into());
        self
    }
    /// <p>The name of the certificate to attach to a distribution.</p>
    /// <p>Only certificates with a status of <code>ISSUED</code> can be attached to a distribution.</p>
    /// <p>Use the <code>GetCertificates</code> action to get a list of certificate names that you can specify.</p> <note>
    /// <p>This is the name of the certificate resource type and is used only to reference the certificate in other API actions. It can be different than the domain name of the certificate. For example, your certificate name might be <code>WordPress-Blog-Certificate</code> and the domain name of the certificate might be <code>example.com</code>.</p>
    /// </note>
    pub fn set_certificate_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_certificate_name(input);
        self
    }
    /// <p>The name of the certificate to attach to a distribution.</p>
    /// <p>Only certificates with a status of <code>ISSUED</code> can be attached to a distribution.</p>
    /// <p>Use the <code>GetCertificates</code> action to get a list of certificate names that you can specify.</p> <note>
    /// <p>This is the name of the certificate resource type and is used only to reference the certificate in other API actions. It can be different than the domain name of the certificate. For example, your certificate name might be <code>WordPress-Blog-Certificate</code> and the domain name of the certificate might be <code>example.com</code>.</p>
    /// </note>
    pub fn get_certificate_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_certificate_name()
    }
}
