// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) client: aws_smithy_client::Client<
        aws_smithy_client::erase::DynConnector,
        aws_smithy_client::erase::DynMiddleware<aws_smithy_client::erase::DynConnector>,
    >,
    pub(crate) conf: crate::Config,
}

/// Client for Amazon S3 on Outposts
///
/// Client for invoking operations on Amazon S3 on Outposts. Each operation on Amazon S3 on Outposts is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
///
/// # Examples
/// **Constructing a client and invoking an operation**
/// ```rust,no_run
/// # async fn docs() {
///     // create a shared configuration. This can be used & shared between multiple service clients.
///     let shared_config = aws_config::load_from_env().await;
///     let client = aws_sdk_s3outposts::Client::new(&shared_config);
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
/// let config = aws_sdk_s3outposts::config::Builder::from(&shared_config)
///   .retry_config(RetryConfig::disabled())
///   .build();
/// let client = aws_sdk_s3outposts::Client::from_conf(config);
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

#[doc(inline)]
pub use aws_smithy_client::Builder;

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
    /// Constructs a fluent builder for the [`CreateEndpoint`](crate::client::fluent_builders::CreateEndpoint) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`outpost_id(impl Into<String>)`](crate::client::fluent_builders::CreateEndpoint::outpost_id) / [`set_outpost_id(Option<String>)`](crate::client::fluent_builders::CreateEndpoint::set_outpost_id): <p>The ID of the Outposts. </p>
    ///   - [`subnet_id(impl Into<String>)`](crate::client::fluent_builders::CreateEndpoint::subnet_id) / [`set_subnet_id(Option<String>)`](crate::client::fluent_builders::CreateEndpoint::set_subnet_id): <p>The ID of the subnet in the selected VPC. The endpoint subnet must belong to the Outpost that has Amazon S3 on Outposts provisioned.</p>
    ///   - [`security_group_id(impl Into<String>)`](crate::client::fluent_builders::CreateEndpoint::security_group_id) / [`set_security_group_id(Option<String>)`](crate::client::fluent_builders::CreateEndpoint::set_security_group_id): <p>The ID of the security group to use with the endpoint.</p>
    ///   - [`access_type(EndpointAccessType)`](crate::client::fluent_builders::CreateEndpoint::access_type) / [`set_access_type(Option<EndpointAccessType>)`](crate::client::fluent_builders::CreateEndpoint::set_access_type): <p>The type of access for the network connectivity for the Amazon S3 on Outposts endpoint. To use the Amazon Web Services VPC, choose <code>Private</code>. To use the endpoint with an on-premises network, choose <code>CustomerOwnedIp</code>. If you choose <code>CustomerOwnedIp</code>, you must also provide the customer-owned IP address pool (CoIP pool).</p> <note>   <p> <code>Private</code> is the default access type value.</p>  </note>
    ///   - [`customer_owned_ipv4_pool(impl Into<String>)`](crate::client::fluent_builders::CreateEndpoint::customer_owned_ipv4_pool) / [`set_customer_owned_ipv4_pool(Option<String>)`](crate::client::fluent_builders::CreateEndpoint::set_customer_owned_ipv4_pool): <p>The ID of the customer-owned IPv4 address pool (CoIP pool) for the endpoint. IP addresses are allocated from this pool for the endpoint.</p>
    /// - On success, responds with [`CreateEndpointOutput`](crate::output::CreateEndpointOutput) with field(s):
    ///   - [`endpoint_arn(Option<String>)`](crate::output::CreateEndpointOutput::endpoint_arn): <p>The Amazon Resource Name (ARN) of the endpoint.</p>
    /// - On failure, responds with [`SdkError<CreateEndpointError>`](crate::error::CreateEndpointError)
    pub fn create_endpoint(&self) -> fluent_builders::CreateEndpoint {
        fluent_builders::CreateEndpoint::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`DeleteEndpoint`](crate::client::fluent_builders::DeleteEndpoint) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`endpoint_id(impl Into<String>)`](crate::client::fluent_builders::DeleteEndpoint::endpoint_id) / [`set_endpoint_id(Option<String>)`](crate::client::fluent_builders::DeleteEndpoint::set_endpoint_id): <p>The ID of the endpoint.</p>
    ///   - [`outpost_id(impl Into<String>)`](crate::client::fluent_builders::DeleteEndpoint::outpost_id) / [`set_outpost_id(Option<String>)`](crate::client::fluent_builders::DeleteEndpoint::set_outpost_id): <p>The ID of the Outposts. </p>
    /// - On success, responds with [`DeleteEndpointOutput`](crate::output::DeleteEndpointOutput)

    /// - On failure, responds with [`SdkError<DeleteEndpointError>`](crate::error::DeleteEndpointError)
    pub fn delete_endpoint(&self) -> fluent_builders::DeleteEndpoint {
        fluent_builders::DeleteEndpoint::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListEndpoints`](crate::client::fluent_builders::ListEndpoints) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListEndpoints::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListEndpoints::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListEndpoints::set_next_token): <p>If a previous response from this operation included a <code>NextToken</code> value, provide that value here to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListEndpoints::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListEndpoints::set_max_results): <p>The maximum number of endpoints that will be returned in the response.</p>
    /// - On success, responds with [`ListEndpointsOutput`](crate::output::ListEndpointsOutput) with field(s):
    ///   - [`endpoints(Option<Vec<Endpoint>>)`](crate::output::ListEndpointsOutput::endpoints): <p>The list of endpoints associated with the specified Outpost.</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListEndpointsOutput::next_token): <p>If the number of endpoints associated with the specified Outpost exceeds <code>MaxResults</code>, you can include this value in subsequent calls to this operation to retrieve more results.</p>
    /// - On failure, responds with [`SdkError<ListEndpointsError>`](crate::error::ListEndpointsError)
    pub fn list_endpoints(&self) -> fluent_builders::ListEndpoints {
        fluent_builders::ListEndpoints::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListSharedEndpoints`](crate::client::fluent_builders::ListSharedEndpoints) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListSharedEndpoints::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListSharedEndpoints::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListSharedEndpoints::set_next_token): <p>If a previous response from this operation included a <code>NextToken</code> value, you can provide that value here to retrieve the next page of results.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListSharedEndpoints::max_results) / [`set_max_results(i32)`](crate::client::fluent_builders::ListSharedEndpoints::set_max_results): <p>The maximum number of endpoints that will be returned in the response.</p>
    ///   - [`outpost_id(impl Into<String>)`](crate::client::fluent_builders::ListSharedEndpoints::outpost_id) / [`set_outpost_id(Option<String>)`](crate::client::fluent_builders::ListSharedEndpoints::set_outpost_id): <p>The ID of the Amazon Web Services Outpost.</p>
    /// - On success, responds with [`ListSharedEndpointsOutput`](crate::output::ListSharedEndpointsOutput) with field(s):
    ///   - [`endpoints(Option<Vec<Endpoint>>)`](crate::output::ListSharedEndpointsOutput::endpoints): <p>The list of endpoints associated with the specified Outpost that have been shared by Amazon Web Services Resource Access Manager (RAM).</p>
    ///   - [`next_token(Option<String>)`](crate::output::ListSharedEndpointsOutput::next_token): <p>If the number of endpoints associated with the specified Outpost exceeds <code>MaxResults</code>, you can include this value in subsequent calls to this operation to retrieve more results.</p>
    /// - On failure, responds with [`SdkError<ListSharedEndpointsError>`](crate::error::ListSharedEndpointsError)
    pub fn list_shared_endpoints(&self) -> fluent_builders::ListSharedEndpoints {
        fluent_builders::ListSharedEndpoints::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `CreateEndpoint`.
    ///
    /// <p>Creates an endpoint and associates it with the specified Outpost.</p> <note>
    /// <p>It can take up to 5 minutes for this action to finish.</p>
    /// </note>
    /// <p></p>
    /// <p>Related actions include:</p>
    /// <ul>
    /// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_DeleteEndpoint.html">DeleteEndpoint</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_ListEndpoints.html">ListEndpoints</a> </p> </li>
    /// </ul>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct CreateEndpoint {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::create_endpoint_input::Builder,
    }
    impl CreateEndpoint {
        /// Creates a new `CreateEndpoint`.
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
                crate::operation::CreateEndpoint,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::CreateEndpointError>,
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
            crate::output::CreateEndpointOutput,
            aws_smithy_http::result::SdkError<crate::error::CreateEndpointError>,
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
        /// <p>The ID of the Outposts. </p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.outpost_id(input.into());
            self
        }
        /// <p>The ID of the Outposts. </p>
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_outpost_id(input);
            self
        }
        /// <p>The ID of the subnet in the selected VPC. The endpoint subnet must belong to the Outpost that has Amazon S3 on Outposts provisioned.</p>
        pub fn subnet_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.subnet_id(input.into());
            self
        }
        /// <p>The ID of the subnet in the selected VPC. The endpoint subnet must belong to the Outpost that has Amazon S3 on Outposts provisioned.</p>
        pub fn set_subnet_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_subnet_id(input);
            self
        }
        /// <p>The ID of the security group to use with the endpoint.</p>
        pub fn security_group_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.security_group_id(input.into());
            self
        }
        /// <p>The ID of the security group to use with the endpoint.</p>
        pub fn set_security_group_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_security_group_id(input);
            self
        }
        /// <p>The type of access for the network connectivity for the Amazon S3 on Outposts endpoint. To use the Amazon Web Services VPC, choose <code>Private</code>. To use the endpoint with an on-premises network, choose <code>CustomerOwnedIp</code>. If you choose <code>CustomerOwnedIp</code>, you must also provide the customer-owned IP address pool (CoIP pool).</p> <note>
        /// <p> <code>Private</code> is the default access type value.</p>
        /// </note>
        pub fn access_type(mut self, input: crate::model::EndpointAccessType) -> Self {
            self.inner = self.inner.access_type(input);
            self
        }
        /// <p>The type of access for the network connectivity for the Amazon S3 on Outposts endpoint. To use the Amazon Web Services VPC, choose <code>Private</code>. To use the endpoint with an on-premises network, choose <code>CustomerOwnedIp</code>. If you choose <code>CustomerOwnedIp</code>, you must also provide the customer-owned IP address pool (CoIP pool).</p> <note>
        /// <p> <code>Private</code> is the default access type value.</p>
        /// </note>
        pub fn set_access_type(
            mut self,
            input: std::option::Option<crate::model::EndpointAccessType>,
        ) -> Self {
            self.inner = self.inner.set_access_type(input);
            self
        }
        /// <p>The ID of the customer-owned IPv4 address pool (CoIP pool) for the endpoint. IP addresses are allocated from this pool for the endpoint.</p>
        pub fn customer_owned_ipv4_pool(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.customer_owned_ipv4_pool(input.into());
            self
        }
        /// <p>The ID of the customer-owned IPv4 address pool (CoIP pool) for the endpoint. IP addresses are allocated from this pool for the endpoint.</p>
        pub fn set_customer_owned_ipv4_pool(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.inner = self.inner.set_customer_owned_ipv4_pool(input);
            self
        }
    }
    /// Fluent builder constructing a request to `DeleteEndpoint`.
    ///
    /// <p>Deletes an endpoint.</p> <note>
    /// <p>It can take up to 5 minutes for this action to finish.</p>
    /// </note>
    /// <p></p>
    /// <p>Related actions include:</p>
    /// <ul>
    /// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_CreateEndpoint.html">CreateEndpoint</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_ListEndpoints.html">ListEndpoints</a> </p> </li>
    /// </ul>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct DeleteEndpoint {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::delete_endpoint_input::Builder,
    }
    impl DeleteEndpoint {
        /// Creates a new `DeleteEndpoint`.
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
                crate::operation::DeleteEndpoint,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::DeleteEndpointError>,
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
            crate::output::DeleteEndpointOutput,
            aws_smithy_http::result::SdkError<crate::error::DeleteEndpointError>,
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
        /// <p>The ID of the endpoint.</p>
        pub fn endpoint_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.endpoint_id(input.into());
            self
        }
        /// <p>The ID of the endpoint.</p>
        pub fn set_endpoint_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_endpoint_id(input);
            self
        }
        /// <p>The ID of the Outposts. </p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.outpost_id(input.into());
            self
        }
        /// <p>The ID of the Outposts. </p>
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_outpost_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListEndpoints`.
    ///
    /// <p>Lists endpoints associated with the specified Outpost. </p>
    /// <p>Related actions include:</p>
    /// <ul>
    /// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_CreateEndpoint.html">CreateEndpoint</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_DeleteEndpoint.html">DeleteEndpoint</a> </p> </li>
    /// </ul>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListEndpoints {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_endpoints_input::Builder,
    }
    impl ListEndpoints {
        /// Creates a new `ListEndpoints`.
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
                crate::operation::ListEndpoints,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::ListEndpointsError>,
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
            crate::output::ListEndpointsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListEndpointsError>,
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
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::ListEndpointsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListEndpointsPaginator {
            crate::paginator::ListEndpointsPaginator::new(self.handle, self.inner)
        }
        /// <p>If a previous response from this operation included a <code>NextToken</code> value, provide that value here to retrieve the next page of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>If a previous response from this operation included a <code>NextToken</code> value, provide that value here to retrieve the next page of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of endpoints that will be returned in the response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of endpoints that will be returned in the response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListSharedEndpoints`.
    ///
    /// <p>Lists all endpoints associated with an Outpost that has been shared by Amazon Web Services Resource Access Manager (RAM).</p>
    /// <p>Related actions include:</p>
    /// <ul>
    /// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_CreateEndpoint.html">CreateEndpoint</a> </p> </li>
    /// <li> <p> <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_s3outposts_DeleteEndpoint.html">DeleteEndpoint</a> </p> </li>
    /// </ul>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListSharedEndpoints {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_shared_endpoints_input::Builder,
    }
    impl ListSharedEndpoints {
        /// Creates a new `ListSharedEndpoints`.
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
                crate::operation::ListSharedEndpoints,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::ListSharedEndpointsError>,
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
            crate::output::ListSharedEndpointsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListSharedEndpointsError>,
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
        /// Create a paginator for this request
        ///
        /// Paginators are used by calling [`send().await`](crate::paginator::ListSharedEndpointsPaginator::send) which returns a [`Stream`](tokio_stream::Stream).
        pub fn into_paginator(self) -> crate::paginator::ListSharedEndpointsPaginator {
            crate::paginator::ListSharedEndpointsPaginator::new(self.handle, self.inner)
        }
        /// <p>If a previous response from this operation included a <code>NextToken</code> value, you can provide that value here to retrieve the next page of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>If a previous response from this operation included a <code>NextToken</code> value, you can provide that value here to retrieve the next page of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The maximum number of endpoints that will be returned in the response.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The maximum number of endpoints that will be returned in the response.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The ID of the Amazon Web Services Outpost.</p>
        pub fn outpost_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.outpost_id(input.into());
            self
        }
        /// <p>The ID of the Amazon Web Services Outpost.</p>
        pub fn set_outpost_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_outpost_id(input);
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
