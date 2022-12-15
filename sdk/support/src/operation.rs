// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `AddAttachmentsToSet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`add_attachments_to_set`](crate::client::Client::add_attachments_to_set).
///
/// See [`crate::client::fluent_builders::AddAttachmentsToSet`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AddAttachmentsToSet {
    _private: (),
}
impl AddAttachmentsToSet {
    /// Creates a new builder-style object to manufacture [`AddAttachmentsToSetInput`](crate::input::AddAttachmentsToSetInput).
    pub fn builder() -> crate::input::add_attachments_to_set_input::Builder {
        crate::input::add_attachments_to_set_input::Builder::default()
    }
    /// Creates a new `AddAttachmentsToSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AddAttachmentsToSet {
    type Output = std::result::Result<
        crate::output::AddAttachmentsToSetOutput,
        crate::error::AddAttachmentsToSetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_attachments_to_set_error(response)
        } else {
            crate::operation_deser::parse_add_attachments_to_set_response(response)
        }
    }
}

/// Operation shape for `AddCommunicationToCase`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`add_communication_to_case`](crate::client::Client::add_communication_to_case).
///
/// See [`crate::client::fluent_builders::AddCommunicationToCase`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct AddCommunicationToCase {
    _private: (),
}
impl AddCommunicationToCase {
    /// Creates a new builder-style object to manufacture [`AddCommunicationToCaseInput`](crate::input::AddCommunicationToCaseInput).
    pub fn builder() -> crate::input::add_communication_to_case_input::Builder {
        crate::input::add_communication_to_case_input::Builder::default()
    }
    /// Creates a new `AddCommunicationToCase` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for AddCommunicationToCase {
    type Output = std::result::Result<
        crate::output::AddCommunicationToCaseOutput,
        crate::error::AddCommunicationToCaseError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_add_communication_to_case_error(response)
        } else {
            crate::operation_deser::parse_add_communication_to_case_response(response)
        }
    }
}

/// Operation shape for `CreateCase`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_case`](crate::client::Client::create_case).
///
/// See [`crate::client::fluent_builders::CreateCase`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateCase {
    _private: (),
}
impl CreateCase {
    /// Creates a new builder-style object to manufacture [`CreateCaseInput`](crate::input::CreateCaseInput).
    pub fn builder() -> crate::input::create_case_input::Builder {
        crate::input::create_case_input::Builder::default()
    }
    /// Creates a new `CreateCase` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateCase {
    type Output =
        std::result::Result<crate::output::CreateCaseOutput, crate::error::CreateCaseError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_case_error(response)
        } else {
            crate::operation_deser::parse_create_case_response(response)
        }
    }
}

/// Operation shape for `DescribeAttachment`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_attachment`](crate::client::Client::describe_attachment).
///
/// See [`crate::client::fluent_builders::DescribeAttachment`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeAttachment {
    _private: (),
}
impl DescribeAttachment {
    /// Creates a new builder-style object to manufacture [`DescribeAttachmentInput`](crate::input::DescribeAttachmentInput).
    pub fn builder() -> crate::input::describe_attachment_input::Builder {
        crate::input::describe_attachment_input::Builder::default()
    }
    /// Creates a new `DescribeAttachment` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeAttachment {
    type Output = std::result::Result<
        crate::output::DescribeAttachmentOutput,
        crate::error::DescribeAttachmentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_attachment_error(response)
        } else {
            crate::operation_deser::parse_describe_attachment_response(response)
        }
    }
}

/// Operation shape for `DescribeCases`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_cases`](crate::client::Client::describe_cases).
///
/// See [`crate::client::fluent_builders::DescribeCases`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeCases {
    _private: (),
}
impl DescribeCases {
    /// Creates a new builder-style object to manufacture [`DescribeCasesInput`](crate::input::DescribeCasesInput).
    pub fn builder() -> crate::input::describe_cases_input::Builder {
        crate::input::describe_cases_input::Builder::default()
    }
    /// Creates a new `DescribeCases` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCases {
    type Output =
        std::result::Result<crate::output::DescribeCasesOutput, crate::error::DescribeCasesError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_cases_error(response)
        } else {
            crate::operation_deser::parse_describe_cases_response(response)
        }
    }
}

/// Operation shape for `DescribeCommunications`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_communications`](crate::client::Client::describe_communications).
///
/// See [`crate::client::fluent_builders::DescribeCommunications`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeCommunications {
    _private: (),
}
impl DescribeCommunications {
    /// Creates a new builder-style object to manufacture [`DescribeCommunicationsInput`](crate::input::DescribeCommunicationsInput).
    pub fn builder() -> crate::input::describe_communications_input::Builder {
        crate::input::describe_communications_input::Builder::default()
    }
    /// Creates a new `DescribeCommunications` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeCommunications {
    type Output = std::result::Result<
        crate::output::DescribeCommunicationsOutput,
        crate::error::DescribeCommunicationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_communications_error(response)
        } else {
            crate::operation_deser::parse_describe_communications_response(response)
        }
    }
}

/// Operation shape for `DescribeServices`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_services`](crate::client::Client::describe_services).
///
/// See [`crate::client::fluent_builders::DescribeServices`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeServices {
    _private: (),
}
impl DescribeServices {
    /// Creates a new builder-style object to manufacture [`DescribeServicesInput`](crate::input::DescribeServicesInput).
    pub fn builder() -> crate::input::describe_services_input::Builder {
        crate::input::describe_services_input::Builder::default()
    }
    /// Creates a new `DescribeServices` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeServices {
    type Output = std::result::Result<
        crate::output::DescribeServicesOutput,
        crate::error::DescribeServicesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_services_error(response)
        } else {
            crate::operation_deser::parse_describe_services_response(response)
        }
    }
}

/// Operation shape for `DescribeSeverityLevels`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_severity_levels`](crate::client::Client::describe_severity_levels).
///
/// See [`crate::client::fluent_builders::DescribeSeverityLevels`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeSeverityLevels {
    _private: (),
}
impl DescribeSeverityLevels {
    /// Creates a new builder-style object to manufacture [`DescribeSeverityLevelsInput`](crate::input::DescribeSeverityLevelsInput).
    pub fn builder() -> crate::input::describe_severity_levels_input::Builder {
        crate::input::describe_severity_levels_input::Builder::default()
    }
    /// Creates a new `DescribeSeverityLevels` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeSeverityLevels {
    type Output = std::result::Result<
        crate::output::DescribeSeverityLevelsOutput,
        crate::error::DescribeSeverityLevelsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_severity_levels_error(response)
        } else {
            crate::operation_deser::parse_describe_severity_levels_response(response)
        }
    }
}

/// Operation shape for `DescribeTrustedAdvisorCheckRefreshStatuses`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_trusted_advisor_check_refresh_statuses`](crate::client::Client::describe_trusted_advisor_check_refresh_statuses).
///
/// See [`crate::client::fluent_builders::DescribeTrustedAdvisorCheckRefreshStatuses`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeTrustedAdvisorCheckRefreshStatuses {
    _private: (),
}
impl DescribeTrustedAdvisorCheckRefreshStatuses {
    /// Creates a new builder-style object to manufacture [`DescribeTrustedAdvisorCheckRefreshStatusesInput`](crate::input::DescribeTrustedAdvisorCheckRefreshStatusesInput).
    pub fn builder() -> crate::input::describe_trusted_advisor_check_refresh_statuses_input::Builder
    {
        crate::input::describe_trusted_advisor_check_refresh_statuses_input::Builder::default()
    }
    /// Creates a new `DescribeTrustedAdvisorCheckRefreshStatuses` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTrustedAdvisorCheckRefreshStatuses {
    type Output = std::result::Result<
        crate::output::DescribeTrustedAdvisorCheckRefreshStatusesOutput,
        crate::error::DescribeTrustedAdvisorCheckRefreshStatusesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_trusted_advisor_check_refresh_statuses_error(
                response,
            )
        } else {
            crate::operation_deser::parse_describe_trusted_advisor_check_refresh_statuses_response(
                response,
            )
        }
    }
}

/// Operation shape for `DescribeTrustedAdvisorCheckResult`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_trusted_advisor_check_result`](crate::client::Client::describe_trusted_advisor_check_result).
///
/// See [`crate::client::fluent_builders::DescribeTrustedAdvisorCheckResult`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeTrustedAdvisorCheckResult {
    _private: (),
}
impl DescribeTrustedAdvisorCheckResult {
    /// Creates a new builder-style object to manufacture [`DescribeTrustedAdvisorCheckResultInput`](crate::input::DescribeTrustedAdvisorCheckResultInput).
    pub fn builder() -> crate::input::describe_trusted_advisor_check_result_input::Builder {
        crate::input::describe_trusted_advisor_check_result_input::Builder::default()
    }
    /// Creates a new `DescribeTrustedAdvisorCheckResult` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTrustedAdvisorCheckResult {
    type Output = std::result::Result<
        crate::output::DescribeTrustedAdvisorCheckResultOutput,
        crate::error::DescribeTrustedAdvisorCheckResultError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_trusted_advisor_check_result_error(response)
        } else {
            crate::operation_deser::parse_describe_trusted_advisor_check_result_response(response)
        }
    }
}

/// Operation shape for `DescribeTrustedAdvisorChecks`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_trusted_advisor_checks`](crate::client::Client::describe_trusted_advisor_checks).
///
/// See [`crate::client::fluent_builders::DescribeTrustedAdvisorChecks`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeTrustedAdvisorChecks {
    _private: (),
}
impl DescribeTrustedAdvisorChecks {
    /// Creates a new builder-style object to manufacture [`DescribeTrustedAdvisorChecksInput`](crate::input::DescribeTrustedAdvisorChecksInput).
    pub fn builder() -> crate::input::describe_trusted_advisor_checks_input::Builder {
        crate::input::describe_trusted_advisor_checks_input::Builder::default()
    }
    /// Creates a new `DescribeTrustedAdvisorChecks` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTrustedAdvisorChecks {
    type Output = std::result::Result<
        crate::output::DescribeTrustedAdvisorChecksOutput,
        crate::error::DescribeTrustedAdvisorChecksError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_trusted_advisor_checks_error(response)
        } else {
            crate::operation_deser::parse_describe_trusted_advisor_checks_response(response)
        }
    }
}

/// Operation shape for `DescribeTrustedAdvisorCheckSummaries`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_trusted_advisor_check_summaries`](crate::client::Client::describe_trusted_advisor_check_summaries).
///
/// See [`crate::client::fluent_builders::DescribeTrustedAdvisorCheckSummaries`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeTrustedAdvisorCheckSummaries {
    _private: (),
}
impl DescribeTrustedAdvisorCheckSummaries {
    /// Creates a new builder-style object to manufacture [`DescribeTrustedAdvisorCheckSummariesInput`](crate::input::DescribeTrustedAdvisorCheckSummariesInput).
    pub fn builder() -> crate::input::describe_trusted_advisor_check_summaries_input::Builder {
        crate::input::describe_trusted_advisor_check_summaries_input::Builder::default()
    }
    /// Creates a new `DescribeTrustedAdvisorCheckSummaries` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeTrustedAdvisorCheckSummaries {
    type Output = std::result::Result<
        crate::output::DescribeTrustedAdvisorCheckSummariesOutput,
        crate::error::DescribeTrustedAdvisorCheckSummariesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_trusted_advisor_check_summaries_error(response)
        } else {
            crate::operation_deser::parse_describe_trusted_advisor_check_summaries_response(
                response,
            )
        }
    }
}

/// Operation shape for `RefreshTrustedAdvisorCheck`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`refresh_trusted_advisor_check`](crate::client::Client::refresh_trusted_advisor_check).
///
/// See [`crate::client::fluent_builders::RefreshTrustedAdvisorCheck`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct RefreshTrustedAdvisorCheck {
    _private: (),
}
impl RefreshTrustedAdvisorCheck {
    /// Creates a new builder-style object to manufacture [`RefreshTrustedAdvisorCheckInput`](crate::input::RefreshTrustedAdvisorCheckInput).
    pub fn builder() -> crate::input::refresh_trusted_advisor_check_input::Builder {
        crate::input::refresh_trusted_advisor_check_input::Builder::default()
    }
    /// Creates a new `RefreshTrustedAdvisorCheck` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for RefreshTrustedAdvisorCheck {
    type Output = std::result::Result<
        crate::output::RefreshTrustedAdvisorCheckOutput,
        crate::error::RefreshTrustedAdvisorCheckError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_refresh_trusted_advisor_check_error(response)
        } else {
            crate::operation_deser::parse_refresh_trusted_advisor_check_response(response)
        }
    }
}

/// Operation shape for `ResolveCase`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`resolve_case`](crate::client::Client::resolve_case).
///
/// See [`crate::client::fluent_builders::ResolveCase`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ResolveCase {
    _private: (),
}
impl ResolveCase {
    /// Creates a new builder-style object to manufacture [`ResolveCaseInput`](crate::input::ResolveCaseInput).
    pub fn builder() -> crate::input::resolve_case_input::Builder {
        crate::input::resolve_case_input::Builder::default()
    }
    /// Creates a new `ResolveCase` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ResolveCase {
    type Output =
        std::result::Result<crate::output::ResolveCaseOutput, crate::error::ResolveCaseError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_resolve_case_error(response)
        } else {
            crate::operation_deser::parse_resolve_case_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
