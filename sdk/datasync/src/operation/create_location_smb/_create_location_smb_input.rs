// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>CreateLocationSmbRequest</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CreateLocationSmbInput {
    /// <p>Specifies the name of the share exported by your SMB file server where DataSync will read or write data. You can include a subdirectory in the share path (for example, <code>/path/to/subdirectory</code>). Make sure that other SMB clients in your network can also mount this path.</p>
    /// <p>To copy all data in the specified subdirectory, DataSync must be able to mount the SMB share and access all of its data. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub subdirectory: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the Domain Name Service (DNS) name or IP address of the SMB file server that your DataSync agent will mount.</p> <note>
    /// <p>You can't specify an IP version 6 (IPv6) address.</p>
    /// </note>
    pub server_hostname: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the user name that can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For information about choosing a user with the right level of access for your transfer, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub user: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the Windows domain name that your SMB file server belongs to. </p>
    /// <p>If you have multiple domains in your environment, configuring this parameter makes sure that DataSync connects to the right file server.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub domain: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the password of the user who can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub password: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the DataSync agent (or agents) which you want to connect to your SMB file server. You specify an agent by using its Amazon Resource Name (ARN).</p>
    pub agent_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Specifies the version of the SMB protocol that DataSync uses to access your SMB file server.</p>
    pub mount_options: ::std::option::Option<crate::types::SmbMountOptions>,
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your location.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::TagListEntry>>,
}
impl CreateLocationSmbInput {
    /// <p>Specifies the name of the share exported by your SMB file server where DataSync will read or write data. You can include a subdirectory in the share path (for example, <code>/path/to/subdirectory</code>). Make sure that other SMB clients in your network can also mount this path.</p>
    /// <p>To copy all data in the specified subdirectory, DataSync must be able to mount the SMB share and access all of its data. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn subdirectory(&self) -> ::std::option::Option<&str> {
        self.subdirectory.as_deref()
    }
    /// <p>Specifies the Domain Name Service (DNS) name or IP address of the SMB file server that your DataSync agent will mount.</p> <note>
    /// <p>You can't specify an IP version 6 (IPv6) address.</p>
    /// </note>
    pub fn server_hostname(&self) -> ::std::option::Option<&str> {
        self.server_hostname.as_deref()
    }
    /// <p>Specifies the user name that can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For information about choosing a user with the right level of access for your transfer, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn user(&self) -> ::std::option::Option<&str> {
        self.user.as_deref()
    }
    /// <p>Specifies the Windows domain name that your SMB file server belongs to. </p>
    /// <p>If you have multiple domains in your environment, configuring this parameter makes sure that DataSync connects to the right file server.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn domain(&self) -> ::std::option::Option<&str> {
        self.domain.as_deref()
    }
    /// <p>Specifies the password of the user who can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn password(&self) -> ::std::option::Option<&str> {
        self.password.as_deref()
    }
    /// <p>Specifies the DataSync agent (or agents) which you want to connect to your SMB file server. You specify an agent by using its Amazon Resource Name (ARN).</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.agent_arns.is_none()`.
    pub fn agent_arns(&self) -> &[::std::string::String] {
        self.agent_arns.as_deref().unwrap_or_default()
    }
    /// <p>Specifies the version of the SMB protocol that DataSync uses to access your SMB file server.</p>
    pub fn mount_options(&self) -> ::std::option::Option<&crate::types::SmbMountOptions> {
        self.mount_options.as_ref()
    }
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your location.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::TagListEntry] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for CreateLocationSmbInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateLocationSmbInput");
        formatter.field("subdirectory", &self.subdirectory);
        formatter.field("server_hostname", &self.server_hostname);
        formatter.field("user", &self.user);
        formatter.field("domain", &self.domain);
        formatter.field("password", &"*** Sensitive Data Redacted ***");
        formatter.field("agent_arns", &self.agent_arns);
        formatter.field("mount_options", &self.mount_options);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
impl CreateLocationSmbInput {
    /// Creates a new builder-style object to manufacture [`CreateLocationSmbInput`](crate::operation::create_location_smb::CreateLocationSmbInput).
    pub fn builder() -> crate::operation::create_location_smb::builders::CreateLocationSmbInputBuilder {
        crate::operation::create_location_smb::builders::CreateLocationSmbInputBuilder::default()
    }
}

/// A builder for [`CreateLocationSmbInput`](crate::operation::create_location_smb::CreateLocationSmbInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CreateLocationSmbInputBuilder {
    pub(crate) subdirectory: ::std::option::Option<::std::string::String>,
    pub(crate) server_hostname: ::std::option::Option<::std::string::String>,
    pub(crate) user: ::std::option::Option<::std::string::String>,
    pub(crate) domain: ::std::option::Option<::std::string::String>,
    pub(crate) password: ::std::option::Option<::std::string::String>,
    pub(crate) agent_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) mount_options: ::std::option::Option<crate::types::SmbMountOptions>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::TagListEntry>>,
}
impl CreateLocationSmbInputBuilder {
    /// <p>Specifies the name of the share exported by your SMB file server where DataSync will read or write data. You can include a subdirectory in the share path (for example, <code>/path/to/subdirectory</code>). Make sure that other SMB clients in your network can also mount this path.</p>
    /// <p>To copy all data in the specified subdirectory, DataSync must be able to mount the SMB share and access all of its data. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    /// This field is required.
    pub fn subdirectory(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subdirectory = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the name of the share exported by your SMB file server where DataSync will read or write data. You can include a subdirectory in the share path (for example, <code>/path/to/subdirectory</code>). Make sure that other SMB clients in your network can also mount this path.</p>
    /// <p>To copy all data in the specified subdirectory, DataSync must be able to mount the SMB share and access all of its data. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn set_subdirectory(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subdirectory = input;
        self
    }
    /// <p>Specifies the name of the share exported by your SMB file server where DataSync will read or write data. You can include a subdirectory in the share path (for example, <code>/path/to/subdirectory</code>). Make sure that other SMB clients in your network can also mount this path.</p>
    /// <p>To copy all data in the specified subdirectory, DataSync must be able to mount the SMB share and access all of its data. For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn get_subdirectory(&self) -> &::std::option::Option<::std::string::String> {
        &self.subdirectory
    }
    /// <p>Specifies the Domain Name Service (DNS) name or IP address of the SMB file server that your DataSync agent will mount.</p> <note>
    /// <p>You can't specify an IP version 6 (IPv6) address.</p>
    /// </note>
    /// This field is required.
    pub fn server_hostname(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.server_hostname = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the Domain Name Service (DNS) name or IP address of the SMB file server that your DataSync agent will mount.</p> <note>
    /// <p>You can't specify an IP version 6 (IPv6) address.</p>
    /// </note>
    pub fn set_server_hostname(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.server_hostname = input;
        self
    }
    /// <p>Specifies the Domain Name Service (DNS) name or IP address of the SMB file server that your DataSync agent will mount.</p> <note>
    /// <p>You can't specify an IP version 6 (IPv6) address.</p>
    /// </note>
    pub fn get_server_hostname(&self) -> &::std::option::Option<::std::string::String> {
        &self.server_hostname
    }
    /// <p>Specifies the user name that can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For information about choosing a user with the right level of access for your transfer, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    /// This field is required.
    pub fn user(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the user name that can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For information about choosing a user with the right level of access for your transfer, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn set_user(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user = input;
        self
    }
    /// <p>Specifies the user name that can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For information about choosing a user with the right level of access for your transfer, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn get_user(&self) -> &::std::option::Option<::std::string::String> {
        &self.user
    }
    /// <p>Specifies the Windows domain name that your SMB file server belongs to. </p>
    /// <p>If you have multiple domains in your environment, configuring this parameter makes sure that DataSync connects to the right file server.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn domain(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the Windows domain name that your SMB file server belongs to. </p>
    /// <p>If you have multiple domains in your environment, configuring this parameter makes sure that DataSync connects to the right file server.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn set_domain(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain = input;
        self
    }
    /// <p>Specifies the Windows domain name that your SMB file server belongs to. </p>
    /// <p>If you have multiple domains in your environment, configuring this parameter makes sure that DataSync connects to the right file server.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn get_domain(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain
    }
    /// <p>Specifies the password of the user who can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    /// This field is required.
    pub fn password(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.password = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the password of the user who can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn set_password(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.password = input;
        self
    }
    /// <p>Specifies the password of the user who can mount your SMB file server and has permission to access the files and folders involved in your transfer.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/datasync/latest/userguide/create-smb-location.html#configuring-smb-permissions">required permissions</a> for SMB locations.</p>
    pub fn get_password(&self) -> &::std::option::Option<::std::string::String> {
        &self.password
    }
    /// Appends an item to `agent_arns`.
    ///
    /// To override the contents of this collection use [`set_agent_arns`](Self::set_agent_arns).
    ///
    /// <p>Specifies the DataSync agent (or agents) which you want to connect to your SMB file server. You specify an agent by using its Amazon Resource Name (ARN).</p>
    pub fn agent_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.agent_arns.unwrap_or_default();
        v.push(input.into());
        self.agent_arns = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies the DataSync agent (or agents) which you want to connect to your SMB file server. You specify an agent by using its Amazon Resource Name (ARN).</p>
    pub fn set_agent_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.agent_arns = input;
        self
    }
    /// <p>Specifies the DataSync agent (or agents) which you want to connect to your SMB file server. You specify an agent by using its Amazon Resource Name (ARN).</p>
    pub fn get_agent_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.agent_arns
    }
    /// <p>Specifies the version of the SMB protocol that DataSync uses to access your SMB file server.</p>
    pub fn mount_options(mut self, input: crate::types::SmbMountOptions) -> Self {
        self.mount_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the version of the SMB protocol that DataSync uses to access your SMB file server.</p>
    pub fn set_mount_options(mut self, input: ::std::option::Option<crate::types::SmbMountOptions>) -> Self {
        self.mount_options = input;
        self
    }
    /// <p>Specifies the version of the SMB protocol that DataSync uses to access your SMB file server.</p>
    pub fn get_mount_options(&self) -> &::std::option::Option<crate::types::SmbMountOptions> {
        &self.mount_options
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your location.</p>
    pub fn tags(mut self, input: crate::types::TagListEntry) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your location.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagListEntry>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Specifies labels that help you categorize, filter, and search for your Amazon Web Services resources. We recommend creating at least a name tag for your location.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagListEntry>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`CreateLocationSmbInput`](crate::operation::create_location_smb::CreateLocationSmbInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_location_smb::CreateLocationSmbInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_location_smb::CreateLocationSmbInput {
            subdirectory: self.subdirectory,
            server_hostname: self.server_hostname,
            user: self.user,
            domain: self.domain,
            password: self.password,
            agent_arns: self.agent_arns,
            mount_options: self.mount_options,
            tags: self.tags,
        })
    }
}
impl ::std::fmt::Debug for CreateLocationSmbInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CreateLocationSmbInputBuilder");
        formatter.field("subdirectory", &self.subdirectory);
        formatter.field("server_hostname", &self.server_hostname);
        formatter.field("user", &self.user);
        formatter.field("domain", &self.domain);
        formatter.field("password", &"*** Sensitive Data Redacted ***");
        formatter.field("agent_arns", &self.agent_arns);
        formatter.field("mount_options", &self.mount_options);
        formatter.field("tags", &self.tags);
        formatter.finish()
    }
}
