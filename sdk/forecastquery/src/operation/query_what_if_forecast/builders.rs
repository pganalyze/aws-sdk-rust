// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::query_what_if_forecast::_query_what_if_forecast_output::QueryWhatIfForecastOutputBuilder;

pub use crate::operation::query_what_if_forecast::_query_what_if_forecast_input::QueryWhatIfForecastInputBuilder;

impl QueryWhatIfForecastInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::query_what_if_forecast::QueryWhatIfForecastOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::query_what_if_forecast::QueryWhatIfForecastError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.query_what_if_forecast();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `QueryWhatIfForecast`.
///
/// <p>Retrieves a what-if forecast.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct QueryWhatIfForecastFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::query_what_if_forecast::builders::QueryWhatIfForecastInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::query_what_if_forecast::QueryWhatIfForecastOutput,
        crate::operation::query_what_if_forecast::QueryWhatIfForecastError,
    > for QueryWhatIfForecastFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::query_what_if_forecast::QueryWhatIfForecastOutput,
            crate::operation::query_what_if_forecast::QueryWhatIfForecastError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl QueryWhatIfForecastFluentBuilder {
    /// Creates a new `QueryWhatIfForecast`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the QueryWhatIfForecast as a reference.
    pub fn as_input(&self) -> &crate::operation::query_what_if_forecast::builders::QueryWhatIfForecastInputBuilder {
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
        crate::operation::query_what_if_forecast::QueryWhatIfForecastOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::query_what_if_forecast::QueryWhatIfForecastError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::query_what_if_forecast::QueryWhatIfForecast::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::query_what_if_forecast::QueryWhatIfForecast::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::query_what_if_forecast::QueryWhatIfForecastOutput,
        crate::operation::query_what_if_forecast::QueryWhatIfForecastError,
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
    /// <p>The Amazon Resource Name (ARN) of the what-if forecast to query.</p>
    pub fn what_if_forecast_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.what_if_forecast_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the what-if forecast to query.</p>
    pub fn set_what_if_forecast_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_what_if_forecast_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the what-if forecast to query.</p>
    pub fn get_what_if_forecast_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_what_if_forecast_arn()
    }
    /// <p>The start date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    pub fn start_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.start_date(input.into());
        self
    }
    /// <p>The start date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    pub fn set_start_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_start_date(input);
        self
    }
    /// <p>The start date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T08:00:00.</p>
    pub fn get_start_date(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_start_date()
    }
    /// <p>The end date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    pub fn end_date(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.end_date(input.into());
        self
    }
    /// <p>The end date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    pub fn set_end_date(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_end_date(input);
        self
    }
    /// <p>The end date for the what-if forecast. Specify the date using this format: yyyy-MM-dd'T'HH:mm:ss (ISO 8601 format). For example, 2015-01-01T20:00:00. </p>
    pub fn get_end_date(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_end_date()
    }
    /// Adds a key-value pair to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filtering criteria to apply when retrieving the forecast. For example, to get the forecast for <code>client_21</code> in the electricity usage dataset, specify the following:</p>
    /// <p> <code>{"item_id" : "client_21"}</code> </p>
    /// <p>To get the full what-if forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateWhatIfForecastExport.html">CreateForecastExportJob</a> operation.</p>
    pub fn filters(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.filters(k.into(), v.into());
        self
    }
    /// <p>The filtering criteria to apply when retrieving the forecast. For example, to get the forecast for <code>client_21</code> in the electricity usage dataset, specify the following:</p>
    /// <p> <code>{"item_id" : "client_21"}</code> </p>
    /// <p>To get the full what-if forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateWhatIfForecastExport.html">CreateForecastExportJob</a> operation.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filtering criteria to apply when retrieving the forecast. For example, to get the forecast for <code>client_21</code> in the electricity usage dataset, specify the following:</p>
    /// <p> <code>{"item_id" : "client_21"}</code> </p>
    /// <p>To get the full what-if forecast, use the <a href="https://docs.aws.amazon.com/en_us/forecast/latest/dg/API_CreateWhatIfForecastExport.html">CreateForecastExportJob</a> operation.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_filters()
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>If the result of the previous request was truncated, the response includes a <code>NextToken</code>. To retrieve the next set of results, use the token in the next request. Tokens expire after 24 hours.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
