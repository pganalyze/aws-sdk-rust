// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_asset_bundle_export_job::_start_asset_bundle_export_job_output::StartAssetBundleExportJobOutputBuilder;

pub use crate::operation::start_asset_bundle_export_job::_start_asset_bundle_export_job_input::StartAssetBundleExportJobInputBuilder;

impl StartAssetBundleExportJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_asset_bundle_export_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartAssetBundleExportJob`.
///
/// <p>Starts an Asset Bundle export job.</p>
/// <p>An Asset Bundle export job exports specified Amazon QuickSight assets. You can also choose to export any asset dependencies in the same job. Export jobs run asynchronously and can be polled with a <code>DescribeAssetBundleExportJob</code> API call. When a job is successfully completed, a download URL that contains the exported assets is returned. The URL is valid for 5 minutes and can be refreshed with a <code>DescribeAssetBundleExportJob</code> API call. Each Amazon QuickSight account can run up to 5 export jobs concurrently.</p>
/// <p>The API caller must have the necessary permissions in their IAM role to access each resource before the resources can be exported.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartAssetBundleExportJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_asset_bundle_export_job::builders::StartAssetBundleExportJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobOutput,
        crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobError,
    > for StartAssetBundleExportJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobOutput,
            crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartAssetBundleExportJobFluentBuilder {
    /// Creates a new `StartAssetBundleExportJob`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartAssetBundleExportJob as a reference.
    pub fn as_input(&self) -> &crate::operation::start_asset_bundle_export_job::builders::StartAssetBundleExportJobInputBuilder {
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
        crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobOutput,
        crate::operation::start_asset_bundle_export_job::StartAssetBundleExportJobError,
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
    /// <p>The ID of the Amazon Web Services account to export assets from.</p>
    pub fn aws_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.aws_account_id(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account to export assets from.</p>
    pub fn set_aws_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_aws_account_id(input);
        self
    }
    /// <p>The ID of the Amazon Web Services account to export assets from.</p>
    pub fn get_aws_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_aws_account_id()
    }
    /// <p>The ID of the job. This ID is unique while the job is running. After the job is completed, you can reuse this ID for another job.</p>
    pub fn asset_bundle_export_job_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.asset_bundle_export_job_id(input.into());
        self
    }
    /// <p>The ID of the job. This ID is unique while the job is running. After the job is completed, you can reuse this ID for another job.</p>
    pub fn set_asset_bundle_export_job_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_asset_bundle_export_job_id(input);
        self
    }
    /// <p>The ID of the job. This ID is unique while the job is running. After the job is completed, you can reuse this ID for another job.</p>
    pub fn get_asset_bundle_export_job_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_asset_bundle_export_job_id()
    }
    /// Appends an item to `ResourceArns`.
    ///
    /// To override the contents of this collection use [`set_resource_arns`](Self::set_resource_arns).
    ///
    /// <p>An array of resource ARNs to export. The following resources are supported.</p>
    /// <ul>
    /// <li> <p> <code>Analysis</code> </p> </li>
    /// <li> <p> <code>Dashboard</code> </p> </li>
    /// <li> <p> <code>DataSet</code> </p> </li>
    /// <li> <p> <code>DataSource</code> </p> </li>
    /// <li> <p> <code>RefreshSchedule</code> </p> </li>
    /// <li> <p> <code>Theme</code> </p> </li>
    /// <li> <p> <code>VPCConnection</code> </p> </li>
    /// </ul>
    /// <p>The API caller must have the necessary permissions in their IAM role to access each resource before the resources can be exported.</p>
    pub fn resource_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_arns(input.into());
        self
    }
    /// <p>An array of resource ARNs to export. The following resources are supported.</p>
    /// <ul>
    /// <li> <p> <code>Analysis</code> </p> </li>
    /// <li> <p> <code>Dashboard</code> </p> </li>
    /// <li> <p> <code>DataSet</code> </p> </li>
    /// <li> <p> <code>DataSource</code> </p> </li>
    /// <li> <p> <code>RefreshSchedule</code> </p> </li>
    /// <li> <p> <code>Theme</code> </p> </li>
    /// <li> <p> <code>VPCConnection</code> </p> </li>
    /// </ul>
    /// <p>The API caller must have the necessary permissions in their IAM role to access each resource before the resources can be exported.</p>
    pub fn set_resource_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_resource_arns(input);
        self
    }
    /// <p>An array of resource ARNs to export. The following resources are supported.</p>
    /// <ul>
    /// <li> <p> <code>Analysis</code> </p> </li>
    /// <li> <p> <code>Dashboard</code> </p> </li>
    /// <li> <p> <code>DataSet</code> </p> </li>
    /// <li> <p> <code>DataSource</code> </p> </li>
    /// <li> <p> <code>RefreshSchedule</code> </p> </li>
    /// <li> <p> <code>Theme</code> </p> </li>
    /// <li> <p> <code>VPCConnection</code> </p> </li>
    /// </ul>
    /// <p>The API caller must have the necessary permissions in their IAM role to access each resource before the resources can be exported.</p>
    pub fn get_resource_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_resource_arns()
    }
    /// <p>A Boolean that determines whether all dependencies of each resource ARN are recursively exported with the job. For example, say you provided a Dashboard ARN to the <code>ResourceArns</code> parameter. If you set <code>IncludeAllDependencies</code> to <code>TRUE</code>, any theme, dataset, and data source resource that is a dependency of the dashboard is also exported.</p>
    pub fn include_all_dependencies(mut self, input: bool) -> Self {
        self.inner = self.inner.include_all_dependencies(input);
        self
    }
    /// <p>A Boolean that determines whether all dependencies of each resource ARN are recursively exported with the job. For example, say you provided a Dashboard ARN to the <code>ResourceArns</code> parameter. If you set <code>IncludeAllDependencies</code> to <code>TRUE</code>, any theme, dataset, and data source resource that is a dependency of the dashboard is also exported.</p>
    pub fn set_include_all_dependencies(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_all_dependencies(input);
        self
    }
    /// <p>A Boolean that determines whether all dependencies of each resource ARN are recursively exported with the job. For example, say you provided a Dashboard ARN to the <code>ResourceArns</code> parameter. If you set <code>IncludeAllDependencies</code> to <code>TRUE</code>, any theme, dataset, and data source resource that is a dependency of the dashboard is also exported.</p>
    pub fn get_include_all_dependencies(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_all_dependencies()
    }
    /// <p>The export data format.</p>
    pub fn export_format(mut self, input: crate::types::AssetBundleExportFormat) -> Self {
        self.inner = self.inner.export_format(input);
        self
    }
    /// <p>The export data format.</p>
    pub fn set_export_format(mut self, input: ::std::option::Option<crate::types::AssetBundleExportFormat>) -> Self {
        self.inner = self.inner.set_export_format(input);
        self
    }
    /// <p>The export data format.</p>
    pub fn get_export_format(&self) -> &::std::option::Option<crate::types::AssetBundleExportFormat> {
        self.inner.get_export_format()
    }
    /// <p>An optional collection of structures that generate CloudFormation parameters to override the existing resource property values when the resource is exported to a new CloudFormation template.</p>
    /// <p>Use this field if the <code>ExportFormat</code> field of a <code>StartAssetBundleExportJobRequest</code> API call is set to <code>CLOUDFORMATION_JSON</code>.</p>
    pub fn cloud_formation_override_property_configuration(
        mut self,
        input: crate::types::AssetBundleCloudFormationOverridePropertyConfiguration,
    ) -> Self {
        self.inner = self.inner.cloud_formation_override_property_configuration(input);
        self
    }
    /// <p>An optional collection of structures that generate CloudFormation parameters to override the existing resource property values when the resource is exported to a new CloudFormation template.</p>
    /// <p>Use this field if the <code>ExportFormat</code> field of a <code>StartAssetBundleExportJobRequest</code> API call is set to <code>CLOUDFORMATION_JSON</code>.</p>
    pub fn set_cloud_formation_override_property_configuration(
        mut self,
        input: ::std::option::Option<crate::types::AssetBundleCloudFormationOverridePropertyConfiguration>,
    ) -> Self {
        self.inner = self.inner.set_cloud_formation_override_property_configuration(input);
        self
    }
    /// <p>An optional collection of structures that generate CloudFormation parameters to override the existing resource property values when the resource is exported to a new CloudFormation template.</p>
    /// <p>Use this field if the <code>ExportFormat</code> field of a <code>StartAssetBundleExportJobRequest</code> API call is set to <code>CLOUDFORMATION_JSON</code>.</p>
    pub fn get_cloud_formation_override_property_configuration(
        &self,
    ) -> &::std::option::Option<crate::types::AssetBundleCloudFormationOverridePropertyConfiguration> {
        self.inner.get_cloud_formation_override_property_configuration()
    }
    /// <p>A Boolean that determines whether all permissions for each resource ARN are exported with the job. If you set <code>IncludePermissions</code> to <code>TRUE</code>, any permissions associated with each resource are exported. </p>
    pub fn include_permissions(mut self, input: bool) -> Self {
        self.inner = self.inner.include_permissions(input);
        self
    }
    /// <p>A Boolean that determines whether all permissions for each resource ARN are exported with the job. If you set <code>IncludePermissions</code> to <code>TRUE</code>, any permissions associated with each resource are exported. </p>
    pub fn set_include_permissions(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_permissions(input);
        self
    }
    /// <p>A Boolean that determines whether all permissions for each resource ARN are exported with the job. If you set <code>IncludePermissions</code> to <code>TRUE</code>, any permissions associated with each resource are exported. </p>
    pub fn get_include_permissions(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_permissions()
    }
    /// <p> A Boolean that determines whether all tags for each resource ARN are exported with the job. If you set <code>IncludeTags</code> to <code>TRUE</code>, any tags associated with each resource are exported.</p>
    pub fn include_tags(mut self, input: bool) -> Self {
        self.inner = self.inner.include_tags(input);
        self
    }
    /// <p> A Boolean that determines whether all tags for each resource ARN are exported with the job. If you set <code>IncludeTags</code> to <code>TRUE</code>, any tags associated with each resource are exported.</p>
    pub fn set_include_tags(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_tags(input);
        self
    }
    /// <p> A Boolean that determines whether all tags for each resource ARN are exported with the job. If you set <code>IncludeTags</code> to <code>TRUE</code>, any tags associated with each resource are exported.</p>
    pub fn get_include_tags(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_tags()
    }
    /// <p>An optional parameter that determines which validation strategy to use for the export job. If <code>StrictModeForAllResources</code> is set to <code>TRUE</code>, strict validation for every error is enforced. If it is set to <code>FALSE</code>, validation is skipped for specific UI errors that are shown as warnings. The default value for <code>StrictModeForAllResources</code> is <code>FALSE</code>.</p>
    pub fn validation_strategy(mut self, input: crate::types::AssetBundleExportJobValidationStrategy) -> Self {
        self.inner = self.inner.validation_strategy(input);
        self
    }
    /// <p>An optional parameter that determines which validation strategy to use for the export job. If <code>StrictModeForAllResources</code> is set to <code>TRUE</code>, strict validation for every error is enforced. If it is set to <code>FALSE</code>, validation is skipped for specific UI errors that are shown as warnings. The default value for <code>StrictModeForAllResources</code> is <code>FALSE</code>.</p>
    pub fn set_validation_strategy(mut self, input: ::std::option::Option<crate::types::AssetBundleExportJobValidationStrategy>) -> Self {
        self.inner = self.inner.set_validation_strategy(input);
        self
    }
    /// <p>An optional parameter that determines which validation strategy to use for the export job. If <code>StrictModeForAllResources</code> is set to <code>TRUE</code>, strict validation for every error is enforced. If it is set to <code>FALSE</code>, validation is skipped for specific UI errors that are shown as warnings. The default value for <code>StrictModeForAllResources</code> is <code>FALSE</code>.</p>
    pub fn get_validation_strategy(&self) -> &::std::option::Option<crate::types::AssetBundleExportJobValidationStrategy> {
        self.inner.get_validation_strategy()
    }
}
