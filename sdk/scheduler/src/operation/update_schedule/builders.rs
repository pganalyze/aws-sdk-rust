// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_schedule::_update_schedule_output::UpdateScheduleOutputBuilder;

pub use crate::operation::update_schedule::_update_schedule_input::UpdateScheduleInputBuilder;

impl UpdateScheduleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_schedule::UpdateScheduleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_schedule::UpdateScheduleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_schedule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateSchedule`.
///
/// <p> Updates the specified schedule. When you call <code>UpdateSchedule</code>, EventBridge Scheduler uses all values, including empty values, specified in the request and overrides the existing schedule. This is by design. This means that if you do not set an optional field in your request, that field will be set to its system-default value after the update. </p>
/// <p> Before calling this operation, we recommend that you call the <code>GetSchedule</code> API operation and make a note of all optional parameters for your <code>UpdateSchedule</code> call. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateScheduleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_schedule::builders::UpdateScheduleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_schedule::UpdateScheduleOutput,
        crate::operation::update_schedule::UpdateScheduleError,
    > for UpdateScheduleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_schedule::UpdateScheduleOutput,
            crate::operation::update_schedule::UpdateScheduleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateScheduleFluentBuilder {
    /// Creates a new `UpdateSchedule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateSchedule as a reference.
    pub fn as_input(&self) -> &crate::operation::update_schedule::builders::UpdateScheduleInputBuilder {
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
        crate::operation::update_schedule::UpdateScheduleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_schedule::UpdateScheduleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_schedule::UpdateSchedule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_schedule::UpdateSchedule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_schedule::UpdateScheduleOutput,
        crate::operation::update_schedule::UpdateScheduleError,
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
    /// <p>The name of the schedule that you are updating.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the schedule that you are updating.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the schedule that you are updating.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The name of the schedule group with which the schedule is associated. You must provide this value in order for EventBridge Scheduler to find the schedule you want to update. If you omit this value, EventBridge Scheduler assumes the group is associated to the default group.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_name(input.into());
        self
    }
    /// <p>The name of the schedule group with which the schedule is associated. You must provide this value in order for EventBridge Scheduler to find the schedule you want to update. If you omit this value, EventBridge Scheduler assumes the group is associated to the default group.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_name(input);
        self
    }
    /// <p>The name of the schedule group with which the schedule is associated. You must provide this value in order for EventBridge Scheduler to find the schedule you want to update. If you omit this value, EventBridge Scheduler assumes the group is associated to the default group.</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_name()
    }
    /// <p> The expression that defines when the schedule runs. The following formats are supported. </p>
    /// <ul>
    /// <li> <p> <code>at</code> expression - <code>at(yyyy-mm-ddThh:mm:ss)</code> </p> </li>
    /// <li> <p> <code>rate</code> expression - <code>rate(value unit)</code> </p> </li>
    /// <li> <p> <code>cron</code> expression - <code>cron(fields)</code> </p> </li>
    /// </ul>
    /// <p> You can use <code>at</code> expressions to create one-time schedules that invoke a target once, at the time and in the time zone, that you specify. You can use <code>rate</code> and <code>cron</code> expressions to create recurring schedules. Rate-based schedules are useful when you want to invoke a target at regular intervals, such as every 15 minutes or every five days. Cron-based schedules are useful when you want to invoke a target periodically at a specific time, such as at 8:00 am (UTC+0) every 1st day of the month. </p>
    /// <p> A <code>cron</code> expression consists of six fields separated by white spaces: <code>(minutes hours day_of_month month day_of_week year)</code>. </p>
    /// <p> A <code>rate</code> expression consists of a <i>value</i> as a positive integer, and a <i>unit</i> with the following options: <code>minute</code> | <code>minutes</code> | <code>hour</code> | <code>hours</code> | <code>day</code> | <code>days</code> </p>
    /// <p> For more information and examples, see <a href="https://docs.aws.amazon.com/scheduler/latest/UserGuide/schedule-types.html">Schedule types on EventBridge Scheduler</a> in the <i>EventBridge Scheduler User Guide</i>. </p>
    pub fn schedule_expression(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.schedule_expression(input.into());
        self
    }
    /// <p> The expression that defines when the schedule runs. The following formats are supported. </p>
    /// <ul>
    /// <li> <p> <code>at</code> expression - <code>at(yyyy-mm-ddThh:mm:ss)</code> </p> </li>
    /// <li> <p> <code>rate</code> expression - <code>rate(value unit)</code> </p> </li>
    /// <li> <p> <code>cron</code> expression - <code>cron(fields)</code> </p> </li>
    /// </ul>
    /// <p> You can use <code>at</code> expressions to create one-time schedules that invoke a target once, at the time and in the time zone, that you specify. You can use <code>rate</code> and <code>cron</code> expressions to create recurring schedules. Rate-based schedules are useful when you want to invoke a target at regular intervals, such as every 15 minutes or every five days. Cron-based schedules are useful when you want to invoke a target periodically at a specific time, such as at 8:00 am (UTC+0) every 1st day of the month. </p>
    /// <p> A <code>cron</code> expression consists of six fields separated by white spaces: <code>(minutes hours day_of_month month day_of_week year)</code>. </p>
    /// <p> A <code>rate</code> expression consists of a <i>value</i> as a positive integer, and a <i>unit</i> with the following options: <code>minute</code> | <code>minutes</code> | <code>hour</code> | <code>hours</code> | <code>day</code> | <code>days</code> </p>
    /// <p> For more information and examples, see <a href="https://docs.aws.amazon.com/scheduler/latest/UserGuide/schedule-types.html">Schedule types on EventBridge Scheduler</a> in the <i>EventBridge Scheduler User Guide</i>. </p>
    pub fn set_schedule_expression(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_schedule_expression(input);
        self
    }
    /// <p> The expression that defines when the schedule runs. The following formats are supported. </p>
    /// <ul>
    /// <li> <p> <code>at</code> expression - <code>at(yyyy-mm-ddThh:mm:ss)</code> </p> </li>
    /// <li> <p> <code>rate</code> expression - <code>rate(value unit)</code> </p> </li>
    /// <li> <p> <code>cron</code> expression - <code>cron(fields)</code> </p> </li>
    /// </ul>
    /// <p> You can use <code>at</code> expressions to create one-time schedules that invoke a target once, at the time and in the time zone, that you specify. You can use <code>rate</code> and <code>cron</code> expressions to create recurring schedules. Rate-based schedules are useful when you want to invoke a target at regular intervals, such as every 15 minutes or every five days. Cron-based schedules are useful when you want to invoke a target periodically at a specific time, such as at 8:00 am (UTC+0) every 1st day of the month. </p>
    /// <p> A <code>cron</code> expression consists of six fields separated by white spaces: <code>(minutes hours day_of_month month day_of_week year)</code>. </p>
    /// <p> A <code>rate</code> expression consists of a <i>value</i> as a positive integer, and a <i>unit</i> with the following options: <code>minute</code> | <code>minutes</code> | <code>hour</code> | <code>hours</code> | <code>day</code> | <code>days</code> </p>
    /// <p> For more information and examples, see <a href="https://docs.aws.amazon.com/scheduler/latest/UserGuide/schedule-types.html">Schedule types on EventBridge Scheduler</a> in the <i>EventBridge Scheduler User Guide</i>. </p>
    pub fn get_schedule_expression(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_schedule_expression()
    }
    /// <p>The date, in UTC, after which the schedule can begin invoking its target. Depending on the schedule's recurrence expression, invocations might occur on, or after, the <code>StartDate</code> you specify. EventBridge Scheduler ignores <code>StartDate</code> for one-time schedules.</p>
    pub fn start_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_date(input);
        self
    }
    /// <p>The date, in UTC, after which the schedule can begin invoking its target. Depending on the schedule's recurrence expression, invocations might occur on, or after, the <code>StartDate</code> you specify. EventBridge Scheduler ignores <code>StartDate</code> for one-time schedules.</p>
    pub fn set_start_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_date(input);
        self
    }
    /// <p>The date, in UTC, after which the schedule can begin invoking its target. Depending on the schedule's recurrence expression, invocations might occur on, or after, the <code>StartDate</code> you specify. EventBridge Scheduler ignores <code>StartDate</code> for one-time schedules.</p>
    pub fn get_start_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_date()
    }
    /// <p>The date, in UTC, before which the schedule can invoke its target. Depending on the schedule's recurrence expression, invocations might stop on, or before, the <code>EndDate</code> you specify. EventBridge Scheduler ignores <code>EndDate</code> for one-time schedules.</p>
    pub fn end_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_date(input);
        self
    }
    /// <p>The date, in UTC, before which the schedule can invoke its target. Depending on the schedule's recurrence expression, invocations might stop on, or before, the <code>EndDate</code> you specify. EventBridge Scheduler ignores <code>EndDate</code> for one-time schedules.</p>
    pub fn set_end_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_date(input);
        self
    }
    /// <p>The date, in UTC, before which the schedule can invoke its target. Depending on the schedule's recurrence expression, invocations might stop on, or before, the <code>EndDate</code> you specify. EventBridge Scheduler ignores <code>EndDate</code> for one-time schedules.</p>
    pub fn get_end_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_date()
    }
    /// <p>The description you specify for the schedule.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description you specify for the schedule.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description you specify for the schedule.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The timezone in which the scheduling expression is evaluated.</p>
    pub fn schedule_expression_timezone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.schedule_expression_timezone(input.into());
        self
    }
    /// <p>The timezone in which the scheduling expression is evaluated.</p>
    pub fn set_schedule_expression_timezone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_schedule_expression_timezone(input);
        self
    }
    /// <p>The timezone in which the scheduling expression is evaluated.</p>
    pub fn get_schedule_expression_timezone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_schedule_expression_timezone()
    }
    /// <p>Specifies whether the schedule is enabled or disabled.</p>
    pub fn state(mut self, input: crate::types::ScheduleState) -> Self {
        self.inner = self.inner.state(input);
        self
    }
    /// <p>Specifies whether the schedule is enabled or disabled.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::ScheduleState>) -> Self {
        self.inner = self.inner.set_state(input);
        self
    }
    /// <p>Specifies whether the schedule is enabled or disabled.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::ScheduleState> {
        self.inner.get_state()
    }
    /// <p>The ARN for the customer managed KMS key that that you want EventBridge Scheduler to use to encrypt and decrypt your data.</p>
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_arn(input.into());
        self
    }
    /// <p>The ARN for the customer managed KMS key that that you want EventBridge Scheduler to use to encrypt and decrypt your data.</p>
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_arn(input);
        self
    }
    /// <p>The ARN for the customer managed KMS key that that you want EventBridge Scheduler to use to encrypt and decrypt your data.</p>
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_arn()
    }
    /// <p>The schedule target. You can use this operation to change the target that your schedule invokes.</p>
    pub fn target(mut self, input: crate::types::Target) -> Self {
        self.inner = self.inner.target(input);
        self
    }
    /// <p>The schedule target. You can use this operation to change the target that your schedule invokes.</p>
    pub fn set_target(mut self, input: ::std::option::Option<crate::types::Target>) -> Self {
        self.inner = self.inner.set_target(input);
        self
    }
    /// <p>The schedule target. You can use this operation to change the target that your schedule invokes.</p>
    pub fn get_target(&self) -> &::std::option::Option<crate::types::Target> {
        self.inner.get_target()
    }
    /// <p>Allows you to configure a time window during which EventBridge Scheduler invokes the schedule.</p>
    pub fn flexible_time_window(mut self, input: crate::types::FlexibleTimeWindow) -> Self {
        self.inner = self.inner.flexible_time_window(input);
        self
    }
    /// <p>Allows you to configure a time window during which EventBridge Scheduler invokes the schedule.</p>
    pub fn set_flexible_time_window(mut self, input: ::std::option::Option<crate::types::FlexibleTimeWindow>) -> Self {
        self.inner = self.inner.set_flexible_time_window(input);
        self
    }
    /// <p>Allows you to configure a time window during which EventBridge Scheduler invokes the schedule.</p>
    pub fn get_flexible_time_window(&self) -> &::std::option::Option<crate::types::FlexibleTimeWindow> {
        self.inner.get_flexible_time_window()
    }
    /// <p> Unique, case-sensitive identifier you provide to ensure the idempotency of the request. If you do not specify a client token, EventBridge Scheduler uses a randomly generated token for the request to ensure idempotency. </p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p> Unique, case-sensitive identifier you provide to ensure the idempotency of the request. If you do not specify a client token, EventBridge Scheduler uses a randomly generated token for the request to ensure idempotency. </p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p> Unique, case-sensitive identifier you provide to ensure the idempotency of the request. If you do not specify a client token, EventBridge Scheduler uses a randomly generated token for the request to ensure idempotency. </p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>Specifies the action that EventBridge Scheduler applies to the schedule after the schedule completes invoking the target.</p>
    pub fn action_after_completion(mut self, input: crate::types::ActionAfterCompletion) -> Self {
        self.inner = self.inner.action_after_completion(input);
        self
    }
    /// <p>Specifies the action that EventBridge Scheduler applies to the schedule after the schedule completes invoking the target.</p>
    pub fn set_action_after_completion(mut self, input: ::std::option::Option<crate::types::ActionAfterCompletion>) -> Self {
        self.inner = self.inner.set_action_after_completion(input);
        self
    }
    /// <p>Specifies the action that EventBridge Scheduler applies to the schedule after the schedule completes invoking the target.</p>
    pub fn get_action_after_completion(&self) -> &::std::option::Option<crate::types::ActionAfterCompletion> {
        self.inner.get_action_after_completion()
    }
}
