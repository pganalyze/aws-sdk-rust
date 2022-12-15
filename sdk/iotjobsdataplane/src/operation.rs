// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `DescribeJobExecution`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_job_execution`](crate::client::Client::describe_job_execution).
///
/// See [`crate::client::fluent_builders::DescribeJobExecution`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeJobExecution {
    _private: (),
}
impl DescribeJobExecution {
    /// Creates a new builder-style object to manufacture [`DescribeJobExecutionInput`](crate::input::DescribeJobExecutionInput).
    pub fn builder() -> crate::input::describe_job_execution_input::Builder {
        crate::input::describe_job_execution_input::Builder::default()
    }
    /// Creates a new `DescribeJobExecution` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeJobExecution {
    type Output = std::result::Result<
        crate::output::DescribeJobExecutionOutput,
        crate::error::DescribeJobExecutionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_job_execution_error(response)
        } else {
            crate::operation_deser::parse_describe_job_execution_response(response)
        }
    }
}

/// Operation shape for `GetPendingJobExecutions`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_pending_job_executions`](crate::client::Client::get_pending_job_executions).
///
/// See [`crate::client::fluent_builders::GetPendingJobExecutions`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetPendingJobExecutions {
    _private: (),
}
impl GetPendingJobExecutions {
    /// Creates a new builder-style object to manufacture [`GetPendingJobExecutionsInput`](crate::input::GetPendingJobExecutionsInput).
    pub fn builder() -> crate::input::get_pending_job_executions_input::Builder {
        crate::input::get_pending_job_executions_input::Builder::default()
    }
    /// Creates a new `GetPendingJobExecutions` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPendingJobExecutions {
    type Output = std::result::Result<
        crate::output::GetPendingJobExecutionsOutput,
        crate::error::GetPendingJobExecutionsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_pending_job_executions_error(response)
        } else {
            crate::operation_deser::parse_get_pending_job_executions_response(response)
        }
    }
}

/// Operation shape for `StartNextPendingJobExecution`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`start_next_pending_job_execution`](crate::client::Client::start_next_pending_job_execution).
///
/// See [`crate::client::fluent_builders::StartNextPendingJobExecution`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct StartNextPendingJobExecution {
    _private: (),
}
impl StartNextPendingJobExecution {
    /// Creates a new builder-style object to manufacture [`StartNextPendingJobExecutionInput`](crate::input::StartNextPendingJobExecutionInput).
    pub fn builder() -> crate::input::start_next_pending_job_execution_input::Builder {
        crate::input::start_next_pending_job_execution_input::Builder::default()
    }
    /// Creates a new `StartNextPendingJobExecution` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for StartNextPendingJobExecution {
    type Output = std::result::Result<
        crate::output::StartNextPendingJobExecutionOutput,
        crate::error::StartNextPendingJobExecutionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_start_next_pending_job_execution_error(response)
        } else {
            crate::operation_deser::parse_start_next_pending_job_execution_response(response)
        }
    }
}

/// Operation shape for `UpdateJobExecution`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_job_execution`](crate::client::Client::update_job_execution).
///
/// See [`crate::client::fluent_builders::UpdateJobExecution`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateJobExecution {
    _private: (),
}
impl UpdateJobExecution {
    /// Creates a new builder-style object to manufacture [`UpdateJobExecutionInput`](crate::input::UpdateJobExecutionInput).
    pub fn builder() -> crate::input::update_job_execution_input::Builder {
        crate::input::update_job_execution_input::Builder::default()
    }
    /// Creates a new `UpdateJobExecution` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateJobExecution {
    type Output = std::result::Result<
        crate::output::UpdateJobExecutionOutput,
        crate::error::UpdateJobExecutionError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_job_execution_error(response)
        } else {
            crate::operation_deser::parse_update_job_execution_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
