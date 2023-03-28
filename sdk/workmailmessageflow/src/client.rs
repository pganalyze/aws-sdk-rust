// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[doc(inline)]
pub use aws_smithy_client::Builder;
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for Amazon WorkMail Message Flow
///
/// Client for invoking operations on Amazon WorkMail Message Flow. Each operation on Amazon WorkMail Message Flow is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_workmailmessageflow::Client::new(&shared_config);
///     // invoke an operation
///     /* let rsp = client
///         .<operation_name>().
///         .<param>("some value")
///         .send().await; */
/// # }
/// ```
/// **Constructing a client with custom configuration**
/// ```rust,no_run
/// use aws_config::retry::RetryConfig;
/// # async fn docs() {
/// let shared_config = aws_config::load_from_env().await;
/// let config = aws_sdk_workmailmessageflow::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_workmailmessageflow::Client::from_conf(config);
/// # }
#[derive(std::fmt::Debug)]
pub struct Client {
    handle: std::sync::Arc<Handle>,
}

impl std::clone::Clone for Client {
    fn clone(&self) -> Self {
        Self {
            handle: self.handle.clone(),
        }
    }
}

impl
    From<
        aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    > for Client
{
    fn from(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
    ) -> Self {
        Self::with_config(client, crate::Config::builder().build())
    }
}

impl Client {
    /// Creates a client with the given service configuration.
    pub fn with_config(
        client: aws_smithy_client::Client<
            aws_smithy_client::erase::DynConnector,
            aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
        >,
        conf: crate::Config,
    ) -> Self {
        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }

    /// Returns the client's configuration.
    pub fn conf(&self) -> &crate::Config {
        &self.handle.conf
    }
}
impl Client {
    /// Constructs a fluent builder for the [`GetRawMessageContent`](crate::client::fluent_builders::GetRawMessageContent) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`message_id(impl Into<String>)`](crate::client::fluent_builders::GetRawMessageContent::message_id) / [`set_message_id(Option<String>)`](crate::client::fluent_builders::GetRawMessageContent::set_message_id): <p>The identifier of the email message to retrieve.</p>
    /// - On success, responds with [`GetRawMessageContentOutput`](crate::output::GetRawMessageContentOutput) with field(s):
    ///   - [`message_content(ByteStream)`](crate::output::GetRawMessageContentOutput::message_content): <p>The raw content of the email message, in MIME format.</p>
    /// - On failure, responds with [`SdkError<GetRawMessageContentError>`](crate::error::GetRawMessageContentError)
    pub fn get_raw_message_content(&self) -> fluent_builders::GetRawMessageContent {
        fluent_builders::GetRawMessageContent::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`PutRawMessageContent`](crate::client::fluent_builders::PutRawMessageContent) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`message_id(impl Into<String>)`](crate::client::fluent_builders::PutRawMessageContent::message_id) / [`set_message_id(Option<String>)`](crate::client::fluent_builders::PutRawMessageContent::set_message_id): <p>The identifier of the email message being updated.</p>
    ///   - [`content(RawMessageContent)`](crate::client::fluent_builders::PutRawMessageContent::content) / [`set_content(Option<RawMessageContent>)`](crate::client::fluent_builders::PutRawMessageContent::set_content): <p>Describes the raw message content of the updated email message.</p>
    /// - On success, responds with [`PutRawMessageContentOutput`](crate::output::PutRawMessageContentOutput)

    /// - On failure, responds with [`SdkError<PutRawMessageContentError>`](crate::error::PutRawMessageContentError)
    pub fn put_raw_message_content(&self) -> fluent_builders::PutRawMessageContent {
        fluent_builders::PutRawMessageContent::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `GetRawMessageContent`.
    ///
    /// <p>Retrieves the raw content of an in-transit email message, in MIME format.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetRawMessageContent {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_raw_message_content_input::Builder,
    }
    impl GetRawMessageContent {
        /// Creates a new `GetRawMessageContent`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::operation::customize::CustomizableOperation<
                crate::operation::GetRawMessageContent,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::GetRawMessageContentError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
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
        ) -> std::result::Result<
            crate::output::GetRawMessageContentOutput,
            aws_smithy_http::result::SdkError<crate::error::GetRawMessageContentError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&self.handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            self.handle.client.call(op).await
        }
        /// <p>The identifier of the email message to retrieve.</p>
        pub fn message_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.message_id(input.into());
            self
        }
        /// <p>The identifier of the email message to retrieve.</p>
        pub fn set_message_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_message_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `PutRawMessageContent`.
    ///
    /// <p>Updates the raw content of an in-transit email message, in MIME format.</p>
    /// <p>This example describes how to update in-transit email message. For more information and examples for using this API, see <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/update-with-lambda.html"> Updating message content with AWS Lambda</a>.</p> <note>
    /// <p>Updates to an in-transit message only appear when you call <code>PutRawMessageContent</code> from an AWS Lambda function configured with a synchronous <a href="https://docs.aws.amazon.com/workmail/latest/adminguide/lambda.html#synchronous-rules"> Run Lambda</a> rule. If you call <code>PutRawMessageContent</code> on a delivered or sent message, the message remains unchanged, even though <a href="https://docs.aws.amazon.com/workmail/latest/APIReference/API_messageflow_GetRawMessageContent.html">GetRawMessageContent</a> returns an updated message. </p>
    /// </note>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct PutRawMessageContent {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::put_raw_message_content_input::Builder,
    }
    impl PutRawMessageContent {
        /// Creates a new `PutRawMessageContent`.
        pub(crate) fn new(handle: std::sync::Arc<super::Handle>) -> Self {
            Self {
                handle,
                inner: Default::default(),
            }
        }

        /// Consume this builder, creating a customizable operation that can be modified before being
        /// sent. The operation's inner [http::Request] can be modified as well.
        pub async fn customize(
            self,
        ) -> std::result::Result<
            crate::operation::customize::CustomizableOperation<
                crate::operation::PutRawMessageContent,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::PutRawMessageContentError>,
        > {
            let handle = self.handle.clone();
            let operation = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            Ok(crate::operation::customize::CustomizableOperation { handle, operation })
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
        ) -> std::result::Result<
            crate::output::PutRawMessageContentOutput,
            aws_smithy_http::result::SdkError<crate::error::PutRawMessageContentError>,
        > {
            let op = self
                .inner
                .build()
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?
                .make_operation(&self.handle.conf)
                .await
                .map_err(aws_smithy_http::result::SdkError::construction_failure)?;
            self.handle.client.call(op).await
        }
        /// <p>The identifier of the email message being updated.</p>
        pub fn message_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.message_id(input.into());
            self
        }
        /// <p>The identifier of the email message being updated.</p>
        pub fn set_message_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_message_id(input);
            self
        }
        /// <p>Describes the raw message content of the updated email message.</p>
        pub fn content(mut self, input: crate::model::RawMessageContent) -> Self {
            self.inner = self.inner.content(input);
            self
        }
        /// <p>Describes the raw message content of the updated email message.</p>
        pub fn set_content(
            mut self,
            input: std::option::Option<crate::model::RawMessageContent>,
        ) -> Self {
            self.inner = self.inner.set_content(input);
            self
        }
    }
}

impl Client {
    /// Creates a new client from an [SDK Config](aws_types::sdk_config::SdkConfig).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
    ///     the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
    ///     `http_connector` on the Config passed into this function to fix it.
    pub fn new(sdk_config: &aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }

    /// Creates a new client from the service [`Config`](crate::Config).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `conf` is missing an async sleep implementation. If you experience this panic, set
    ///     the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `conf` is missing an HTTP connector. If you experience this panic, set the
    ///     `http_connector` on the Config passed into this function to fix it.
    pub fn from_conf(conf: crate::Config) -> Self {
        let retry_config = conf
            .retry_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::retry::RetryConfig::disabled);
        let timeout_config = conf
            .timeout_config()
            .cloned()
            .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
        let sleep_impl = conf.sleep_impl();
        if (retry_config.has_retry() || timeout_config.has_timeouts()) && sleep_impl.is_none() {
            panic!("An async sleep implementation is required for retries or timeouts to work. \
                                    Set the `sleep_impl` on the Config passed into this function to fix this panic.");
        }

        let connector = conf.http_connector().and_then(|c| {
            let timeout_config = conf
                .timeout_config()
                .cloned()
                .unwrap_or_else(aws_smithy_types::timeout::TimeoutConfig::disabled);
            let connector_settings =
                aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                    &timeout_config,
                );
            c.connector(&connector_settings, conf.sleep_impl())
        });

        let builder = aws_smithy_client::Builder::new();

        let builder = match connector {
            // Use provided connector
            Some(c) => builder.connector(c),
            None => {
                #[cfg(any(feature = "rustls", feature = "native-tls"))]
                {
                    // Use default connector based on enabled features
                    builder.dyn_https_connector(
                        aws_smithy_client::http_connector::ConnectorSettings::from_timeout_config(
                            &timeout_config,
                        ),
                    )
                }
                #[cfg(not(any(feature = "rustls", feature = "native-tls")))]
                {
                    panic!("No HTTP connector was available. Enable the `rustls` or `native-tls` crate feature or set a connector to fix this.");
                }
            }
        };
        let mut builder = builder
            .middleware(aws_smithy_client::erase::DynMiddleware::new(
                crate::middleware::DefaultMiddleware::new(),
            ))
            .retry_config(retry_config.into())
            .operation_timeout_config(timeout_config.into());
        builder.set_sleep_impl(sleep_impl);
        let client = builder.build();

        Self {
            handle: std::sync::Arc::new(Handle { client, conf }),
        }
    }
}
