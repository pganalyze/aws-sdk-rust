// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DistributeDatasetEntries`](crate::operation::distribute_dataset_entries::builders::DistributeDatasetEntriesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`datasets(DistributeDataset)`](crate::operation::distribute_dataset_entries::builders::DistributeDatasetEntriesFluentBuilder::datasets) / [`set_datasets(Option<Vec::<DistributeDataset>>)`](crate::operation::distribute_dataset_entries::builders::DistributeDatasetEntriesFluentBuilder::set_datasets):<br>required: **true**<br><p>The ARNS for the training dataset and test dataset that you want to use. The datasets must belong to the same project. The test dataset must be empty. </p><br>
    /// - On success, responds with [`DistributeDatasetEntriesOutput`](crate::operation::distribute_dataset_entries::DistributeDatasetEntriesOutput)
    /// - On failure, responds with [`SdkError<DistributeDatasetEntriesError>`](crate::operation::distribute_dataset_entries::DistributeDatasetEntriesError)
    pub fn distribute_dataset_entries(&self) -> crate::operation::distribute_dataset_entries::builders::DistributeDatasetEntriesFluentBuilder {
        crate::operation::distribute_dataset_entries::builders::DistributeDatasetEntriesFluentBuilder::new(self.handle.clone())
    }
}
