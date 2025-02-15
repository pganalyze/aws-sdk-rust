// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDataSource`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_name(impl Into<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::domain_name) / [`set_domain_name(Option<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::set_domain_name):<br>required: **true**<br><p>The name of the domain.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::set_name):<br>required: **true**<br><p>The name of the data source.</p><br>
    /// - On success, responds with [`DeleteDataSourceOutput`](crate::operation::delete_data_source::DeleteDataSourceOutput) with field(s):
    ///   - [`message(Option<String>)`](crate::operation::delete_data_source::DeleteDataSourceOutput::message): <p>A message associated with the initiated request.</p>
    /// - On failure, responds with [`SdkError<DeleteDataSourceError>`](crate::operation::delete_data_source::DeleteDataSourceError)
    pub fn delete_data_source(&self) -> crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder {
        crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::new(self.handle.clone())
    }
}
