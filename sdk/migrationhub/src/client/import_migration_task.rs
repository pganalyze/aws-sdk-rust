// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ImportMigrationTask`](crate::operation::import_migration_task::builders::ImportMigrationTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`progress_update_stream(impl Into<String>)`](crate::operation::import_migration_task::builders::ImportMigrationTaskFluentBuilder::progress_update_stream) / [`set_progress_update_stream(Option<String>)`](crate::operation::import_migration_task::builders::ImportMigrationTaskFluentBuilder::set_progress_update_stream):<br>required: **true**<br><p>The name of the ProgressUpdateStream. &gt;</p><br>
    ///   - [`migration_task_name(impl Into<String>)`](crate::operation::import_migration_task::builders::ImportMigrationTaskFluentBuilder::migration_task_name) / [`set_migration_task_name(Option<String>)`](crate::operation::import_migration_task::builders::ImportMigrationTaskFluentBuilder::set_migration_task_name):<br>required: **true**<br><p>Unique identifier that references the migration task. <i>Do not store personal data in this field.</i> </p><br>
    ///   - [`dry_run(bool)`](crate::operation::import_migration_task::builders::ImportMigrationTaskFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::import_migration_task::builders::ImportMigrationTaskFluentBuilder::set_dry_run):<br>required: **false**<br><p>Optional boolean flag to indicate whether any effect should take place. Used to test if the caller has permission to make the call.</p><br>
    /// - On success, responds with [`ImportMigrationTaskOutput`](crate::operation::import_migration_task::ImportMigrationTaskOutput)
    /// - On failure, responds with [`SdkError<ImportMigrationTaskError>`](crate::operation::import_migration_task::ImportMigrationTaskError)
    pub fn import_migration_task(&self) -> crate::operation::import_migration_task::builders::ImportMigrationTaskFluentBuilder {
        crate::operation::import_migration_task::builders::ImportMigrationTaskFluentBuilder::new(self.handle.clone())
    }
}
