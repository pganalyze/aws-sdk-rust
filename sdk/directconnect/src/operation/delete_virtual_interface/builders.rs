// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_virtual_interface::_delete_virtual_interface_output::DeleteVirtualInterfaceOutputBuilder;

pub use crate::operation::delete_virtual_interface::_delete_virtual_interface_input::DeleteVirtualInterfaceInputBuilder;

impl DeleteVirtualInterfaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_virtual_interface::DeleteVirtualInterfaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_virtual_interface::DeleteVirtualInterfaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_virtual_interface();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteVirtualInterface`.
///
/// <p>Deletes a virtual interface.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteVirtualInterfaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_virtual_interface::builders::DeleteVirtualInterfaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_virtual_interface::DeleteVirtualInterfaceOutput,
        crate::operation::delete_virtual_interface::DeleteVirtualInterfaceError,
    > for DeleteVirtualInterfaceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_virtual_interface::DeleteVirtualInterfaceOutput,
            crate::operation::delete_virtual_interface::DeleteVirtualInterfaceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteVirtualInterfaceFluentBuilder {
    /// Creates a new `DeleteVirtualInterface`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteVirtualInterface as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_virtual_interface::builders::DeleteVirtualInterfaceInputBuilder {
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
        crate::operation::delete_virtual_interface::DeleteVirtualInterfaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_virtual_interface::DeleteVirtualInterfaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_virtual_interface::DeleteVirtualInterface::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_virtual_interface::DeleteVirtualInterface::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_virtual_interface::DeleteVirtualInterfaceOutput,
        crate::operation::delete_virtual_interface::DeleteVirtualInterfaceError,
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
    /// <p>The ID of the virtual interface.</p>
    pub fn virtual_interface_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.virtual_interface_id(input.into());
        self
    }
    /// <p>The ID of the virtual interface.</p>
    pub fn set_virtual_interface_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_virtual_interface_id(input);
        self
    }
    /// <p>The ID of the virtual interface.</p>
    pub fn get_virtual_interface_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_virtual_interface_id()
    }
}
