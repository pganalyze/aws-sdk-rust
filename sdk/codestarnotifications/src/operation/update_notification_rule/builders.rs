// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_notification_rule::_update_notification_rule_output::UpdateNotificationRuleOutputBuilder;

pub use crate::operation::update_notification_rule::_update_notification_rule_input::UpdateNotificationRuleInputBuilder;

impl UpdateNotificationRuleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_notification_rule::UpdateNotificationRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_notification_rule::UpdateNotificationRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_notification_rule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateNotificationRule`.
///
/// <p>Updates a notification rule for a resource. You can change the events that trigger the notification rule, the status of the rule, and the targets that receive the notifications.</p> <note>
/// <p>To add or remove tags for a notification rule, you must use <code>TagResource</code> and <code>UntagResource</code>.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateNotificationRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_notification_rule::builders::UpdateNotificationRuleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_notification_rule::UpdateNotificationRuleOutput,
        crate::operation::update_notification_rule::UpdateNotificationRuleError,
    > for UpdateNotificationRuleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_notification_rule::UpdateNotificationRuleOutput,
            crate::operation::update_notification_rule::UpdateNotificationRuleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateNotificationRuleFluentBuilder {
    /// Creates a new `UpdateNotificationRule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateNotificationRule as a reference.
    pub fn as_input(&self) -> &crate::operation::update_notification_rule::builders::UpdateNotificationRuleInputBuilder {
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
        crate::operation::update_notification_rule::UpdateNotificationRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_notification_rule::UpdateNotificationRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_notification_rule::UpdateNotificationRule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_notification_rule::UpdateNotificationRule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_notification_rule::UpdateNotificationRuleOutput,
        crate::operation::update_notification_rule::UpdateNotificationRuleError,
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
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the notification rule.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
    /// <p>The name of the notification rule.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the notification rule.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the notification rule.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The status of the notification rule. Valid statuses include enabled (sending notifications) or disabled (not sending notifications).</p>
    pub fn status(mut self, input: crate::types::NotificationRuleStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The status of the notification rule. Valid statuses include enabled (sending notifications) or disabled (not sending notifications).</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::NotificationRuleStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>The status of the notification rule. Valid statuses include enabled (sending notifications) or disabled (not sending notifications).</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::NotificationRuleStatus> {
        self.inner.get_status()
    }
    /// Appends an item to `EventTypeIds`.
    ///
    /// To override the contents of this collection use [`set_event_type_ids`](Self::set_event_type_ids).
    ///
    /// <p>A list of event types associated with this notification rule. For a complete list of event types and IDs, see <a href="https://docs.aws.amazon.com/codestar-notifications/latest/userguide/concepts.html#concepts-api">Notification concepts</a> in the <i>Developer Tools Console User Guide</i>.</p>
    pub fn event_type_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.event_type_ids(input.into());
        self
    }
    /// <p>A list of event types associated with this notification rule. For a complete list of event types and IDs, see <a href="https://docs.aws.amazon.com/codestar-notifications/latest/userguide/concepts.html#concepts-api">Notification concepts</a> in the <i>Developer Tools Console User Guide</i>.</p>
    pub fn set_event_type_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_event_type_ids(input);
        self
    }
    /// <p>A list of event types associated with this notification rule. For a complete list of event types and IDs, see <a href="https://docs.aws.amazon.com/codestar-notifications/latest/userguide/concepts.html#concepts-api">Notification concepts</a> in the <i>Developer Tools Console User Guide</i>.</p>
    pub fn get_event_type_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_event_type_ids()
    }
    /// Appends an item to `Targets`.
    ///
    /// To override the contents of this collection use [`set_targets`](Self::set_targets).
    ///
    /// <p>The address and type of the targets to receive notifications from this notification rule.</p>
    pub fn targets(mut self, input: crate::types::Target) -> Self {
        self.inner = self.inner.targets(input);
        self
    }
    /// <p>The address and type of the targets to receive notifications from this notification rule.</p>
    pub fn set_targets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Target>>) -> Self {
        self.inner = self.inner.set_targets(input);
        self
    }
    /// <p>The address and type of the targets to receive notifications from this notification rule.</p>
    pub fn get_targets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Target>> {
        self.inner.get_targets()
    }
    /// <p>The level of detail to include in the notifications for this resource. BASIC will include only the contents of the event as it would appear in Amazon CloudWatch. FULL will include any supplemental information provided by AWS CodeStar Notifications and/or the service for the resource for which the notification is created.</p>
    pub fn detail_type(mut self, input: crate::types::DetailType) -> Self {
        self.inner = self.inner.detail_type(input);
        self
    }
    /// <p>The level of detail to include in the notifications for this resource. BASIC will include only the contents of the event as it would appear in Amazon CloudWatch. FULL will include any supplemental information provided by AWS CodeStar Notifications and/or the service for the resource for which the notification is created.</p>
    pub fn set_detail_type(mut self, input: ::std::option::Option<crate::types::DetailType>) -> Self {
        self.inner = self.inner.set_detail_type(input);
        self
    }
    /// <p>The level of detail to include in the notifications for this resource. BASIC will include only the contents of the event as it would appear in Amazon CloudWatch. FULL will include any supplemental information provided by AWS CodeStar Notifications and/or the service for the resource for which the notification is created.</p>
    pub fn get_detail_type(&self) -> &::std::option::Option<crate::types::DetailType> {
        self.inner.get_detail_type()
    }
}
