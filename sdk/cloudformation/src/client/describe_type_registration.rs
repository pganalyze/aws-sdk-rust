// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTypeRegistration`](crate::operation::describe_type_registration::builders::DescribeTypeRegistrationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`registration_token(impl Into<String>)`](crate::operation::describe_type_registration::builders::DescribeTypeRegistrationFluentBuilder::registration_token) / [`set_registration_token(Option<String>)`](crate::operation::describe_type_registration::builders::DescribeTypeRegistrationFluentBuilder::set_registration_token):<br>required: **true**<br><p>The identifier for this registration request.</p>  <p>This registration token is generated by CloudFormation when you initiate a registration request using <code>RegisterType</code>.</p><br>
    /// - On success, responds with [`DescribeTypeRegistrationOutput`](crate::operation::describe_type_registration::DescribeTypeRegistrationOutput) with field(s):
    ///   - [`progress_status(Option<RegistrationStatus>)`](crate::operation::describe_type_registration::DescribeTypeRegistrationOutput::progress_status): <p>The current status of the extension registration request.</p>
    ///   - [`description(Option<String>)`](crate::operation::describe_type_registration::DescribeTypeRegistrationOutput::description): <p>The description of the extension registration request.</p>
    ///   - [`type_arn(Option<String>)`](crate::operation::describe_type_registration::DescribeTypeRegistrationOutput::type_arn): <p>The Amazon Resource Name (ARN) of the extension being registered.</p>  <p>For registration requests with a <code>ProgressStatus</code> of other than <code>COMPLETE</code>, this will be <code>null</code>.</p>
    ///   - [`type_version_arn(Option<String>)`](crate::operation::describe_type_registration::DescribeTypeRegistrationOutput::type_version_arn): <p>The Amazon Resource Name (ARN) of this specific version of the extension being registered.</p>  <p>For registration requests with a <code>ProgressStatus</code> of other than <code>COMPLETE</code>, this will be <code>null</code>.</p>
    /// - On failure, responds with [`SdkError<DescribeTypeRegistrationError>`](crate::operation::describe_type_registration::DescribeTypeRegistrationError)
    pub fn describe_type_registration(&self) -> crate::operation::describe_type_registration::builders::DescribeTypeRegistrationFluentBuilder {
        crate::operation::describe_type_registration::builders::DescribeTypeRegistrationFluentBuilder::new(self.handle.clone())
    }
}
