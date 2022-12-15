// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateConnector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_connector`](crate::client::Client::create_connector).
///
/// See [`crate::client::fluent_builders::CreateConnector`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateConnector {
    _private: (),
}
impl CreateConnector {
    /// Creates a new builder-style object to manufacture [`CreateConnectorInput`](crate::input::CreateConnectorInput).
    pub fn builder() -> crate::input::create_connector_input::Builder {
        crate::input::create_connector_input::Builder::default()
    }
    /// Creates a new `CreateConnector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateConnector {
    type Output = std::result::Result<
        crate::output::CreateConnectorOutput,
        crate::error::CreateConnectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_connector_error(response)
        } else {
            crate::operation_deser::parse_create_connector_response(response)
        }
    }
}

/// Operation shape for `CreateCustomPlugin`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_custom_plugin`](crate::client::Client::create_custom_plugin).
///
/// See [`crate::client::fluent_builders::CreateCustomPlugin`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateCustomPlugin {
    _private: (),
}
impl CreateCustomPlugin {
    /// Creates a new builder-style object to manufacture [`CreateCustomPluginInput`](crate::input::CreateCustomPluginInput).
    pub fn builder() -> crate::input::create_custom_plugin_input::Builder {
        crate::input::create_custom_plugin_input::Builder::default()
    }
    /// Creates a new `CreateCustomPlugin` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCustomPlugin {
    type Output = std::result::Result<
        crate::output::CreateCustomPluginOutput,
        crate::error::CreateCustomPluginError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_custom_plugin_error(response)
        } else {
            crate::operation_deser::parse_create_custom_plugin_response(response)
        }
    }
}

/// Operation shape for `CreateWorkerConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_worker_configuration`](crate::client::Client::create_worker_configuration).
///
/// See [`crate::client::fluent_builders::CreateWorkerConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateWorkerConfiguration {
    _private: (),
}
impl CreateWorkerConfiguration {
    /// Creates a new builder-style object to manufacture [`CreateWorkerConfigurationInput`](crate::input::CreateWorkerConfigurationInput).
    pub fn builder() -> crate::input::create_worker_configuration_input::Builder {
        crate::input::create_worker_configuration_input::Builder::default()
    }
    /// Creates a new `CreateWorkerConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateWorkerConfiguration {
    type Output = std::result::Result<
        crate::output::CreateWorkerConfigurationOutput,
        crate::error::CreateWorkerConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_worker_configuration_error(response)
        } else {
            crate::operation_deser::parse_create_worker_configuration_response(response)
        }
    }
}

/// Operation shape for `DeleteConnector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_connector`](crate::client::Client::delete_connector).
///
/// See [`crate::client::fluent_builders::DeleteConnector`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteConnector {
    _private: (),
}
impl DeleteConnector {
    /// Creates a new builder-style object to manufacture [`DeleteConnectorInput`](crate::input::DeleteConnectorInput).
    pub fn builder() -> crate::input::delete_connector_input::Builder {
        crate::input::delete_connector_input::Builder::default()
    }
    /// Creates a new `DeleteConnector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteConnector {
    type Output = std::result::Result<
        crate::output::DeleteConnectorOutput,
        crate::error::DeleteConnectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_connector_error(response)
        } else {
            crate::operation_deser::parse_delete_connector_response(response)
        }
    }
}

/// Operation shape for `DeleteCustomPlugin`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_custom_plugin`](crate::client::Client::delete_custom_plugin).
///
/// See [`crate::client::fluent_builders::DeleteCustomPlugin`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteCustomPlugin {
    _private: (),
}
impl DeleteCustomPlugin {
    /// Creates a new builder-style object to manufacture [`DeleteCustomPluginInput`](crate::input::DeleteCustomPluginInput).
    pub fn builder() -> crate::input::delete_custom_plugin_input::Builder {
        crate::input::delete_custom_plugin_input::Builder::default()
    }
    /// Creates a new `DeleteCustomPlugin` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteCustomPlugin {
    type Output = std::result::Result<
        crate::output::DeleteCustomPluginOutput,
        crate::error::DeleteCustomPluginError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_custom_plugin_error(response)
        } else {
            crate::operation_deser::parse_delete_custom_plugin_response(response)
        }
    }
}

/// Operation shape for `DescribeConnector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_connector`](crate::client::Client::describe_connector).
///
/// See [`crate::client::fluent_builders::DescribeConnector`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeConnector {
    _private: (),
}
impl DescribeConnector {
    /// Creates a new builder-style object to manufacture [`DescribeConnectorInput`](crate::input::DescribeConnectorInput).
    pub fn builder() -> crate::input::describe_connector_input::Builder {
        crate::input::describe_connector_input::Builder::default()
    }
    /// Creates a new `DescribeConnector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeConnector {
    type Output = std::result::Result<
        crate::output::DescribeConnectorOutput,
        crate::error::DescribeConnectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_connector_error(response)
        } else {
            crate::operation_deser::parse_describe_connector_response(response)
        }
    }
}

/// Operation shape for `DescribeCustomPlugin`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_custom_plugin`](crate::client::Client::describe_custom_plugin).
///
/// See [`crate::client::fluent_builders::DescribeCustomPlugin`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeCustomPlugin {
    _private: (),
}
impl DescribeCustomPlugin {
    /// Creates a new builder-style object to manufacture [`DescribeCustomPluginInput`](crate::input::DescribeCustomPluginInput).
    pub fn builder() -> crate::input::describe_custom_plugin_input::Builder {
        crate::input::describe_custom_plugin_input::Builder::default()
    }
    /// Creates a new `DescribeCustomPlugin` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCustomPlugin {
    type Output = std::result::Result<
        crate::output::DescribeCustomPluginOutput,
        crate::error::DescribeCustomPluginError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_custom_plugin_error(response)
        } else {
            crate::operation_deser::parse_describe_custom_plugin_response(response)
        }
    }
}

/// Operation shape for `DescribeWorkerConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_worker_configuration`](crate::client::Client::describe_worker_configuration).
///
/// See [`crate::client::fluent_builders::DescribeWorkerConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeWorkerConfiguration {
    _private: (),
}
impl DescribeWorkerConfiguration {
    /// Creates a new builder-style object to manufacture [`DescribeWorkerConfigurationInput`](crate::input::DescribeWorkerConfigurationInput).
    pub fn builder() -> crate::input::describe_worker_configuration_input::Builder {
        crate::input::describe_worker_configuration_input::Builder::default()
    }
    /// Creates a new `DescribeWorkerConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeWorkerConfiguration {
    type Output = std::result::Result<
        crate::output::DescribeWorkerConfigurationOutput,
        crate::error::DescribeWorkerConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_worker_configuration_error(response)
        } else {
            crate::operation_deser::parse_describe_worker_configuration_response(response)
        }
    }
}

/// Operation shape for `ListConnectors`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_connectors`](crate::client::Client::list_connectors).
///
/// See [`crate::client::fluent_builders::ListConnectors`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListConnectors {
    _private: (),
}
impl ListConnectors {
    /// Creates a new builder-style object to manufacture [`ListConnectorsInput`](crate::input::ListConnectorsInput).
    pub fn builder() -> crate::input::list_connectors_input::Builder {
        crate::input::list_connectors_input::Builder::default()
    }
    /// Creates a new `ListConnectors` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListConnectors {
    type Output =
        std::result::Result<crate::output::ListConnectorsOutput, crate::error::ListConnectorsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_connectors_error(response)
        } else {
            crate::operation_deser::parse_list_connectors_response(response)
        }
    }
}

/// Operation shape for `ListCustomPlugins`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_custom_plugins`](crate::client::Client::list_custom_plugins).
///
/// See [`crate::client::fluent_builders::ListCustomPlugins`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListCustomPlugins {
    _private: (),
}
impl ListCustomPlugins {
    /// Creates a new builder-style object to manufacture [`ListCustomPluginsInput`](crate::input::ListCustomPluginsInput).
    pub fn builder() -> crate::input::list_custom_plugins_input::Builder {
        crate::input::list_custom_plugins_input::Builder::default()
    }
    /// Creates a new `ListCustomPlugins` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListCustomPlugins {
    type Output = std::result::Result<
        crate::output::ListCustomPluginsOutput,
        crate::error::ListCustomPluginsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_custom_plugins_error(response)
        } else {
            crate::operation_deser::parse_list_custom_plugins_response(response)
        }
    }
}

/// Operation shape for `ListWorkerConfigurations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_worker_configurations`](crate::client::Client::list_worker_configurations).
///
/// See [`crate::client::fluent_builders::ListWorkerConfigurations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListWorkerConfigurations {
    _private: (),
}
impl ListWorkerConfigurations {
    /// Creates a new builder-style object to manufacture [`ListWorkerConfigurationsInput`](crate::input::ListWorkerConfigurationsInput).
    pub fn builder() -> crate::input::list_worker_configurations_input::Builder {
        crate::input::list_worker_configurations_input::Builder::default()
    }
    /// Creates a new `ListWorkerConfigurations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListWorkerConfigurations {
    type Output = std::result::Result<
        crate::output::ListWorkerConfigurationsOutput,
        crate::error::ListWorkerConfigurationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_worker_configurations_error(response)
        } else {
            crate::operation_deser::parse_list_worker_configurations_response(response)
        }
    }
}

/// Operation shape for `UpdateConnector`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_connector`](crate::client::Client::update_connector).
///
/// See [`crate::client::fluent_builders::UpdateConnector`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateConnector {
    _private: (),
}
impl UpdateConnector {
    /// Creates a new builder-style object to manufacture [`UpdateConnectorInput`](crate::input::UpdateConnectorInput).
    pub fn builder() -> crate::input::update_connector_input::Builder {
        crate::input::update_connector_input::Builder::default()
    }
    /// Creates a new `UpdateConnector` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateConnector {
    type Output = std::result::Result<
        crate::output::UpdateConnectorOutput,
        crate::error::UpdateConnectorError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_connector_error(response)
        } else {
            crate::operation_deser::parse_update_connector_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
