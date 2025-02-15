// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_bot_locale::_update_bot_locale_output::UpdateBotLocaleOutputBuilder;

pub use crate::operation::update_bot_locale::_update_bot_locale_input::UpdateBotLocaleInputBuilder;

impl UpdateBotLocaleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_bot_locale::UpdateBotLocaleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_bot_locale::UpdateBotLocaleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_bot_locale();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateBotLocale`.
///
/// <p>Updates the settings that a bot has for a specific locale.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateBotLocaleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_bot_locale::builders::UpdateBotLocaleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_bot_locale::UpdateBotLocaleOutput,
        crate::operation::update_bot_locale::UpdateBotLocaleError,
    > for UpdateBotLocaleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_bot_locale::UpdateBotLocaleOutput,
            crate::operation::update_bot_locale::UpdateBotLocaleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateBotLocaleFluentBuilder {
    /// Creates a new `UpdateBotLocale`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateBotLocale as a reference.
    pub fn as_input(&self) -> &crate::operation::update_bot_locale::builders::UpdateBotLocaleInputBuilder {
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
        crate::operation::update_bot_locale::UpdateBotLocaleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_bot_locale::UpdateBotLocaleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_bot_locale::UpdateBotLocale::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_bot_locale::UpdateBotLocale::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_bot_locale::UpdateBotLocaleOutput,
        crate::operation::update_bot_locale::UpdateBotLocaleError,
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
    /// <p>The unique identifier of the bot that contains the locale.</p>
    pub fn bot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_id(input.into());
        self
    }
    /// <p>The unique identifier of the bot that contains the locale.</p>
    pub fn set_bot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_id(input);
        self
    }
    /// <p>The unique identifier of the bot that contains the locale.</p>
    pub fn get_bot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_id()
    }
    /// <p>The version of the bot that contains the locale to be updated. The version can only be the <code>DRAFT</code> version.</p>
    pub fn bot_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bot_version(input.into());
        self
    }
    /// <p>The version of the bot that contains the locale to be updated. The version can only be the <code>DRAFT</code> version.</p>
    pub fn set_bot_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bot_version(input);
        self
    }
    /// <p>The version of the bot that contains the locale to be updated. The version can only be the <code>DRAFT</code> version.</p>
    pub fn get_bot_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bot_version()
    }
    /// <p>The identifier of the language and locale to update. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn locale_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.locale_id(input.into());
        self
    }
    /// <p>The identifier of the language and locale to update. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn set_locale_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_locale_id(input);
        self
    }
    /// <p>The identifier of the language and locale to update. The string must match one of the supported locales. For more information, see <a href="https://docs.aws.amazon.com/lexv2/latest/dg/how-languages.html">Supported languages</a>.</p>
    pub fn get_locale_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_locale_id()
    }
    /// <p>The new description of the locale.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The new description of the locale.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The new description of the locale.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The new confidence threshold where Amazon Lex inserts the <code>AMAZON.FallbackIntent</code> and <code>AMAZON.KendraSearchIntent</code> intents in the list of possible intents for an utterance.</p>
    pub fn nlu_intent_confidence_threshold(mut self, input: f64) -> Self {
        self.inner = self.inner.nlu_intent_confidence_threshold(input);
        self
    }
    /// <p>The new confidence threshold where Amazon Lex inserts the <code>AMAZON.FallbackIntent</code> and <code>AMAZON.KendraSearchIntent</code> intents in the list of possible intents for an utterance.</p>
    pub fn set_nlu_intent_confidence_threshold(mut self, input: ::std::option::Option<f64>) -> Self {
        self.inner = self.inner.set_nlu_intent_confidence_threshold(input);
        self
    }
    /// <p>The new confidence threshold where Amazon Lex inserts the <code>AMAZON.FallbackIntent</code> and <code>AMAZON.KendraSearchIntent</code> intents in the list of possible intents for an utterance.</p>
    pub fn get_nlu_intent_confidence_threshold(&self) -> &::std::option::Option<f64> {
        self.inner.get_nlu_intent_confidence_threshold()
    }
    /// <p>The new Amazon Polly voice Amazon Lex should use for voice interaction with the user.</p>
    pub fn voice_settings(mut self, input: crate::types::VoiceSettings) -> Self {
        self.inner = self.inner.voice_settings(input);
        self
    }
    /// <p>The new Amazon Polly voice Amazon Lex should use for voice interaction with the user.</p>
    pub fn set_voice_settings(mut self, input: ::std::option::Option<crate::types::VoiceSettings>) -> Self {
        self.inner = self.inner.set_voice_settings(input);
        self
    }
    /// <p>The new Amazon Polly voice Amazon Lex should use for voice interaction with the user.</p>
    pub fn get_voice_settings(&self) -> &::std::option::Option<crate::types::VoiceSettings> {
        self.inner.get_voice_settings()
    }
}
