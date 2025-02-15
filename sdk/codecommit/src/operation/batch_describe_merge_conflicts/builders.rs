// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_describe_merge_conflicts::_batch_describe_merge_conflicts_output::BatchDescribeMergeConflictsOutputBuilder;

pub use crate::operation::batch_describe_merge_conflicts::_batch_describe_merge_conflicts_input::BatchDescribeMergeConflictsInputBuilder;

impl BatchDescribeMergeConflictsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_describe_merge_conflicts();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchDescribeMergeConflicts`.
///
/// <p>Returns information about one or more merge conflicts in the attempted merge of two commit specifiers using the squash or three-way merge strategy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchDescribeMergeConflictsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_describe_merge_conflicts::builders::BatchDescribeMergeConflictsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsOutput,
        crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsError,
    > for BatchDescribeMergeConflictsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsOutput,
            crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchDescribeMergeConflictsFluentBuilder {
    /// Creates a new `BatchDescribeMergeConflicts`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchDescribeMergeConflicts as a reference.
    pub fn as_input(&self) -> &crate::operation::batch_describe_merge_conflicts::builders::BatchDescribeMergeConflictsInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflicts::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflicts::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsOutput,
        crate::operation::batch_describe_merge_conflicts::BatchDescribeMergeConflictsError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The name of the repository that contains the merge conflicts you want to review.</p>
    pub fn repository_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.repository_name(input.into());
        self
    }
    /// <p>The name of the repository that contains the merge conflicts you want to review.</p>
    pub fn set_repository_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_repository_name(input);
        self
    }
    /// <p>The name of the repository that contains the merge conflicts you want to review.</p>
    pub fn get_repository_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_repository_name()
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn destination_commit_specifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_commit_specifier(input.into());
        self
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn set_destination_commit_specifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_commit_specifier(input);
        self
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn get_destination_commit_specifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_commit_specifier()
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn source_commit_specifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_commit_specifier(input.into());
        self
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn set_source_commit_specifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_commit_specifier(input);
        self
    }
    /// <p>The branch, tag, HEAD, or other fully qualified reference used to identify a commit (for example, a branch name or a full commit ID).</p>
    pub fn get_source_commit_specifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_commit_specifier()
    }
    /// <p>The merge option or strategy you want to use to merge the code.</p>
    pub fn merge_option(mut self, input: crate::types::MergeOptionTypeEnum) -> Self {
        self.inner = self.inner.merge_option(input);
        self
    }
    /// <p>The merge option or strategy you want to use to merge the code.</p>
    pub fn set_merge_option(mut self, input: ::std::option::Option<crate::types::MergeOptionTypeEnum>) -> Self {
        self.inner = self.inner.set_merge_option(input);
        self
    }
    /// <p>The merge option or strategy you want to use to merge the code.</p>
    pub fn get_merge_option(&self) -> &::std::option::Option<crate::types::MergeOptionTypeEnum> {
        self.inner.get_merge_option()
    }
    /// <p>The maximum number of merge hunks to include in the output.</p>
    pub fn max_merge_hunks(mut self, input: i32) -> Self {
        self.inner = self.inner.max_merge_hunks(input);
        self
    }
    /// <p>The maximum number of merge hunks to include in the output.</p>
    pub fn set_max_merge_hunks(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_merge_hunks(input);
        self
    }
    /// <p>The maximum number of merge hunks to include in the output.</p>
    pub fn get_max_merge_hunks(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_merge_hunks()
    }
    /// <p>The maximum number of files to include in the output.</p>
    pub fn max_conflict_files(mut self, input: i32) -> Self {
        self.inner = self.inner.max_conflict_files(input);
        self
    }
    /// <p>The maximum number of files to include in the output.</p>
    pub fn set_max_conflict_files(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_conflict_files(input);
        self
    }
    /// <p>The maximum number of files to include in the output.</p>
    pub fn get_max_conflict_files(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_conflict_files()
    }
    /// Appends an item to `filePaths`.
    ///
    /// To override the contents of this collection use [`set_file_paths`](Self::set_file_paths).
    ///
    /// <p>The path of the target files used to describe the conflicts. If not specified, the default is all conflict files.</p>
    pub fn file_paths(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.file_paths(input.into());
        self
    }
    /// <p>The path of the target files used to describe the conflicts. If not specified, the default is all conflict files.</p>
    pub fn set_file_paths(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_file_paths(input);
        self
    }
    /// <p>The path of the target files used to describe the conflicts. If not specified, the default is all conflict files.</p>
    pub fn get_file_paths(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_file_paths()
    }
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    pub fn conflict_detail_level(mut self, input: crate::types::ConflictDetailLevelTypeEnum) -> Self {
        self.inner = self.inner.conflict_detail_level(input);
        self
    }
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    pub fn set_conflict_detail_level(mut self, input: ::std::option::Option<crate::types::ConflictDetailLevelTypeEnum>) -> Self {
        self.inner = self.inner.set_conflict_detail_level(input);
        self
    }
    /// <p>The level of conflict detail to use. If unspecified, the default FILE_LEVEL is used, which returns a not-mergeable result if the same file has differences in both branches. If LINE_LEVEL is specified, a conflict is considered not mergeable if the same file in both branches has differences on the same line.</p>
    pub fn get_conflict_detail_level(&self) -> &::std::option::Option<crate::types::ConflictDetailLevelTypeEnum> {
        self.inner.get_conflict_detail_level()
    }
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    pub fn conflict_resolution_strategy(mut self, input: crate::types::ConflictResolutionStrategyTypeEnum) -> Self {
        self.inner = self.inner.conflict_resolution_strategy(input);
        self
    }
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    pub fn set_conflict_resolution_strategy(mut self, input: ::std::option::Option<crate::types::ConflictResolutionStrategyTypeEnum>) -> Self {
        self.inner = self.inner.set_conflict_resolution_strategy(input);
        self
    }
    /// <p>Specifies which branch to use when resolving conflicts, or whether to attempt automatically merging two versions of a file. The default is NONE, which requires any conflicts to be resolved manually before the merge operation is successful.</p>
    pub fn get_conflict_resolution_strategy(&self) -> &::std::option::Option<crate::types::ConflictResolutionStrategyTypeEnum> {
        self.inner.get_conflict_resolution_strategy()
    }
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>An enumeration token that, when provided in a request, returns the next batch of the results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
