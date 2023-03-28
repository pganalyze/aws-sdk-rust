// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `BatchMeterUsage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`batch_meter_usage`](crate::client::Client::batch_meter_usage).
///
/// `ParseStrictResponse` impl for `BatchMeterUsage`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct BatchMeterUsage {
    _private: (),
}
impl BatchMeterUsage {
    /// Creates a new builder-style object to manufacture [`BatchMeterUsageInput`](crate::input::BatchMeterUsageInput).
    pub fn builder() -> crate::input::batch_meter_usage_input::Builder {
        crate::input::batch_meter_usage_input::Builder::default()
    }
    /// Creates a new `BatchMeterUsage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for BatchMeterUsage {
    type Output = std::result::Result<
        crate::output::BatchMeterUsageOutput,
        crate::error::BatchMeterUsageError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_batch_meter_usage_error(response)
        } else {
            crate::operation_deser::parse_batch_meter_usage_response(response)
        }
    }
}

/// Operation shape for `MeterUsage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`meter_usage`](crate::client::Client::meter_usage).
///
/// `ParseStrictResponse` impl for `MeterUsage`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct MeterUsage {
    _private: (),
}
impl MeterUsage {
    /// Creates a new builder-style object to manufacture [`MeterUsageInput`](crate::input::MeterUsageInput).
    pub fn builder() -> crate::input::meter_usage_input::Builder {
        crate::input::meter_usage_input::Builder::default()
    }
    /// Creates a new `MeterUsage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for MeterUsage {
    type Output =
        std::result::Result<crate::output::MeterUsageOutput, crate::error::MeterUsageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_meter_usage_error(response)
        } else {
            crate::operation_deser::parse_meter_usage_response(response)
        }
    }
}

/// Operation shape for `RegisterUsage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`register_usage`](crate::client::Client::register_usage).
///
/// `ParseStrictResponse` impl for `RegisterUsage`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RegisterUsage {
    _private: (),
}
impl RegisterUsage {
    /// Creates a new builder-style object to manufacture [`RegisterUsageInput`](crate::input::RegisterUsageInput).
    pub fn builder() -> crate::input::register_usage_input::Builder {
        crate::input::register_usage_input::Builder::default()
    }
    /// Creates a new `RegisterUsage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RegisterUsage {
    type Output =
        std::result::Result<crate::output::RegisterUsageOutput, crate::error::RegisterUsageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_register_usage_error(response)
        } else {
            crate::operation_deser::parse_register_usage_response(response)
        }
    }
}

/// Operation shape for `ResolveCustomer`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`resolve_customer`](crate::client::Client::resolve_customer).
///
/// `ParseStrictResponse` impl for `ResolveCustomer`.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ResolveCustomer {
    _private: (),
}
impl ResolveCustomer {
    /// Creates a new builder-style object to manufacture [`ResolveCustomerInput`](crate::input::ResolveCustomerInput).
    pub fn builder() -> crate::input::resolve_customer_input::Builder {
        crate::input::resolve_customer_input::Builder::default()
    }
    /// Creates a new `ResolveCustomer` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ResolveCustomer {
    type Output = std::result::Result<
        crate::output::ResolveCustomerOutput,
        crate::error::ResolveCustomerError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        tracing::debug!(request_id = ?aws_http::request_id::RequestId::request_id(response));
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_resolve_customer_error(response)
        } else {
            crate::operation_deser::parse_resolve_customer_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
