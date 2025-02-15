// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteConnector`](crate::operation::delete_connector::builders::DeleteConnectorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`connector_arn(impl Into<String>)`](crate::operation::delete_connector::builders::DeleteConnectorFluentBuilder::connector_arn) / [`set_connector_arn(Option<String>)`](crate::operation::delete_connector::builders::DeleteConnectorFluentBuilder::set_connector_arn):<br>required: **true**<br><p> The Amazon Resource Name (ARN) that was returned when you called <a href="https://docs.aws.amazon.com/pca-connector-ad/latest/APIReference/API_CreateConnector.html">CreateConnector</a>.</p><br>
    /// - On success, responds with [`DeleteConnectorOutput`](crate::operation::delete_connector::DeleteConnectorOutput)
    /// - On failure, responds with [`SdkError<DeleteConnectorError>`](crate::operation::delete_connector::DeleteConnectorError)
    pub fn delete_connector(&self) -> crate::operation::delete_connector::builders::DeleteConnectorFluentBuilder {
        crate::operation::delete_connector::builders::DeleteConnectorFluentBuilder::new(self.handle.clone())
    }
}
