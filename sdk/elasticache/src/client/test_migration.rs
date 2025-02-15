// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`TestMigration`](crate::operation::test_migration::builders::TestMigrationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`replication_group_id(impl Into<String>)`](crate::operation::test_migration::builders::TestMigrationFluentBuilder::replication_group_id) / [`set_replication_group_id(Option<String>)`](crate::operation::test_migration::builders::TestMigrationFluentBuilder::set_replication_group_id):<br>required: **true**<br><p> The ID of the replication group to which data is to be migrated. </p><br>
    ///   - [`customer_node_endpoint_list(CustomerNodeEndpoint)`](crate::operation::test_migration::builders::TestMigrationFluentBuilder::customer_node_endpoint_list) / [`set_customer_node_endpoint_list(Option<Vec::<CustomerNodeEndpoint>>)`](crate::operation::test_migration::builders::TestMigrationFluentBuilder::set_customer_node_endpoint_list):<br>required: **true**<br><p> List of endpoints from which data should be migrated. List should have only one element. </p><br>
    /// - On success, responds with [`TestMigrationOutput`](crate::operation::test_migration::TestMigrationOutput) with field(s):
    ///   - [`replication_group(Option<ReplicationGroup>)`](crate::operation::test_migration::TestMigrationOutput::replication_group): <p>Contains all of the attributes of a specific Redis replication group.</p>
    /// - On failure, responds with [`SdkError<TestMigrationError>`](crate::operation::test_migration::TestMigrationError)
    pub fn test_migration(&self) -> crate::operation::test_migration::builders::TestMigrationFluentBuilder {
        crate::operation::test_migration::builders::TestMigrationFluentBuilder::new(self.handle.clone())
    }
}
