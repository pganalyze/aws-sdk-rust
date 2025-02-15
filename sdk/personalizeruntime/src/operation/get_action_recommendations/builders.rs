// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_action_recommendations::_get_action_recommendations_output::GetActionRecommendationsOutputBuilder;

pub use crate::operation::get_action_recommendations::_get_action_recommendations_input::GetActionRecommendationsInputBuilder;

impl GetActionRecommendationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_action_recommendations::GetActionRecommendationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_action_recommendations::GetActionRecommendationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_action_recommendations();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetActionRecommendations`.
///
/// <p>Returns a list of recommended actions in sorted in descending order by prediction score. Use the <code>GetActionRecommendations</code> API if you have a custom campaign that deploys a solution version trained with a PERSONALIZED_ACTIONS recipe. </p>
/// <p>For more information about PERSONALIZED_ACTIONS recipes, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/nexts-best-action-recipes.html">PERSONALIZED_ACTIONS recipes</a>. For more information about getting action recommendations, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/get-action-recommendations.html">Getting action recommendations</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetActionRecommendationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_action_recommendations::builders::GetActionRecommendationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_action_recommendations::GetActionRecommendationsOutput,
        crate::operation::get_action_recommendations::GetActionRecommendationsError,
    > for GetActionRecommendationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_action_recommendations::GetActionRecommendationsOutput,
            crate::operation::get_action_recommendations::GetActionRecommendationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetActionRecommendationsFluentBuilder {
    /// Creates a new `GetActionRecommendations`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetActionRecommendations as a reference.
    pub fn as_input(&self) -> &crate::operation::get_action_recommendations::builders::GetActionRecommendationsInputBuilder {
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
        crate::operation::get_action_recommendations::GetActionRecommendationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_action_recommendations::GetActionRecommendationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_action_recommendations::GetActionRecommendations::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_action_recommendations::GetActionRecommendations::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_action_recommendations::GetActionRecommendationsOutput,
        crate::operation::get_action_recommendations::GetActionRecommendationsError,
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
    /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting action recommendations. This campaign must deploy a solution version trained with a PERSONALIZED_ACTIONS recipe.</p>
    pub fn campaign_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.campaign_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting action recommendations. This campaign must deploy a solution version trained with a PERSONALIZED_ACTIONS recipe.</p>
    pub fn set_campaign_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_campaign_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the campaign to use for getting action recommendations. This campaign must deploy a solution version trained with a PERSONALIZED_ACTIONS recipe.</p>
    pub fn get_campaign_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_campaign_arn()
    }
    /// <p>The user ID of the user to provide action recommendations for.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>The user ID of the user to provide action recommendations for.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// <p>The user ID of the user to provide action recommendations for.</p>
    pub fn get_user_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_id()
    }
    /// <p>The number of results to return. The default is 5. The maximum is 100.</p>
    pub fn num_results(mut self, input: i32) -> Self {
        self.inner = self.inner.num_results(input);
        self
    }
    /// <p>The number of results to return. The default is 5. The maximum is 100.</p>
    pub fn set_num_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_num_results(input);
        self
    }
    /// <p>The number of results to return. The default is 5. The maximum is 100.</p>
    pub fn get_num_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_num_results()
    }
    /// <p>The ARN of the filter to apply to the returned recommendations. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    /// <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
    pub fn filter_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.filter_arn(input.into());
        self
    }
    /// <p>The ARN of the filter to apply to the returned recommendations. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    /// <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
    pub fn set_filter_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_filter_arn(input);
        self
    }
    /// <p>The ARN of the filter to apply to the returned recommendations. For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering Recommendations</a>.</p>
    /// <p>When using this parameter, be sure the filter resource is <code>ACTIVE</code>.</p>
    pub fn get_filter_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_filter_arn()
    }
    /// Adds a key-value pair to `filterValues`.
    ///
    /// To override the contents of this collection use [`set_filter_values`](Self::set_filter_values).
    ///
    /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
    /// <p>For filter expressions that use an <code>INCLUDE</code> element to include actions, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude actions, you can omit the <code>filter-values</code>. In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
    pub fn filter_values(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.filter_values(k.into(), v.into());
        self
    }
    /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
    /// <p>For filter expressions that use an <code>INCLUDE</code> element to include actions, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude actions, you can omit the <code>filter-values</code>. In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
    pub fn set_filter_values(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_filter_values(input);
        self
    }
    /// <p>The values to use when filtering recommendations. For each placeholder parameter in your filter expression, provide the parameter name (in matching case) as a key and the filter value(s) as the corresponding value. Separate multiple values for one parameter with a comma. </p>
    /// <p>For filter expressions that use an <code>INCLUDE</code> element to include actions, you must provide values for all parameters that are defined in the expression. For filters with expressions that use an <code>EXCLUDE</code> element to exclude actions, you can omit the <code>filter-values</code>. In this case, Amazon Personalize doesn't use that portion of the expression to filter recommendations.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/personalize/latest/dg/filter.html">Filtering recommendations and user segments</a>.</p>
    pub fn get_filter_values(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_filter_values()
    }
}
