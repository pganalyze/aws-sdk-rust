// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_settings::_update_settings_output::UpdateSettingsOutputBuilder;

pub use crate::operation::update_settings::_update_settings_input::UpdateSettingsInputBuilder;

impl UpdateSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_settings::UpdateSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_settings::UpdateSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateSettings`.
///
/// <p> Updates Audit Manager settings for the current account. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_settings::builders::UpdateSettingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_settings::UpdateSettingsOutput,
        crate::operation::update_settings::UpdateSettingsError,
    > for UpdateSettingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_settings::UpdateSettingsOutput,
            crate::operation::update_settings::UpdateSettingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateSettingsFluentBuilder {
    /// Creates a new `UpdateSettings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::update_settings::builders::UpdateSettingsInputBuilder {
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
        crate::operation::update_settings::UpdateSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_settings::UpdateSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_settings::UpdateSettings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_settings::UpdateSettings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_settings::UpdateSettingsOutput,
        crate::operation::update_settings::UpdateSettingsError,
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
    /// <p> The Amazon Simple Notification Service (Amazon SNS) topic that Audit Manager sends notifications to. </p>
    pub fn sns_topic(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.sns_topic(input.into());
        self
    }
    /// <p> The Amazon Simple Notification Service (Amazon SNS) topic that Audit Manager sends notifications to. </p>
    pub fn set_sns_topic(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_sns_topic(input);
        self
    }
    /// <p> The Amazon Simple Notification Service (Amazon SNS) topic that Audit Manager sends notifications to. </p>
    pub fn get_sns_topic(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_sns_topic()
    }
    /// <p> The default S3 destination bucket for storing assessment reports. </p>
    pub fn default_assessment_reports_destination(mut self, input: crate::types::AssessmentReportsDestination) -> Self {
        self.inner = self.inner.default_assessment_reports_destination(input);
        self
    }
    /// <p> The default S3 destination bucket for storing assessment reports. </p>
    pub fn set_default_assessment_reports_destination(mut self, input: ::std::option::Option<crate::types::AssessmentReportsDestination>) -> Self {
        self.inner = self.inner.set_default_assessment_reports_destination(input);
        self
    }
    /// <p> The default S3 destination bucket for storing assessment reports. </p>
    pub fn get_default_assessment_reports_destination(&self) -> &::std::option::Option<crate::types::AssessmentReportsDestination> {
        self.inner.get_default_assessment_reports_destination()
    }
    /// Appends an item to `defaultProcessOwners`.
    ///
    /// To override the contents of this collection use [`set_default_process_owners`](Self::set_default_process_owners).
    ///
    /// <p> A list of the default audit owners. </p>
    pub fn default_process_owners(mut self, input: crate::types::Role) -> Self {
        self.inner = self.inner.default_process_owners(input);
        self
    }
    /// <p> A list of the default audit owners. </p>
    pub fn set_default_process_owners(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Role>>) -> Self {
        self.inner = self.inner.set_default_process_owners(input);
        self
    }
    /// <p> A list of the default audit owners. </p>
    pub fn get_default_process_owners(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Role>> {
        self.inner.get_default_process_owners()
    }
    /// <p> The KMS key details. </p>
    pub fn kms_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key(input.into());
        self
    }
    /// <p> The KMS key details. </p>
    pub fn set_kms_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key(input);
        self
    }
    /// <p> The KMS key details. </p>
    pub fn get_kms_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key()
    }
    /// <p>Specifies whether the evidence finder feature is enabled. Change this attribute to enable or disable evidence finder.</p> <important>
    /// <p>When you use this attribute to disable evidence finder, Audit Manager deletes the event data store that’s used to query your evidence data. As a result, you can’t re-enable evidence finder and use the feature again. Your only alternative is to <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_DeregisterAccount.html">deregister</a> and then <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_RegisterAccount.html">re-register</a> Audit Manager. </p>
    /// </important>
    pub fn evidence_finder_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.evidence_finder_enabled(input);
        self
    }
    /// <p>Specifies whether the evidence finder feature is enabled. Change this attribute to enable or disable evidence finder.</p> <important>
    /// <p>When you use this attribute to disable evidence finder, Audit Manager deletes the event data store that’s used to query your evidence data. As a result, you can’t re-enable evidence finder and use the feature again. Your only alternative is to <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_DeregisterAccount.html">deregister</a> and then <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_RegisterAccount.html">re-register</a> Audit Manager. </p>
    /// </important>
    pub fn set_evidence_finder_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_evidence_finder_enabled(input);
        self
    }
    /// <p>Specifies whether the evidence finder feature is enabled. Change this attribute to enable or disable evidence finder.</p> <important>
    /// <p>When you use this attribute to disable evidence finder, Audit Manager deletes the event data store that’s used to query your evidence data. As a result, you can’t re-enable evidence finder and use the feature again. Your only alternative is to <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_DeregisterAccount.html">deregister</a> and then <a href="https://docs.aws.amazon.com/audit-manager/latest/APIReference/API_RegisterAccount.html">re-register</a> Audit Manager. </p>
    /// </important>
    pub fn get_evidence_finder_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_evidence_finder_enabled()
    }
    /// <p>The deregistration policy for your Audit Manager data. You can use this attribute to determine how your data is handled when you deregister Audit Manager.</p>
    pub fn deregistration_policy(mut self, input: crate::types::DeregistrationPolicy) -> Self {
        self.inner = self.inner.deregistration_policy(input);
        self
    }
    /// <p>The deregistration policy for your Audit Manager data. You can use this attribute to determine how your data is handled when you deregister Audit Manager.</p>
    pub fn set_deregistration_policy(mut self, input: ::std::option::Option<crate::types::DeregistrationPolicy>) -> Self {
        self.inner = self.inner.set_deregistration_policy(input);
        self
    }
    /// <p>The deregistration policy for your Audit Manager data. You can use this attribute to determine how your data is handled when you deregister Audit Manager.</p>
    pub fn get_deregistration_policy(&self) -> &::std::option::Option<crate::types::DeregistrationPolicy> {
        self.inner.get_deregistration_policy()
    }
    /// <p> The default S3 destination bucket for storing evidence finder exports. </p>
    pub fn default_export_destination(mut self, input: crate::types::DefaultExportDestination) -> Self {
        self.inner = self.inner.default_export_destination(input);
        self
    }
    /// <p> The default S3 destination bucket for storing evidence finder exports. </p>
    pub fn set_default_export_destination(mut self, input: ::std::option::Option<crate::types::DefaultExportDestination>) -> Self {
        self.inner = self.inner.set_default_export_destination(input);
        self
    }
    /// <p> The default S3 destination bucket for storing evidence finder exports. </p>
    pub fn get_default_export_destination(&self) -> &::std::option::Option<crate::types::DefaultExportDestination> {
        self.inner.get_default_export_destination()
    }
}
