// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_data_lake_organization_configuration::_get_data_lake_organization_configuration_output::GetDataLakeOrganizationConfigurationOutputBuilder;

pub use crate::operation::get_data_lake_organization_configuration::_get_data_lake_organization_configuration_input::GetDataLakeOrganizationConfigurationInputBuilder;

impl GetDataLakeOrganizationConfigurationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_data_lake_organization_configuration();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetDataLakeOrganizationConfiguration`.
///
/// <p>Retrieves the configuration that will be automatically set up for accounts added to the organization after the organization has onboarded to Amazon Security Lake. This API does not take input parameters.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetDataLakeOrganizationConfigurationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_data_lake_organization_configuration::builders::GetDataLakeOrganizationConfigurationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationOutput,
        crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationError,
    > for GetDataLakeOrganizationConfigurationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationOutput,
            crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetDataLakeOrganizationConfigurationFluentBuilder {
    /// Creates a new `GetDataLakeOrganizationConfiguration`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetDataLakeOrganizationConfiguration as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::get_data_lake_organization_configuration::builders::GetDataLakeOrganizationConfigurationInputBuilder {
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
        crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfiguration::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfiguration::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationOutput,
        crate::operation::get_data_lake_organization_configuration::GetDataLakeOrganizationConfigurationError,
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
}
