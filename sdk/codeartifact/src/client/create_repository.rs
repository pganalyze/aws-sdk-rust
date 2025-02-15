// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateRepository`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain(impl Into<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::domain) / [`set_domain(Option<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_domain):<br>required: **true**<br><p> The name of the domain that contains the created repository. </p><br>
    ///   - [`domain_owner(impl Into<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::domain_owner) / [`set_domain_owner(Option<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_domain_owner):<br>required: **false**<br><p> The 12-digit account number of the Amazon Web Services account that owns the domain. It does not include dashes or spaces. </p><br>
    ///   - [`repository(impl Into<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::repository) / [`set_repository(Option<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_repository):<br>required: **true**<br><p> The name of the repository to create. </p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_description):<br>required: **false**<br><p> A description of the created repository. </p><br>
    ///   - [`upstreams(UpstreamRepository)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::upstreams) / [`set_upstreams(Option<Vec::<UpstreamRepository>>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_upstreams):<br>required: **false**<br><p> A list of upstream repositories to associate with the repository. The order of the upstream repositories in the list determines their priority order when CodeArtifact looks for a requested package version. For more information, see <a href="https://docs.aws.amazon.com/codeartifact/latest/ug/repos-upstream.html">Working with upstream repositories</a>. </p><br>
    ///   - [`tags(Tag)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::tags) / [`set_tags(Option<Vec::<Tag>>)`](crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::set_tags):<br>required: **false**<br><p>One or more tag key-value pairs for the repository.</p><br>
    /// - On success, responds with [`CreateRepositoryOutput`](crate::operation::create_repository::CreateRepositoryOutput) with field(s):
    ///   - [`repository(Option<RepositoryDescription>)`](crate::operation::create_repository::CreateRepositoryOutput::repository): <p> Information about the created repository after processing the request. </p>
    /// - On failure, responds with [`SdkError<CreateRepositoryError>`](crate::operation::create_repository::CreateRepositoryError)
    pub fn create_repository(&self) -> crate::operation::create_repository::builders::CreateRepositoryFluentBuilder {
        crate::operation::create_repository::builders::CreateRepositoryFluentBuilder::new(self.handle.clone())
    }
}
