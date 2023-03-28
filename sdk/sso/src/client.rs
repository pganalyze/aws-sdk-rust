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

/// Client for AWS Single Sign-On
///
/// Client for invoking operations on AWS Single Sign-On. Each operation on AWS Single Sign-On is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
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
    /// Constructs a fluent builder for the [`GetRoleCredentials`](crate::client::fluent_builders::GetRoleCredentials) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`role_name(impl Into<String>)`](crate::client::fluent_builders::GetRoleCredentials::role_name) / [`set_role_name(Option<String>)`](crate::client::fluent_builders::GetRoleCredentials::set_role_name): <p>The friendly name of the role that is assigned to the user.</p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::GetRoleCredentials::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::GetRoleCredentials::set_account_id): <p>The identifier for the AWS account that is assigned to the user.</p>
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::GetRoleCredentials::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::GetRoleCredentials::set_access_token): <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    /// - On success, responds with [`GetRoleCredentialsOutput`](crate::output::GetRoleCredentialsOutput) with field(s):
    ///   - [`role_credentials(Option<RoleCredentials>)`](crate::output::GetRoleCredentialsOutput::role_credentials): <p>The credentials for the role that is assigned to the user.</p>
    /// - On failure, responds with [`SdkError<GetRoleCredentialsError>`](crate::error::GetRoleCredentialsError)
    pub fn get_role_credentials(&self) -> fluent_builders::GetRoleCredentials {
        fluent_builders::GetRoleCredentials::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListAccountRoles`](crate::client::fluent_builders::ListAccountRoles) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAccountRoles::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAccountRoles::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAccountRoles::set_next_token): <p>The page token from the previous response output when you request subsequent pages.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAccountRoles::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAccountRoles::set_max_results): <p>The number of items that clients can request per page.</p>
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::ListAccountRoles::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::ListAccountRoles::set_access_token): <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    ///   - [`account_id(impl Into<String>)`](crate::client::fluent_builders::ListAccountRoles::account_id) / [`set_account_id(Option<String>)`](crate::client::fluent_builders::ListAccountRoles::set_account_id): <p>The identifier for the AWS account that is assigned to the user.</p>
    /// - On success, responds with [`ListAccountRolesOutput`](crate::output::ListAccountRolesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListAccountRolesOutput::next_token): <p>The page token client that is used to retrieve the list of accounts.</p>
    ///   - [`role_list(Option<Vec<RoleInfo>>)`](crate::output::ListAccountRolesOutput::role_list): <p>A paginated response with the list of roles and the next token if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListAccountRolesError>`](crate::error::ListAccountRolesError)
    pub fn list_account_roles(&self) -> fluent_builders::ListAccountRoles {
        fluent_builders::ListAccountRoles::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`ListAccounts`](crate::client::fluent_builders::ListAccounts) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::client::fluent_builders::ListAccounts::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::client::fluent_builders::ListAccounts::next_token) / [`set_next_token(Option<String>)`](crate::client::fluent_builders::ListAccounts::set_next_token): <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
    ///   - [`max_results(i32)`](crate::client::fluent_builders::ListAccounts::max_results) / [`set_max_results(Option<i32>)`](crate::client::fluent_builders::ListAccounts::set_max_results): <p>This is the number of items clients can request per page.</p>
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::ListAccounts::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::ListAccounts::set_access_token): <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    /// - On success, responds with [`ListAccountsOutput`](crate::output::ListAccountsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::output::ListAccountsOutput::next_token): <p>The page token client that is used to retrieve the list of accounts.</p>
    ///   - [`account_list(Option<Vec<AccountInfo>>)`](crate::output::ListAccountsOutput::account_list): <p>A paginated response with the list of account information and the next token if more results are available.</p>
    /// - On failure, responds with [`SdkError<ListAccountsError>`](crate::error::ListAccountsError)
    pub fn list_accounts(&self) -> fluent_builders::ListAccounts {
        fluent_builders::ListAccounts::new(self.handle.clone())
    }
    /// Constructs a fluent builder for the [`Logout`](crate::client::fluent_builders::Logout) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`access_token(impl Into<String>)`](crate::client::fluent_builders::Logout::access_token) / [`set_access_token(Option<String>)`](crate::client::fluent_builders::Logout::set_access_token): <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
    /// - On success, responds with [`LogoutOutput`](crate::output::LogoutOutput)

    /// - On failure, responds with [`SdkError<LogoutError>`](crate::error::LogoutError)
    pub fn logout(&self) -> fluent_builders::Logout {
        fluent_builders::Logout::new(self.handle.clone())
    }
}
pub mod fluent_builders {

    //! Utilities to ergonomically construct a request to the service.
    //!
    //! Fluent builders are created through the [`Client`](crate::client::Client) by calling
    //! one if its operation methods. After parameters are set using the builder methods,
    //! the `send` method can be called to initiate the request.
    /// Fluent builder constructing a request to `GetRoleCredentials`.
    ///
    /// <p>Returns the STS short-term credentials for a given role name that is assigned to the user.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct GetRoleCredentials {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::get_role_credentials_input::Builder,
    }
    impl GetRoleCredentials {
        /// Creates a new `GetRoleCredentials`.
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
                crate::operation::GetRoleCredentials,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::GetRoleCredentialsError>,
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
            crate::output::GetRoleCredentialsOutput,
            aws_smithy_http::result::SdkError<crate::error::GetRoleCredentialsError>,
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
        /// <p>The friendly name of the role that is assigned to the user.</p>
        pub fn role_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.role_name(input.into());
            self
        }
        /// <p>The friendly name of the role that is assigned to the user.</p>
        pub fn set_role_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_role_name(input);
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.account_id(input.into());
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_account_id(input);
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.access_token(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_access_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListAccountRoles`.
    ///
    /// <p>Lists all roles that are assigned to the user for a given AWS account.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListAccountRoles {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_account_roles_input::Builder,
    }
    impl ListAccountRoles {
        /// Creates a new `ListAccountRoles`.
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
                crate::operation::ListAccountRoles,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::ListAccountRolesError>,
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
            crate::output::ListAccountRolesOutput,
            aws_smithy_http::result::SdkError<crate::error::ListAccountRolesError>,
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
        /// Paginators are used by calling [`send().await`](crate::paginator::ListAccountRolesPaginator::send) which returns a `Stream`.
        pub fn into_paginator(self) -> crate::paginator::ListAccountRolesPaginator {
            crate::paginator::ListAccountRolesPaginator::new(self.handle, self.inner)
        }
        /// <p>The page token from the previous response output when you request subsequent pages.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>The page token from the previous response output when you request subsequent pages.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>The number of items that clients can request per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>The number of items that clients can request per page.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.access_token(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_access_token(input);
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn account_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.account_id(input.into());
            self
        }
        /// <p>The identifier for the AWS account that is assigned to the user.</p>
        pub fn set_account_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_account_id(input);
            self
        }
    }
    /// Fluent builder constructing a request to `ListAccounts`.
    ///
    /// <p>Lists all AWS accounts assigned to the user. These AWS accounts are assigned by the administrator of the account. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/useraccess.html#assignusers">Assign User Access</a> in the <i>IAM Identity Center User Guide</i>. This operation returns a paginated response.</p>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct ListAccounts {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::list_accounts_input::Builder,
    }
    impl ListAccounts {
        /// Creates a new `ListAccounts`.
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
                crate::operation::ListAccounts,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::ListAccountsError>,
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
            crate::output::ListAccountsOutput,
            aws_smithy_http::result::SdkError<crate::error::ListAccountsError>,
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
        /// Paginators are used by calling [`send().await`](crate::paginator::ListAccountsPaginator::send) which returns a `Stream`.
        pub fn into_paginator(self) -> crate::paginator::ListAccountsPaginator {
            crate::paginator::ListAccountsPaginator::new(self.handle, self.inner)
        }
        /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.next_token(input.into());
            self
        }
        /// <p>(Optional) When requesting subsequent pages, this is the page token from the previous response output.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_next_token(input);
            self
        }
        /// <p>This is the number of items clients can request per page.</p>
        pub fn max_results(mut self, input: i32) -> Self {
            self.inner = self.inner.max_results(input);
            self
        }
        /// <p>This is the number of items clients can request per page.</p>
        pub fn set_max_results(mut self, input: std::option::Option<i32>) -> Self {
            self.inner = self.inner.set_max_results(input);
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.access_token(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_access_token(input);
            self
        }
    }
    /// Fluent builder constructing a request to `Logout`.
    ///
    /// <p>Removes the locally stored SSO tokens from the client-side cache and sends an API call to the IAM Identity Center service to invalidate the corresponding server-side IAM Identity Center sign in session.</p> <note>
    /// <p>If a user uses IAM Identity Center to access the AWS CLI, the user’s IAM Identity Center sign in session is used to obtain an IAM session, as specified in the corresponding IAM Identity Center permission set. More specifically, IAM Identity Center assumes an IAM role in the target account on behalf of the user, and the corresponding temporary AWS credentials are returned to the client.</p>
    /// <p>After user logout, any existing IAM role sessions that were created by using IAM Identity Center permission sets continue based on the duration configured in the permission set. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/userguide/authconcept.html">User authentications</a> in the <i>IAM Identity Center User Guide</i>.</p>
    /// </note>
    #[derive(std::clone::Clone, std::fmt::Debug)]
    pub struct Logout {
        handle: std::sync::Arc<super::Handle>,
        inner: crate::input::logout_input::Builder,
    }
    impl Logout {
        /// Creates a new `Logout`.
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
                crate::operation::Logout,
                aws_http::retry::AwsResponseRetryClassifier,
            >,
            aws_smithy_http::result::SdkError<crate::error::LogoutError>,
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
            crate::output::LogoutOutput,
            aws_smithy_http::result::SdkError<crate::error::LogoutError>,
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
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn access_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.inner = self.inner.access_token(input.into());
            self
        }
        /// <p>The token issued by the <code>CreateToken</code> API call. For more information, see <a href="https://docs.aws.amazon.com/singlesignon/latest/OIDCAPIReference/API_CreateToken.html">CreateToken</a> in the <i>IAM Identity Center OIDC API Reference Guide</i>.</p>
        pub fn set_access_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.inner = self.inner.set_access_token(input);
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
