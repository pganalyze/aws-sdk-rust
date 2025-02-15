// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_qualification_from_worker::_disassociate_qualification_from_worker_output::DisassociateQualificationFromWorkerOutputBuilder;

pub use crate::operation::disassociate_qualification_from_worker::_disassociate_qualification_from_worker_input::DisassociateQualificationFromWorkerInputBuilder;

impl DisassociateQualificationFromWorkerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_qualification_from_worker();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateQualificationFromWorker`.
///
/// <p> The <code>DisassociateQualificationFromWorker</code> revokes a previously granted Qualification from a user. </p>
/// <p> You can provide a text message explaining why the Qualification was revoked. The user who had the Qualification can see this message. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateQualificationFromWorkerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_qualification_from_worker::builders::DisassociateQualificationFromWorkerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerOutput,
        crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerError,
    > for DisassociateQualificationFromWorkerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerOutput,
            crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateQualificationFromWorkerFluentBuilder {
    /// Creates a new `DisassociateQualificationFromWorker`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateQualificationFromWorker as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_qualification_from_worker::builders::DisassociateQualificationFromWorkerInputBuilder {
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
        crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorker::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorker::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerOutput,
        crate::operation::disassociate_qualification_from_worker::DisassociateQualificationFromWorkerError,
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
    /// <p>The ID of the Worker who possesses the Qualification to be revoked.</p>
    pub fn worker_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.worker_id(input.into());
        self
    }
    /// <p>The ID of the Worker who possesses the Qualification to be revoked.</p>
    pub fn set_worker_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_worker_id(input);
        self
    }
    /// <p>The ID of the Worker who possesses the Qualification to be revoked.</p>
    pub fn get_worker_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_worker_id()
    }
    /// <p>The ID of the Qualification type of the Qualification to be revoked.</p>
    pub fn qualification_type_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.qualification_type_id(input.into());
        self
    }
    /// <p>The ID of the Qualification type of the Qualification to be revoked.</p>
    pub fn set_qualification_type_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_qualification_type_id(input);
        self
    }
    /// <p>The ID of the Qualification type of the Qualification to be revoked.</p>
    pub fn get_qualification_type_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_qualification_type_id()
    }
    /// <p>A text message that explains why the Qualification was revoked. The user who had the Qualification sees this message.</p>
    pub fn reason(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reason(input.into());
        self
    }
    /// <p>A text message that explains why the Qualification was revoked. The user who had the Qualification sees this message.</p>
    pub fn set_reason(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_reason(input);
        self
    }
    /// <p>A text message that explains why the Qualification was revoked. The user who had the Qualification sees this message.</p>
    pub fn get_reason(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_reason()
    }
}
