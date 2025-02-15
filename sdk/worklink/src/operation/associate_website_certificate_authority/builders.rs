// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_website_certificate_authority::_associate_website_certificate_authority_output::AssociateWebsiteCertificateAuthorityOutputBuilder;

pub use crate::operation::associate_website_certificate_authority::_associate_website_certificate_authority_input::AssociateWebsiteCertificateAuthorityInputBuilder;

impl AssociateWebsiteCertificateAuthorityInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_website_certificate_authority();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateWebsiteCertificateAuthority`.
///
/// <p>Imports the root certificate of a certificate authority (CA) used to obtain TLS certificates used by associated websites within the company network.</p>
#[deprecated(note = "Amazon WorkLink is no longer supported. This will be removed in a future version of the SDK.")]
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateWebsiteCertificateAuthorityFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_website_certificate_authority::builders::AssociateWebsiteCertificateAuthorityInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityOutput,
        crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityError,
    > for AssociateWebsiteCertificateAuthorityFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityOutput,
            crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateWebsiteCertificateAuthorityFluentBuilder {
    /// Creates a new `AssociateWebsiteCertificateAuthority`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateWebsiteCertificateAuthority as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_website_certificate_authority::builders::AssociateWebsiteCertificateAuthorityInputBuilder {
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
        crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthority::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthority::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityOutput,
        crate::operation::associate_website_certificate_authority::AssociateWebsiteCertificateAuthorityError,
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
    /// <p>The ARN of the fleet.</p>
    pub fn fleet_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fleet_arn(input.into());
        self
    }
    /// <p>The ARN of the fleet.</p>
    pub fn set_fleet_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_arn(input);
        self
    }
    /// <p>The ARN of the fleet.</p>
    pub fn get_fleet_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fleet_arn()
    }
    /// <p>The root certificate of the CA.</p>
    pub fn certificate(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.certificate(input.into());
        self
    }
    /// <p>The root certificate of the CA.</p>
    pub fn set_certificate(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_certificate(input);
        self
    }
    /// <p>The root certificate of the CA.</p>
    pub fn get_certificate(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_certificate()
    }
    /// <p>The certificate name to display.</p>
    pub fn display_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.display_name(input.into());
        self
    }
    /// <p>The certificate name to display.</p>
    pub fn set_display_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_display_name(input);
        self
    }
    /// <p>The certificate name to display.</p>
    pub fn get_display_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_display_name()
    }
}
