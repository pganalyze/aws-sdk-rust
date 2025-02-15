// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_access::_describe_access_output::DescribeAccessOutputBuilder;

pub use crate::operation::describe_access::_describe_access_input::DescribeAccessInputBuilder;

impl DescribeAccessInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_access::DescribeAccessOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_access::DescribeAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_access();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeAccess`.
///
/// <p>Describes the access that is assigned to the specific file transfer protocol-enabled server, as identified by its <code>ServerId</code> property and its <code>ExternalId</code>.</p>
/// <p>The response from this call returns the properties of the access that is associated with the <code>ServerId</code> value that was specified.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeAccessFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_access::builders::DescribeAccessInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_access::DescribeAccessOutput,
        crate::operation::describe_access::DescribeAccessError,
    > for DescribeAccessFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_access::DescribeAccessOutput,
            crate::operation::describe_access::DescribeAccessError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeAccessFluentBuilder {
    /// Creates a new `DescribeAccess`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeAccess as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_access::builders::DescribeAccessInputBuilder {
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
        crate::operation::describe_access::DescribeAccessOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_access::DescribeAccessError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_access::DescribeAccess::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_access::DescribeAccess::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_access::DescribeAccessOutput,
        crate::operation::describe_access::DescribeAccessError,
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
    /// <p>A system-assigned unique identifier for a server that has this access assigned.</p>
    pub fn server_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.server_id(input.into());
        self
    }
    /// <p>A system-assigned unique identifier for a server that has this access assigned.</p>
    pub fn set_server_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_server_id(input);
        self
    }
    /// <p>A system-assigned unique identifier for a server that has this access assigned.</p>
    pub fn get_server_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_server_id()
    }
    /// <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p>
    /// <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p>
    /// <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p>
    /// <p>The regular expression used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    pub fn external_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.external_id(input.into());
        self
    }
    /// <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p>
    /// <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p>
    /// <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p>
    /// <p>The regular expression used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    pub fn set_external_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_external_id(input);
        self
    }
    /// <p>A unique identifier that is required to identify specific groups within your directory. The users of the group that you associate have access to your Amazon S3 or Amazon EFS resources over the enabled protocols using Transfer Family. If you know the group name, you can view the SID values by running the following command using Windows PowerShell.</p>
    /// <p> <code>Get-ADGroup -Filter {samAccountName -like "<i>YourGroupName</i>*"} -Properties * | Select SamAccountName,ObjectSid</code> </p>
    /// <p>In that command, replace <i>YourGroupName</i> with the name of your Active Directory group.</p>
    /// <p>The regular expression used to validate this parameter is a string of characters consisting of uppercase and lowercase alphanumeric characters with no spaces. You can also include underscores or any of the following characters: =,.@:/-</p>
    pub fn get_external_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_external_id()
    }
}
