// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_custom_routing_accelerator::_create_custom_routing_accelerator_output::CreateCustomRoutingAcceleratorOutputBuilder;

pub use crate::operation::create_custom_routing_accelerator::_create_custom_routing_accelerator_input::CreateCustomRoutingAcceleratorInputBuilder;

impl CreateCustomRoutingAcceleratorInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_custom_routing_accelerator();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateCustomRoutingAccelerator`.
///
/// <p>Create a custom routing accelerator. A custom routing accelerator directs traffic to one of possibly thousands of Amazon EC2 instance destinations running in a single or multiple virtual private clouds (VPC) subnet endpoints.</p>
/// <p>Be aware that, by default, all destination EC2 instances in a VPC subnet endpoint cannot receive traffic. To enable all destinations to receive traffic, or to specify individual port mappings that can receive traffic, see the <a href="https://docs.aws.amazon.com/global-accelerator/latest/api/API_AllowCustomRoutingTraffic.html"> AllowCustomRoutingTraffic</a> operation.</p> <important>
/// <p>Global Accelerator is a global service that supports endpoints in multiple Amazon Web Services Regions but you must specify the US West (Oregon) Region to create, update, or otherwise work with accelerators. That is, for example, specify <code>--region us-west-2</code> on Amazon Web Services CLI commands.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateCustomRoutingAcceleratorFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_custom_routing_accelerator::builders::CreateCustomRoutingAcceleratorInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorOutput,
        crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorError,
    > for CreateCustomRoutingAcceleratorFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorOutput,
            crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateCustomRoutingAcceleratorFluentBuilder {
    /// Creates a new `CreateCustomRoutingAccelerator`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateCustomRoutingAccelerator as a reference.
    pub fn as_input(&self) -> &crate::operation::create_custom_routing_accelerator::builders::CreateCustomRoutingAcceleratorInputBuilder {
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
        crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAccelerator::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAccelerator::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorOutput,
        crate::operation::create_custom_routing_accelerator::CreateCustomRoutingAcceleratorError,
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
    /// <p>The name of a custom routing accelerator. The name can have a maximum of 64 characters, must contain only alphanumeric characters or hyphens (-), and must not begin or end with a hyphen.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of a custom routing accelerator. The name can have a maximum of 64 characters, must contain only alphanumeric characters or hyphens (-), and must not begin or end with a hyphen.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of a custom routing accelerator. The name can have a maximum of 64 characters, must contain only alphanumeric characters or hyphens (-), and must not begin or end with a hyphen.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The IP address type that an accelerator supports. For a custom routing accelerator, the value must be IPV4.</p>
    pub fn ip_address_type(mut self, input: crate::types::IpAddressType) -> Self {
        self.inner = self.inner.ip_address_type(input);
        self
    }
    /// <p>The IP address type that an accelerator supports. For a custom routing accelerator, the value must be IPV4.</p>
    pub fn set_ip_address_type(mut self, input: ::std::option::Option<crate::types::IpAddressType>) -> Self {
        self.inner = self.inner.set_ip_address_type(input);
        self
    }
    /// <p>The IP address type that an accelerator supports. For a custom routing accelerator, the value must be IPV4.</p>
    pub fn get_ip_address_type(&self) -> &::std::option::Option<crate::types::IpAddressType> {
        self.inner.get_ip_address_type()
    }
    /// Appends an item to `IpAddresses`.
    ///
    /// To override the contents of this collection use [`set_ip_addresses`](Self::set_ip_addresses).
    ///
    /// <p>Optionally, if you've added your own IP address pool to Global Accelerator (BYOIP), you can choose an IPv4 address from your own pool to use for the accelerator's static IPv4 address when you create an accelerator. </p>
    /// <p>After you bring an address range to Amazon Web Services, it appears in your account as an address pool. When you create an accelerator, you can assign one IPv4 address from your range to it. Global Accelerator assigns you a second static IPv4 address from an Amazon IP address range. If you bring two IPv4 address ranges to Amazon Web Services, you can assign one IPv4 address from each range to your accelerator. This restriction is because Global Accelerator assigns each address range to a different network zone, for high availability.</p>
    /// <p>You can specify one or two addresses, separated by a space. Do not include the /32 suffix.</p>
    /// <p>Note that you can't update IP addresses for an existing accelerator. To change them, you must create a new accelerator with the new addresses.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring your own IP addresses (BYOIP)</a> in the <i>Global Accelerator Developer Guide</i>.</p>
    pub fn ip_addresses(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ip_addresses(input.into());
        self
    }
    /// <p>Optionally, if you've added your own IP address pool to Global Accelerator (BYOIP), you can choose an IPv4 address from your own pool to use for the accelerator's static IPv4 address when you create an accelerator. </p>
    /// <p>After you bring an address range to Amazon Web Services, it appears in your account as an address pool. When you create an accelerator, you can assign one IPv4 address from your range to it. Global Accelerator assigns you a second static IPv4 address from an Amazon IP address range. If you bring two IPv4 address ranges to Amazon Web Services, you can assign one IPv4 address from each range to your accelerator. This restriction is because Global Accelerator assigns each address range to a different network zone, for high availability.</p>
    /// <p>You can specify one or two addresses, separated by a space. Do not include the /32 suffix.</p>
    /// <p>Note that you can't update IP addresses for an existing accelerator. To change them, you must create a new accelerator with the new addresses.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring your own IP addresses (BYOIP)</a> in the <i>Global Accelerator Developer Guide</i>.</p>
    pub fn set_ip_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_ip_addresses(input);
        self
    }
    /// <p>Optionally, if you've added your own IP address pool to Global Accelerator (BYOIP), you can choose an IPv4 address from your own pool to use for the accelerator's static IPv4 address when you create an accelerator. </p>
    /// <p>After you bring an address range to Amazon Web Services, it appears in your account as an address pool. When you create an accelerator, you can assign one IPv4 address from your range to it. Global Accelerator assigns you a second static IPv4 address from an Amazon IP address range. If you bring two IPv4 address ranges to Amazon Web Services, you can assign one IPv4 address from each range to your accelerator. This restriction is because Global Accelerator assigns each address range to a different network zone, for high availability.</p>
    /// <p>You can specify one or two addresses, separated by a space. Do not include the /32 suffix.</p>
    /// <p>Note that you can't update IP addresses for an existing accelerator. To change them, you must create a new accelerator with the new addresses.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/using-byoip.html">Bring your own IP addresses (BYOIP)</a> in the <i>Global Accelerator Developer Guide</i>.</p>
    pub fn get_ip_addresses(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_ip_addresses()
    }
    /// <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p>
    /// <p>If the value is set to true, an accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.enabled(input);
        self
    }
    /// <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p>
    /// <p>If the value is set to true, an accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enabled(input);
        self
    }
    /// <p>Indicates whether an accelerator is enabled. The value is true or false. The default value is true. </p>
    /// <p>If the value is set to true, an accelerator cannot be deleted. If set to false, the accelerator can be deleted.</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_enabled()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    pub fn idempotency_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.idempotency_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    pub fn set_idempotency_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_idempotency_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency—that is, the uniqueness—of the request.</p>
    pub fn get_idempotency_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_idempotency_token()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Create tags for an accelerator.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in Global Accelerator</a> in the <i>Global Accelerator Developer Guide</i>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Create tags for an accelerator.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in Global Accelerator</a> in the <i>Global Accelerator Developer Guide</i>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Create tags for an accelerator.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/global-accelerator/latest/dg/tagging-in-global-accelerator.html">Tagging in Global Accelerator</a> in the <i>Global Accelerator Developer Guide</i>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
