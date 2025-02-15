// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopModel`](crate::operation::stop_model::builders::StopModelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`project_name(impl Into<String>)`](crate::operation::stop_model::builders::StopModelFluentBuilder::project_name) / [`set_project_name(Option<String>)`](crate::operation::stop_model::builders::StopModelFluentBuilder::set_project_name):<br>required: **true**<br><p>The name of the project that contains the model that you want to stop.</p><br>
    ///   - [`model_version(impl Into<String>)`](crate::operation::stop_model::builders::StopModelFluentBuilder::model_version) / [`set_model_version(Option<String>)`](crate::operation::stop_model::builders::StopModelFluentBuilder::set_model_version):<br>required: **true**<br><p>The version of the model that you want to stop.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::stop_model::builders::StopModelFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::stop_model::builders::StopModelFluentBuilder::set_client_token):<br>required: **false**<br><p>ClientToken is an idempotency token that ensures a call to <code>StopModel</code> completes only once. You choose the value to pass. For example, An issue might prevent you from getting a response from <code>StopModel</code>. In this case, safely retry your call to <code>StopModel</code> by using the same <code>ClientToken</code> parameter value.</p>  <p>If you don't supply a value for <code>ClientToken</code>, the AWS SDK you are using inserts a value for you. This prevents retries after a network error from making multiple stop requests. You'll need to provide your own value for other use cases. </p>  <p>An error occurs if the other input parameters are not the same as in the first request. Using a different value for <code>ClientToken</code> is considered a new call to <code>StopModel</code>. An idempotency token is active for 8 hours. </p><br>
    /// - On success, responds with [`StopModelOutput`](crate::operation::stop_model::StopModelOutput) with field(s):
    ///   - [`status(Option<ModelHostingStatus>)`](crate::operation::stop_model::StopModelOutput::status): <p>The status of the model.</p>
    /// - On failure, responds with [`SdkError<StopModelError>`](crate::operation::stop_model::StopModelError)
    pub fn stop_model(&self) -> crate::operation::stop_model::builders::StopModelFluentBuilder {
        crate::operation::stop_model::builders::StopModelFluentBuilder::new(self.handle.clone())
    }
}
