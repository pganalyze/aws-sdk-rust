// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteDataSource`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::set_application_id):<br>required: **true**<br><p>The identifier of the Amazon Q application used with the data source connector.</p><br>
    ///   - [`index_id(impl Into<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::index_id) / [`set_index_id(Option<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::set_index_id):<br>required: **true**<br><p>The identifier of the index used with the data source connector.</p><br>
    ///   - [`data_source_id(impl Into<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::data_source_id) / [`set_data_source_id(Option<String>)`](crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::set_data_source_id):<br>required: **true**<br><p>The identifier of the data source connector that you want to delete. </p><br>
    /// - On success, responds with [`DeleteDataSourceOutput`](crate::operation::delete_data_source::DeleteDataSourceOutput)
    /// - On failure, responds with [`SdkError<DeleteDataSourceError>`](crate::operation::delete_data_source::DeleteDataSourceError)
    pub fn delete_data_source(&self) -> crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder {
        crate::operation::delete_data_source::builders::DeleteDataSourceFluentBuilder::new(self.handle.clone())
    }
}
