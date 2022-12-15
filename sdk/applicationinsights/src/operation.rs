// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_application`](crate::client::Client::create_application).
///
/// See [`crate::client::fluent_builders::CreateApplication`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateApplication {
    _private: (),
}
impl CreateApplication {
    /// Creates a new builder-style object to manufacture [`CreateApplicationInput`](crate::input::CreateApplicationInput).
    pub fn builder() -> crate::input::create_application_input::Builder {
        crate::input::create_application_input::Builder::default()
    }
    /// Creates a new `CreateApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateApplication {
    type Output = std::result::Result<
        crate::output::CreateApplicationOutput,
        crate::error::CreateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_application_error(response)
        } else {
            crate::operation_deser::parse_create_application_response(response)
        }
    }
}

/// Operation shape for `CreateComponent`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_component`](crate::client::Client::create_component).
///
/// See [`crate::client::fluent_builders::CreateComponent`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateComponent {
    _private: (),
}
impl CreateComponent {
    /// Creates a new builder-style object to manufacture [`CreateComponentInput`](crate::input::CreateComponentInput).
    pub fn builder() -> crate::input::create_component_input::Builder {
        crate::input::create_component_input::Builder::default()
    }
    /// Creates a new `CreateComponent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateComponent {
    type Output = std::result::Result<
        crate::output::CreateComponentOutput,
        crate::error::CreateComponentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_component_error(response)
        } else {
            crate::operation_deser::parse_create_component_response(response)
        }
    }
}

/// Operation shape for `CreateLogPattern`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_log_pattern`](crate::client::Client::create_log_pattern).
///
/// See [`crate::client::fluent_builders::CreateLogPattern`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateLogPattern {
    _private: (),
}
impl CreateLogPattern {
    /// Creates a new builder-style object to manufacture [`CreateLogPatternInput`](crate::input::CreateLogPatternInput).
    pub fn builder() -> crate::input::create_log_pattern_input::Builder {
        crate::input::create_log_pattern_input::Builder::default()
    }
    /// Creates a new `CreateLogPattern` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateLogPattern {
    type Output = std::result::Result<
        crate::output::CreateLogPatternOutput,
        crate::error::CreateLogPatternError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_log_pattern_error(response)
        } else {
            crate::operation_deser::parse_create_log_pattern_response(response)
        }
    }
}

/// Operation shape for `DeleteApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_application`](crate::client::Client::delete_application).
///
/// See [`crate::client::fluent_builders::DeleteApplication`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteApplication {
    _private: (),
}
impl DeleteApplication {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationInput`](crate::input::DeleteApplicationInput).
    pub fn builder() -> crate::input::delete_application_input::Builder {
        crate::input::delete_application_input::Builder::default()
    }
    /// Creates a new `DeleteApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteApplication {
    type Output = std::result::Result<
        crate::output::DeleteApplicationOutput,
        crate::error::DeleteApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_application_error(response)
        } else {
            crate::operation_deser::parse_delete_application_response(response)
        }
    }
}

/// Operation shape for `DeleteComponent`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_component`](crate::client::Client::delete_component).
///
/// See [`crate::client::fluent_builders::DeleteComponent`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteComponent {
    _private: (),
}
impl DeleteComponent {
    /// Creates a new builder-style object to manufacture [`DeleteComponentInput`](crate::input::DeleteComponentInput).
    pub fn builder() -> crate::input::delete_component_input::Builder {
        crate::input::delete_component_input::Builder::default()
    }
    /// Creates a new `DeleteComponent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteComponent {
    type Output = std::result::Result<
        crate::output::DeleteComponentOutput,
        crate::error::DeleteComponentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_component_error(response)
        } else {
            crate::operation_deser::parse_delete_component_response(response)
        }
    }
}

/// Operation shape for `DeleteLogPattern`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_log_pattern`](crate::client::Client::delete_log_pattern).
///
/// See [`crate::client::fluent_builders::DeleteLogPattern`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteLogPattern {
    _private: (),
}
impl DeleteLogPattern {
    /// Creates a new builder-style object to manufacture [`DeleteLogPatternInput`](crate::input::DeleteLogPatternInput).
    pub fn builder() -> crate::input::delete_log_pattern_input::Builder {
        crate::input::delete_log_pattern_input::Builder::default()
    }
    /// Creates a new `DeleteLogPattern` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteLogPattern {
    type Output = std::result::Result<
        crate::output::DeleteLogPatternOutput,
        crate::error::DeleteLogPatternError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_log_pattern_error(response)
        } else {
            crate::operation_deser::parse_delete_log_pattern_response(response)
        }
    }
}

/// Operation shape for `DescribeApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_application`](crate::client::Client::describe_application).
///
/// See [`crate::client::fluent_builders::DescribeApplication`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeApplication {
    _private: (),
}
impl DescribeApplication {
    /// Creates a new builder-style object to manufacture [`DescribeApplicationInput`](crate::input::DescribeApplicationInput).
    pub fn builder() -> crate::input::describe_application_input::Builder {
        crate::input::describe_application_input::Builder::default()
    }
    /// Creates a new `DescribeApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeApplication {
    type Output = std::result::Result<
        crate::output::DescribeApplicationOutput,
        crate::error::DescribeApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_application_error(response)
        } else {
            crate::operation_deser::parse_describe_application_response(response)
        }
    }
}

/// Operation shape for `DescribeComponent`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_component`](crate::client::Client::describe_component).
///
/// See [`crate::client::fluent_builders::DescribeComponent`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeComponent {
    _private: (),
}
impl DescribeComponent {
    /// Creates a new builder-style object to manufacture [`DescribeComponentInput`](crate::input::DescribeComponentInput).
    pub fn builder() -> crate::input::describe_component_input::Builder {
        crate::input::describe_component_input::Builder::default()
    }
    /// Creates a new `DescribeComponent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeComponent {
    type Output = std::result::Result<
        crate::output::DescribeComponentOutput,
        crate::error::DescribeComponentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_component_error(response)
        } else {
            crate::operation_deser::parse_describe_component_response(response)
        }
    }
}

/// Operation shape for `DescribeComponentConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_component_configuration`](crate::client::Client::describe_component_configuration).
///
/// See [`crate::client::fluent_builders::DescribeComponentConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeComponentConfiguration {
    _private: (),
}
impl DescribeComponentConfiguration {
    /// Creates a new builder-style object to manufacture [`DescribeComponentConfigurationInput`](crate::input::DescribeComponentConfigurationInput).
    pub fn builder() -> crate::input::describe_component_configuration_input::Builder {
        crate::input::describe_component_configuration_input::Builder::default()
    }
    /// Creates a new `DescribeComponentConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeComponentConfiguration {
    type Output = std::result::Result<
        crate::output::DescribeComponentConfigurationOutput,
        crate::error::DescribeComponentConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_component_configuration_error(response)
        } else {
            crate::operation_deser::parse_describe_component_configuration_response(response)
        }
    }
}

/// Operation shape for `DescribeComponentConfigurationRecommendation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_component_configuration_recommendation`](crate::client::Client::describe_component_configuration_recommendation).
///
/// See [`crate::client::fluent_builders::DescribeComponentConfigurationRecommendation`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeComponentConfigurationRecommendation {
    _private: (),
}
impl DescribeComponentConfigurationRecommendation {
    /// Creates a new builder-style object to manufacture [`DescribeComponentConfigurationRecommendationInput`](crate::input::DescribeComponentConfigurationRecommendationInput).
    pub fn builder() -> crate::input::describe_component_configuration_recommendation_input::Builder
    {
        crate::input::describe_component_configuration_recommendation_input::Builder::default()
    }
    /// Creates a new `DescribeComponentConfigurationRecommendation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse
    for DescribeComponentConfigurationRecommendation
{
    type Output = std::result::Result<
        crate::output::DescribeComponentConfigurationRecommendationOutput,
        crate::error::DescribeComponentConfigurationRecommendationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_component_configuration_recommendation_error(
                response,
            )
        } else {
            crate::operation_deser::parse_describe_component_configuration_recommendation_response(
                response,
            )
        }
    }
}

/// Operation shape for `DescribeLogPattern`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_log_pattern`](crate::client::Client::describe_log_pattern).
///
/// See [`crate::client::fluent_builders::DescribeLogPattern`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeLogPattern {
    _private: (),
}
impl DescribeLogPattern {
    /// Creates a new builder-style object to manufacture [`DescribeLogPatternInput`](crate::input::DescribeLogPatternInput).
    pub fn builder() -> crate::input::describe_log_pattern_input::Builder {
        crate::input::describe_log_pattern_input::Builder::default()
    }
    /// Creates a new `DescribeLogPattern` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeLogPattern {
    type Output = std::result::Result<
        crate::output::DescribeLogPatternOutput,
        crate::error::DescribeLogPatternError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_log_pattern_error(response)
        } else {
            crate::operation_deser::parse_describe_log_pattern_response(response)
        }
    }
}

/// Operation shape for `DescribeObservation`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_observation`](crate::client::Client::describe_observation).
///
/// See [`crate::client::fluent_builders::DescribeObservation`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeObservation {
    _private: (),
}
impl DescribeObservation {
    /// Creates a new builder-style object to manufacture [`DescribeObservationInput`](crate::input::DescribeObservationInput).
    pub fn builder() -> crate::input::describe_observation_input::Builder {
        crate::input::describe_observation_input::Builder::default()
    }
    /// Creates a new `DescribeObservation` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeObservation {
    type Output = std::result::Result<
        crate::output::DescribeObservationOutput,
        crate::error::DescribeObservationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_observation_error(response)
        } else {
            crate::operation_deser::parse_describe_observation_response(response)
        }
    }
}

/// Operation shape for `DescribeProblem`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_problem`](crate::client::Client::describe_problem).
///
/// See [`crate::client::fluent_builders::DescribeProblem`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeProblem {
    _private: (),
}
impl DescribeProblem {
    /// Creates a new builder-style object to manufacture [`DescribeProblemInput`](crate::input::DescribeProblemInput).
    pub fn builder() -> crate::input::describe_problem_input::Builder {
        crate::input::describe_problem_input::Builder::default()
    }
    /// Creates a new `DescribeProblem` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeProblem {
    type Output = std::result::Result<
        crate::output::DescribeProblemOutput,
        crate::error::DescribeProblemError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_problem_error(response)
        } else {
            crate::operation_deser::parse_describe_problem_response(response)
        }
    }
}

/// Operation shape for `DescribeProblemObservations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`describe_problem_observations`](crate::client::Client::describe_problem_observations).
///
/// See [`crate::client::fluent_builders::DescribeProblemObservations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DescribeProblemObservations {
    _private: (),
}
impl DescribeProblemObservations {
    /// Creates a new builder-style object to manufacture [`DescribeProblemObservationsInput`](crate::input::DescribeProblemObservationsInput).
    pub fn builder() -> crate::input::describe_problem_observations_input::Builder {
        crate::input::describe_problem_observations_input::Builder::default()
    }
    /// Creates a new `DescribeProblemObservations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DescribeProblemObservations {
    type Output = std::result::Result<
        crate::output::DescribeProblemObservationsOutput,
        crate::error::DescribeProblemObservationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_describe_problem_observations_error(response)
        } else {
            crate::operation_deser::parse_describe_problem_observations_response(response)
        }
    }
}

/// Operation shape for `ListApplications`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_applications`](crate::client::Client::list_applications).
///
/// See [`crate::client::fluent_builders::ListApplications`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListApplications {
    _private: (),
}
impl ListApplications {
    /// Creates a new builder-style object to manufacture [`ListApplicationsInput`](crate::input::ListApplicationsInput).
    pub fn builder() -> crate::input::list_applications_input::Builder {
        crate::input::list_applications_input::Builder::default()
    }
    /// Creates a new `ListApplications` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListApplications {
    type Output = std::result::Result<
        crate::output::ListApplicationsOutput,
        crate::error::ListApplicationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_applications_error(response)
        } else {
            crate::operation_deser::parse_list_applications_response(response)
        }
    }
}

/// Operation shape for `ListComponents`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_components`](crate::client::Client::list_components).
///
/// See [`crate::client::fluent_builders::ListComponents`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListComponents {
    _private: (),
}
impl ListComponents {
    /// Creates a new builder-style object to manufacture [`ListComponentsInput`](crate::input::ListComponentsInput).
    pub fn builder() -> crate::input::list_components_input::Builder {
        crate::input::list_components_input::Builder::default()
    }
    /// Creates a new `ListComponents` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListComponents {
    type Output =
        std::result::Result<crate::output::ListComponentsOutput, crate::error::ListComponentsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_components_error(response)
        } else {
            crate::operation_deser::parse_list_components_response(response)
        }
    }
}

/// Operation shape for `ListConfigurationHistory`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_configuration_history`](crate::client::Client::list_configuration_history).
///
/// See [`crate::client::fluent_builders::ListConfigurationHistory`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListConfigurationHistory {
    _private: (),
}
impl ListConfigurationHistory {
    /// Creates a new builder-style object to manufacture [`ListConfigurationHistoryInput`](crate::input::ListConfigurationHistoryInput).
    pub fn builder() -> crate::input::list_configuration_history_input::Builder {
        crate::input::list_configuration_history_input::Builder::default()
    }
    /// Creates a new `ListConfigurationHistory` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListConfigurationHistory {
    type Output = std::result::Result<
        crate::output::ListConfigurationHistoryOutput,
        crate::error::ListConfigurationHistoryError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_configuration_history_error(response)
        } else {
            crate::operation_deser::parse_list_configuration_history_response(response)
        }
    }
}

/// Operation shape for `ListLogPatterns`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_log_patterns`](crate::client::Client::list_log_patterns).
///
/// See [`crate::client::fluent_builders::ListLogPatterns`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListLogPatterns {
    _private: (),
}
impl ListLogPatterns {
    /// Creates a new builder-style object to manufacture [`ListLogPatternsInput`](crate::input::ListLogPatternsInput).
    pub fn builder() -> crate::input::list_log_patterns_input::Builder {
        crate::input::list_log_patterns_input::Builder::default()
    }
    /// Creates a new `ListLogPatterns` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLogPatterns {
    type Output = std::result::Result<
        crate::output::ListLogPatternsOutput,
        crate::error::ListLogPatternsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_log_patterns_error(response)
        } else {
            crate::operation_deser::parse_list_log_patterns_response(response)
        }
    }
}

/// Operation shape for `ListLogPatternSets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_log_pattern_sets`](crate::client::Client::list_log_pattern_sets).
///
/// See [`crate::client::fluent_builders::ListLogPatternSets`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListLogPatternSets {
    _private: (),
}
impl ListLogPatternSets {
    /// Creates a new builder-style object to manufacture [`ListLogPatternSetsInput`](crate::input::ListLogPatternSetsInput).
    pub fn builder() -> crate::input::list_log_pattern_sets_input::Builder {
        crate::input::list_log_pattern_sets_input::Builder::default()
    }
    /// Creates a new `ListLogPatternSets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListLogPatternSets {
    type Output = std::result::Result<
        crate::output::ListLogPatternSetsOutput,
        crate::error::ListLogPatternSetsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_log_pattern_sets_error(response)
        } else {
            crate::operation_deser::parse_list_log_pattern_sets_response(response)
        }
    }
}

/// Operation shape for `ListProblems`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_problems`](crate::client::Client::list_problems).
///
/// See [`crate::client::fluent_builders::ListProblems`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListProblems {
    _private: (),
}
impl ListProblems {
    /// Creates a new builder-style object to manufacture [`ListProblemsInput`](crate::input::ListProblemsInput).
    pub fn builder() -> crate::input::list_problems_input::Builder {
        crate::input::list_problems_input::Builder::default()
    }
    /// Creates a new `ListProblems` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListProblems {
    type Output =
        std::result::Result<crate::output::ListProblemsOutput, crate::error::ListProblemsError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_problems_error(response)
        } else {
            crate::operation_deser::parse_list_problems_response(response)
        }
    }
}

/// Operation shape for `ListTagsForResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_tags_for_resource`](crate::client::Client::list_tags_for_resource).
///
/// See [`crate::client::fluent_builders::ListTagsForResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListTagsForResource {
    _private: (),
}
impl ListTagsForResource {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceInput`](crate::input::ListTagsForResourceInput).
    pub fn builder() -> crate::input::list_tags_for_resource_input::Builder {
        crate::input::list_tags_for_resource_input::Builder::default()
    }
    /// Creates a new `ListTagsForResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListTagsForResource {
    type Output = std::result::Result<
        crate::output::ListTagsForResourceOutput,
        crate::error::ListTagsForResourceError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_tags_for_resource_error(response)
        } else {
            crate::operation_deser::parse_list_tags_for_resource_response(response)
        }
    }
}

/// Operation shape for `TagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`tag_resource`](crate::client::Client::tag_resource).
///
/// See [`crate::client::fluent_builders::TagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct TagResource {
    _private: (),
}
impl TagResource {
    /// Creates a new builder-style object to manufacture [`TagResourceInput`](crate::input::TagResourceInput).
    pub fn builder() -> crate::input::tag_resource_input::Builder {
        crate::input::tag_resource_input::Builder::default()
    }
    /// Creates a new `TagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for TagResource {
    type Output =
        std::result::Result<crate::output::TagResourceOutput, crate::error::TagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_tag_resource_error(response)
        } else {
            crate::operation_deser::parse_tag_resource_response(response)
        }
    }
}

/// Operation shape for `UntagResource`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`untag_resource`](crate::client::Client::untag_resource).
///
/// See [`crate::client::fluent_builders::UntagResource`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UntagResource {
    _private: (),
}
impl UntagResource {
    /// Creates a new builder-style object to manufacture [`UntagResourceInput`](crate::input::UntagResourceInput).
    pub fn builder() -> crate::input::untag_resource_input::Builder {
        crate::input::untag_resource_input::Builder::default()
    }
    /// Creates a new `UntagResource` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UntagResource {
    type Output =
        std::result::Result<crate::output::UntagResourceOutput, crate::error::UntagResourceError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_untag_resource_error(response)
        } else {
            crate::operation_deser::parse_untag_resource_response(response)
        }
    }
}

/// Operation shape for `UpdateApplication`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_application`](crate::client::Client::update_application).
///
/// See [`crate::client::fluent_builders::UpdateApplication`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateApplication {
    _private: (),
}
impl UpdateApplication {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationInput`](crate::input::UpdateApplicationInput).
    pub fn builder() -> crate::input::update_application_input::Builder {
        crate::input::update_application_input::Builder::default()
    }
    /// Creates a new `UpdateApplication` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateApplication {
    type Output = std::result::Result<
        crate::output::UpdateApplicationOutput,
        crate::error::UpdateApplicationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_application_error(response)
        } else {
            crate::operation_deser::parse_update_application_response(response)
        }
    }
}

/// Operation shape for `UpdateComponent`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_component`](crate::client::Client::update_component).
///
/// See [`crate::client::fluent_builders::UpdateComponent`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateComponent {
    _private: (),
}
impl UpdateComponent {
    /// Creates a new builder-style object to manufacture [`UpdateComponentInput`](crate::input::UpdateComponentInput).
    pub fn builder() -> crate::input::update_component_input::Builder {
        crate::input::update_component_input::Builder::default()
    }
    /// Creates a new `UpdateComponent` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateComponent {
    type Output = std::result::Result<
        crate::output::UpdateComponentOutput,
        crate::error::UpdateComponentError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_component_error(response)
        } else {
            crate::operation_deser::parse_update_component_response(response)
        }
    }
}

/// Operation shape for `UpdateComponentConfiguration`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_component_configuration`](crate::client::Client::update_component_configuration).
///
/// See [`crate::client::fluent_builders::UpdateComponentConfiguration`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateComponentConfiguration {
    _private: (),
}
impl UpdateComponentConfiguration {
    /// Creates a new builder-style object to manufacture [`UpdateComponentConfigurationInput`](crate::input::UpdateComponentConfigurationInput).
    pub fn builder() -> crate::input::update_component_configuration_input::Builder {
        crate::input::update_component_configuration_input::Builder::default()
    }
    /// Creates a new `UpdateComponentConfiguration` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateComponentConfiguration {
    type Output = std::result::Result<
        crate::output::UpdateComponentConfigurationOutput,
        crate::error::UpdateComponentConfigurationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_component_configuration_error(response)
        } else {
            crate::operation_deser::parse_update_component_configuration_response(response)
        }
    }
}

/// Operation shape for `UpdateLogPattern`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_log_pattern`](crate::client::Client::update_log_pattern).
///
/// See [`crate::client::fluent_builders::UpdateLogPattern`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateLogPattern {
    _private: (),
}
impl UpdateLogPattern {
    /// Creates a new builder-style object to manufacture [`UpdateLogPatternInput`](crate::input::UpdateLogPatternInput).
    pub fn builder() -> crate::input::update_log_pattern_input::Builder {
        crate::input::update_log_pattern_input::Builder::default()
    }
    /// Creates a new `UpdateLogPattern` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateLogPattern {
    type Output = std::result::Result<
        crate::output::UpdateLogPatternOutput,
        crate::error::UpdateLogPatternError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_log_pattern_error(response)
        } else {
            crate::operation_deser::parse_update_log_pattern_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
