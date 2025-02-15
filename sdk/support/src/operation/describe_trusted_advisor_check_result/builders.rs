// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_trusted_advisor_check_result::_describe_trusted_advisor_check_result_output::DescribeTrustedAdvisorCheckResultOutputBuilder;

pub use crate::operation::describe_trusted_advisor_check_result::_describe_trusted_advisor_check_result_input::DescribeTrustedAdvisorCheckResultInputBuilder;

impl DescribeTrustedAdvisorCheckResultInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_trusted_advisor_check_result();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeTrustedAdvisorCheckResult`.
///
/// <p>Returns the results of the Trusted Advisor check that has the specified check ID. You can get the check IDs by calling the <code>DescribeTrustedAdvisorChecks</code> operation.</p>
/// <p>The response contains a <code>TrustedAdvisorCheckResult</code> object, which contains these three objects:</p>
/// <ul>
/// <li> <p> <code>TrustedAdvisorCategorySpecificSummary</code> </p> </li>
/// <li> <p> <code>TrustedAdvisorResourceDetail</code> </p> </li>
/// <li> <p> <code>TrustedAdvisorResourcesSummary</code> </p> </li>
/// </ul>
/// <p>In addition, the response contains these fields:</p>
/// <ul>
/// <li> <p> <b>status</b> - The alert status of the check can be <code>ok</code> (green), <code>warning</code> (yellow), <code>error</code> (red), or <code>not_available</code>.</p> </li>
/// <li> <p> <b>timestamp</b> - The time of the last refresh of the check.</p> </li>
/// <li> <p> <b>checkId</b> - The unique identifier for the check.</p> </li>
/// </ul> <note>
/// <ul>
/// <li> <p>You must have a Business, Enterprise On-Ramp, or Enterprise Support plan to use the Amazon Web Services Support API. </p> </li>
/// <li> <p>If you call the Amazon Web Services Support API from an account that doesn't have a Business, Enterprise On-Ramp, or Enterprise Support plan, the <code>SubscriptionRequiredException</code> error message appears. For information about changing your support plan, see <a href="http://aws.amazon.com/premiumsupport/">Amazon Web Services Support</a>.</p> </li>
/// </ul>
/// </note>
/// <p>To call the Trusted Advisor operations in the Amazon Web Services Support API, you must use the US East (N. Virginia) endpoint. Currently, the US West (Oregon) and Europe (Ireland) endpoints don't support the Trusted Advisor operations. For more information, see <a href="https://docs.aws.amazon.com/awssupport/latest/user/about-support-api.html#endpoint">About the Amazon Web Services Support API</a> in the <i>Amazon Web Services Support User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeTrustedAdvisorCheckResultFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_trusted_advisor_check_result::builders::DescribeTrustedAdvisorCheckResultInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultOutput,
        crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultError,
    > for DescribeTrustedAdvisorCheckResultFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultOutput,
            crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeTrustedAdvisorCheckResultFluentBuilder {
    /// Creates a new `DescribeTrustedAdvisorCheckResult`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeTrustedAdvisorCheckResult as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_trusted_advisor_check_result::builders::DescribeTrustedAdvisorCheckResultInputBuilder {
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
        crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResult::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResult::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultOutput,
        crate::operation::describe_trusted_advisor_check_result::DescribeTrustedAdvisorCheckResultError,
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
    /// <p>The unique identifier for the Trusted Advisor check.</p>
    pub fn check_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.check_id(input.into());
        self
    }
    /// <p>The unique identifier for the Trusted Advisor check.</p>
    pub fn set_check_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_check_id(input);
        self
    }
    /// <p>The unique identifier for the Trusted Advisor check.</p>
    pub fn get_check_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_check_id()
    }
    /// <p>The ISO 639-1 code for the language that you want your check results to appear in.</p>
    /// <p>The Amazon Web Services Support API currently supports the following languages for Trusted Advisor:</p>
    /// <ul>
    /// <li> <p>Chinese, Simplified - <code>zh</code> </p> </li>
    /// <li> <p>Chinese, Traditional - <code>zh_TW</code> </p> </li>
    /// <li> <p>English - <code>en</code> </p> </li>
    /// <li> <p>French - <code>fr</code> </p> </li>
    /// <li> <p>German - <code>de</code> </p> </li>
    /// <li> <p>Indonesian - <code>id</code> </p> </li>
    /// <li> <p>Italian - <code>it</code> </p> </li>
    /// <li> <p>Japanese - <code>ja</code> </p> </li>
    /// <li> <p>Korean - <code>ko</code> </p> </li>
    /// <li> <p>Portuguese, Brazilian - <code>pt_BR</code> </p> </li>
    /// <li> <p>Spanish - <code>es</code> </p> </li>
    /// </ul>
    pub fn language(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.language(input.into());
        self
    }
    /// <p>The ISO 639-1 code for the language that you want your check results to appear in.</p>
    /// <p>The Amazon Web Services Support API currently supports the following languages for Trusted Advisor:</p>
    /// <ul>
    /// <li> <p>Chinese, Simplified - <code>zh</code> </p> </li>
    /// <li> <p>Chinese, Traditional - <code>zh_TW</code> </p> </li>
    /// <li> <p>English - <code>en</code> </p> </li>
    /// <li> <p>French - <code>fr</code> </p> </li>
    /// <li> <p>German - <code>de</code> </p> </li>
    /// <li> <p>Indonesian - <code>id</code> </p> </li>
    /// <li> <p>Italian - <code>it</code> </p> </li>
    /// <li> <p>Japanese - <code>ja</code> </p> </li>
    /// <li> <p>Korean - <code>ko</code> </p> </li>
    /// <li> <p>Portuguese, Brazilian - <code>pt_BR</code> </p> </li>
    /// <li> <p>Spanish - <code>es</code> </p> </li>
    /// </ul>
    pub fn set_language(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_language(input);
        self
    }
    /// <p>The ISO 639-1 code for the language that you want your check results to appear in.</p>
    /// <p>The Amazon Web Services Support API currently supports the following languages for Trusted Advisor:</p>
    /// <ul>
    /// <li> <p>Chinese, Simplified - <code>zh</code> </p> </li>
    /// <li> <p>Chinese, Traditional - <code>zh_TW</code> </p> </li>
    /// <li> <p>English - <code>en</code> </p> </li>
    /// <li> <p>French - <code>fr</code> </p> </li>
    /// <li> <p>German - <code>de</code> </p> </li>
    /// <li> <p>Indonesian - <code>id</code> </p> </li>
    /// <li> <p>Italian - <code>it</code> </p> </li>
    /// <li> <p>Japanese - <code>ja</code> </p> </li>
    /// <li> <p>Korean - <code>ko</code> </p> </li>
    /// <li> <p>Portuguese, Brazilian - <code>pt_BR</code> </p> </li>
    /// <li> <p>Spanish - <code>es</code> </p> </li>
    /// </ul>
    pub fn get_language(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_language()
    }
}
