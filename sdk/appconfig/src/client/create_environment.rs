// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateEnvironment`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_application_id):<br>required: **true**<br><p>The application ID.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_name):<br>required: **true**<br><p>A name for the environment.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_description):<br>required: **false**<br><p>A description of the environment.</p><br>
    ///   - [`monitors(Monitor)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::monitors) / [`set_monitors(Option<Vec::<Monitor>>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_monitors):<br>required: **false**<br><p>Amazon CloudWatch alarms to monitor during the deployment process.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::set_tags):<br>required: **false**<br><p>Metadata to assign to the environment. Tags help organize and categorize your AppConfig resources. Each tag consists of a key and an optional value, both of which you define.</p><br>
    /// - On success, responds with [`CreateEnvironmentOutput`](crate::operation::create_environment::CreateEnvironmentOutput) with field(s):
    ///   - [`application_id(Option<String>)`](crate::operation::create_environment::CreateEnvironmentOutput::application_id): <p>The application ID.</p>
    ///   - [`id(Option<String>)`](crate::operation::create_environment::CreateEnvironmentOutput::id): <p>The environment ID.</p>
    ///   - [`name(Option<String>)`](crate::operation::create_environment::CreateEnvironmentOutput::name): <p>The name of the environment.</p>
    ///   - [`description(Option<String>)`](crate::operation::create_environment::CreateEnvironmentOutput::description): <p>The description of the environment.</p>
    ///   - [`state(Option<EnvironmentState>)`](crate::operation::create_environment::CreateEnvironmentOutput::state): <p>The state of the environment. An environment can be in one of the following states: <code>READY_FOR_DEPLOYMENT</code>, <code>DEPLOYING</code>, <code>ROLLING_BACK</code>, or <code>ROLLED_BACK</code> </p>
    ///   - [`monitors(Option<Vec::<Monitor>>)`](crate::operation::create_environment::CreateEnvironmentOutput::monitors): <p>Amazon CloudWatch alarms monitored during the deployment.</p>
    /// - On failure, responds with [`SdkError<CreateEnvironmentError>`](crate::operation::create_environment::CreateEnvironmentError)
    pub fn create_environment(&self) -> crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder {
        crate::operation::create_environment::builders::CreateEnvironmentFluentBuilder::new(self.handle.clone())
    }
}
