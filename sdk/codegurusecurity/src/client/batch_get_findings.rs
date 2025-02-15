// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetFindings`](crate::operation::batch_get_findings::builders::BatchGetFindingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`finding_identifiers(FindingIdentifier)`](crate::operation::batch_get_findings::builders::BatchGetFindingsFluentBuilder::finding_identifiers) / [`set_finding_identifiers(Option<Vec::<FindingIdentifier>>)`](crate::operation::batch_get_findings::builders::BatchGetFindingsFluentBuilder::set_finding_identifiers):<br>required: **true**<br><p>A list of finding identifiers. Each identifier consists of a <code>scanName</code> and a <code>findingId</code>. You retrieve the <code>findingId</code> when you call <code>GetFindings</code>.</p><br>
    /// - On success, responds with [`BatchGetFindingsOutput`](crate::operation::batch_get_findings::BatchGetFindingsOutput) with field(s):
    ///   - [`findings(Vec::<Finding>)`](crate::operation::batch_get_findings::BatchGetFindingsOutput::findings): <p> A list of all requested findings.</p>
    ///   - [`failed_findings(Vec::<BatchGetFindingsError>)`](crate::operation::batch_get_findings::BatchGetFindingsOutput::failed_findings): <p>A list of errors for individual findings which were not fetched. Each BatchGetFindingsError contains the <code>scanName</code>, <code>findingId</code>, <code>errorCode</code> and error <code>message</code>.</p>
    /// - On failure, responds with [`SdkError<BatchGetFindingsError>`](crate::operation::batch_get_findings::BatchGetFindingsError)
    pub fn batch_get_findings(&self) -> crate::operation::batch_get_findings::builders::BatchGetFindingsFluentBuilder {
        crate::operation::batch_get_findings::builders::BatchGetFindingsFluentBuilder::new(self.handle.clone())
    }
}
