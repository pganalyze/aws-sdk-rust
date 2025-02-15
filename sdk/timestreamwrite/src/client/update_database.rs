// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateDatabase`](crate::operation::update_database::builders::UpdateDatabaseFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`database_name(impl Into<String>)`](crate::operation::update_database::builders::UpdateDatabaseFluentBuilder::database_name) / [`set_database_name(Option<String>)`](crate::operation::update_database::builders::UpdateDatabaseFluentBuilder::set_database_name):<br>required: **true**<br><p> The name of the database. </p><br>
    ///   - [`kms_key_id(impl Into<String>)`](crate::operation::update_database::builders::UpdateDatabaseFluentBuilder::kms_key_id) / [`set_kms_key_id(Option<String>)`](crate::operation::update_database::builders::UpdateDatabaseFluentBuilder::set_kms_key_id):<br>required: **true**<br><p> The identifier of the new KMS key (<code>KmsKeyId</code>) to be used to encrypt the data stored in the database. If the <code>KmsKeyId</code> currently registered with the database is the same as the <code>KmsKeyId</code> in the request, there will not be any update. </p>  <p>You can specify the <code>KmsKeyId</code> using any of the following:</p>  <ul>   <li> <p>Key ID: <code>1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Key ARN: <code>arn:aws:kms:us-east-1:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab</code> </p> </li>   <li> <p>Alias name: <code>alias/ExampleAlias</code> </p> </li>   <li> <p>Alias ARN: <code>arn:aws:kms:us-east-1:111122223333:alias/ExampleAlias</code> </p> </li>  </ul><br>
    /// - On success, responds with [`UpdateDatabaseOutput`](crate::operation::update_database::UpdateDatabaseOutput) with field(s):
    ///   - [`database(Option<Database>)`](crate::operation::update_database::UpdateDatabaseOutput::database): <p>A top-level container for a table. Databases and tables are the fundamental management concepts in Amazon Timestream. All tables in a database are encrypted with the same KMS key.</p>
    /// - On failure, responds with [`SdkError<UpdateDatabaseError>`](crate::operation::update_database::UpdateDatabaseError)
    pub fn update_database(&self) -> crate::operation::update_database::builders::UpdateDatabaseFluentBuilder {
        crate::operation::update_database::builders::UpdateDatabaseFluentBuilder::new(self.handle.clone())
    }
}
