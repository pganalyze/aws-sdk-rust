// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreatePlatformApplication`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::set_name):<br>required: **true**<br><p>Application names must be made up of only uppercase and lowercase ASCII letters, numbers, underscores, hyphens, and periods, and must be between 1 and 256 characters long.</p><br>
    ///   - [`platform(impl Into<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::platform) / [`set_platform(Option<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::set_platform):<br>required: **true**<br><p>The following platforms are supported: ADM (Amazon Device Messaging), APNS (Apple Push Notification Service), APNS_SANDBOX, and GCM (Firebase Cloud Messaging).</p><br>
    ///   - [`attributes(impl Into<String>, impl Into<String>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::attributes) / [`set_attributes(Option<HashMap::<String, String>>)`](crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::set_attributes):<br>required: **true**<br><p>For a list of attributes, see <a href="https://docs.aws.amazon.com/sns/latest/api/API_SetPlatformApplicationAttributes.html">SetPlatformApplicationAttributes</a>.</p><br>
    /// - On success, responds with [`CreatePlatformApplicationOutput`](crate::operation::create_platform_application::CreatePlatformApplicationOutput) with field(s):
    ///   - [`platform_application_arn(Option<String>)`](crate::operation::create_platform_application::CreatePlatformApplicationOutput::platform_application_arn): <p>PlatformApplicationArn is returned.</p>
    /// - On failure, responds with [`SdkError<CreatePlatformApplicationError>`](crate::operation::create_platform_application::CreatePlatformApplicationError)
    pub fn create_platform_application(&self) -> crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder {
        crate::operation::create_platform_application::builders::CreatePlatformApplicationFluentBuilder::new(self.handle.clone())
    }
}
