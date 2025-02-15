// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_replication_task_assessment_run::_start_replication_task_assessment_run_output::StartReplicationTaskAssessmentRunOutputBuilder;

pub use crate::operation::start_replication_task_assessment_run::_start_replication_task_assessment_run_input::StartReplicationTaskAssessmentRunInputBuilder;

impl StartReplicationTaskAssessmentRunInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_replication_task_assessment_run();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartReplicationTaskAssessmentRun`.
///
/// <p>Starts a new premigration assessment run for one or more individual assessments of a migration task.</p>
/// <p>The assessments that you can specify depend on the source and target database engine and the migration type defined for the given task. To run this operation, your migration task must already be created. After you run this operation, you can review the status of each individual assessment. You can also run the migration task manually after the assessment run and its individual assessments complete.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartReplicationTaskAssessmentRunFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_replication_task_assessment_run::builders::StartReplicationTaskAssessmentRunInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunOutput,
        crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunError,
    > for StartReplicationTaskAssessmentRunFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunOutput,
            crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartReplicationTaskAssessmentRunFluentBuilder {
    /// Creates a new `StartReplicationTaskAssessmentRun`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartReplicationTaskAssessmentRun as a reference.
    pub fn as_input(&self) -> &crate::operation::start_replication_task_assessment_run::builders::StartReplicationTaskAssessmentRunInputBuilder {
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
        crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRun::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRun::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunOutput,
        crate::operation::start_replication_task_assessment_run::StartReplicationTaskAssessmentRunError,
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
    /// <p>Amazon Resource Name (ARN) of the migration task associated with the premigration assessment run that you want to start.</p>
    pub fn replication_task_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.replication_task_arn(input.into());
        self
    }
    /// <p>Amazon Resource Name (ARN) of the migration task associated with the premigration assessment run that you want to start.</p>
    pub fn set_replication_task_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_replication_task_arn(input);
        self
    }
    /// <p>Amazon Resource Name (ARN) of the migration task associated with the premigration assessment run that you want to start.</p>
    pub fn get_replication_task_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_replication_task_arn()
    }
    /// <p>ARN of the service role needed to start the assessment run. The role must allow the <code>iam:PassRole</code> action.</p>
    pub fn service_access_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_access_role_arn(input.into());
        self
    }
    /// <p>ARN of the service role needed to start the assessment run. The role must allow the <code>iam:PassRole</code> action.</p>
    pub fn set_service_access_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_access_role_arn(input);
        self
    }
    /// <p>ARN of the service role needed to start the assessment run. The role must allow the <code>iam:PassRole</code> action.</p>
    pub fn get_service_access_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_access_role_arn()
    }
    /// <p>Amazon S3 bucket where you want DMS to store the results of this assessment run.</p>
    pub fn result_location_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.result_location_bucket(input.into());
        self
    }
    /// <p>Amazon S3 bucket where you want DMS to store the results of this assessment run.</p>
    pub fn set_result_location_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_result_location_bucket(input);
        self
    }
    /// <p>Amazon S3 bucket where you want DMS to store the results of this assessment run.</p>
    pub fn get_result_location_bucket(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_result_location_bucket()
    }
    /// <p>Folder within an Amazon S3 bucket where you want DMS to store the results of this assessment run.</p>
    pub fn result_location_folder(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.result_location_folder(input.into());
        self
    }
    /// <p>Folder within an Amazon S3 bucket where you want DMS to store the results of this assessment run.</p>
    pub fn set_result_location_folder(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_result_location_folder(input);
        self
    }
    /// <p>Folder within an Amazon S3 bucket where you want DMS to store the results of this assessment run.</p>
    pub fn get_result_location_folder(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_result_location_folder()
    }
    /// <p>Encryption mode that you can specify to encrypt the results of this assessment run. If you don't specify this request parameter, DMS stores the assessment run results without encryption. You can specify one of the options following:</p>
    /// <ul>
    /// <li> <p> <code>"SSE_S3"</code> – The server-side encryption provided as a default by Amazon S3.</p> </li>
    /// <li> <p> <code>"SSE_KMS"</code> – Key Management Service (KMS) encryption. This encryption can use either a custom KMS encryption key that you specify or the default KMS encryption key that DMS provides.</p> </li>
    /// </ul>
    pub fn result_encryption_mode(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.result_encryption_mode(input.into());
        self
    }
    /// <p>Encryption mode that you can specify to encrypt the results of this assessment run. If you don't specify this request parameter, DMS stores the assessment run results without encryption. You can specify one of the options following:</p>
    /// <ul>
    /// <li> <p> <code>"SSE_S3"</code> – The server-side encryption provided as a default by Amazon S3.</p> </li>
    /// <li> <p> <code>"SSE_KMS"</code> – Key Management Service (KMS) encryption. This encryption can use either a custom KMS encryption key that you specify or the default KMS encryption key that DMS provides.</p> </li>
    /// </ul>
    pub fn set_result_encryption_mode(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_result_encryption_mode(input);
        self
    }
    /// <p>Encryption mode that you can specify to encrypt the results of this assessment run. If you don't specify this request parameter, DMS stores the assessment run results without encryption. You can specify one of the options following:</p>
    /// <ul>
    /// <li> <p> <code>"SSE_S3"</code> – The server-side encryption provided as a default by Amazon S3.</p> </li>
    /// <li> <p> <code>"SSE_KMS"</code> – Key Management Service (KMS) encryption. This encryption can use either a custom KMS encryption key that you specify or the default KMS encryption key that DMS provides.</p> </li>
    /// </ul>
    pub fn get_result_encryption_mode(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_result_encryption_mode()
    }
    /// <p>ARN of a custom KMS encryption key that you specify when you set <code>ResultEncryptionMode</code> to <code>"SSE_KMS</code>".</p>
    pub fn result_kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.result_kms_key_arn(input.into());
        self
    }
    /// <p>ARN of a custom KMS encryption key that you specify when you set <code>ResultEncryptionMode</code> to <code>"SSE_KMS</code>".</p>
    pub fn set_result_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_result_kms_key_arn(input);
        self
    }
    /// <p>ARN of a custom KMS encryption key that you specify when you set <code>ResultEncryptionMode</code> to <code>"SSE_KMS</code>".</p>
    pub fn get_result_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_result_kms_key_arn()
    }
    /// <p>Unique name to identify the assessment run.</p>
    pub fn assessment_run_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.assessment_run_name(input.into());
        self
    }
    /// <p>Unique name to identify the assessment run.</p>
    pub fn set_assessment_run_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_assessment_run_name(input);
        self
    }
    /// <p>Unique name to identify the assessment run.</p>
    pub fn get_assessment_run_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_assessment_run_name()
    }
    /// Appends an item to `IncludeOnly`.
    ///
    /// To override the contents of this collection use [`set_include_only`](Self::set_include_only).
    ///
    /// <p>Space-separated list of names for specific individual assessments that you want to include. These names come from the default list of individual assessments that DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note>
    /// <p>You can't set a value for <code>IncludeOnly</code> if you also set a value for <code>Exclude</code> in the API operation. </p>
    /// <p>To identify the names of the default individual assessments that DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p>
    /// </note>
    pub fn include_only(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.include_only(input.into());
        self
    }
    /// <p>Space-separated list of names for specific individual assessments that you want to include. These names come from the default list of individual assessments that DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note>
    /// <p>You can't set a value for <code>IncludeOnly</code> if you also set a value for <code>Exclude</code> in the API operation. </p>
    /// <p>To identify the names of the default individual assessments that DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p>
    /// </note>
    pub fn set_include_only(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_include_only(input);
        self
    }
    /// <p>Space-separated list of names for specific individual assessments that you want to include. These names come from the default list of individual assessments that DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note>
    /// <p>You can't set a value for <code>IncludeOnly</code> if you also set a value for <code>Exclude</code> in the API operation. </p>
    /// <p>To identify the names of the default individual assessments that DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p>
    /// </note>
    pub fn get_include_only(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_include_only()
    }
    /// Appends an item to `Exclude`.
    ///
    /// To override the contents of this collection use [`set_exclude`](Self::set_exclude).
    ///
    /// <p>Space-separated list of names for specific individual assessments that you want to exclude. These names come from the default list of individual assessments that DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note>
    /// <p>You can't set a value for <code>Exclude</code> if you also set a value for <code>IncludeOnly</code> in the API operation.</p>
    /// <p>To identify the names of the default individual assessments that DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p>
    /// </note>
    pub fn exclude(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.exclude(input.into());
        self
    }
    /// <p>Space-separated list of names for specific individual assessments that you want to exclude. These names come from the default list of individual assessments that DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note>
    /// <p>You can't set a value for <code>Exclude</code> if you also set a value for <code>IncludeOnly</code> in the API operation.</p>
    /// <p>To identify the names of the default individual assessments that DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p>
    /// </note>
    pub fn set_exclude(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_exclude(input);
        self
    }
    /// <p>Space-separated list of names for specific individual assessments that you want to exclude. These names come from the default list of individual assessments that DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note>
    /// <p>You can't set a value for <code>Exclude</code> if you also set a value for <code>IncludeOnly</code> in the API operation.</p>
    /// <p>To identify the names of the default individual assessments that DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p>
    /// </note>
    pub fn get_exclude(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_exclude()
    }
}
