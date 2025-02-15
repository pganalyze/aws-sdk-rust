// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteApplicationCloudWatchLoggingOption`](crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_name(impl Into<String>)`](crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionFluentBuilder::application_name) / [`set_application_name(Option<String>)`](crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionFluentBuilder::set_application_name):<br>required: **true**<br><p>The Kinesis Analytics application name.</p><br>
    ///   - [`current_application_version_id(i64)`](crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionFluentBuilder::current_application_version_id) / [`set_current_application_version_id(Option<i64>)`](crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionFluentBuilder::set_current_application_version_id):<br>required: **true**<br><p>The version ID of the Kinesis Analytics application.</p><br>
    ///   - [`cloud_watch_logging_option_id(impl Into<String>)`](crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionFluentBuilder::cloud_watch_logging_option_id) / [`set_cloud_watch_logging_option_id(Option<String>)`](crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionFluentBuilder::set_cloud_watch_logging_option_id):<br>required: **true**<br><p>The <code>CloudWatchLoggingOptionId</code> of the CloudWatch logging option to delete. You can get the <code>CloudWatchLoggingOptionId</code> by using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation. </p><br>
    /// - On success, responds with [`DeleteApplicationCloudWatchLoggingOptionOutput`](crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionOutput)
    /// - On failure, responds with [`SdkError<DeleteApplicationCloudWatchLoggingOptionError>`](crate::operation::delete_application_cloud_watch_logging_option::DeleteApplicationCloudWatchLoggingOptionError)
    pub fn delete_application_cloud_watch_logging_option(
        &self,
    ) -> crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionFluentBuilder {
        crate::operation::delete_application_cloud_watch_logging_option::builders::DeleteApplicationCloudWatchLoggingOptionFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
