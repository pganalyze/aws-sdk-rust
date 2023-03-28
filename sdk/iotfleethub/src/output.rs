// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UpdateApplicationOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for UpdateApplicationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UpdateApplicationOutput {
    /// Creates a new builder-style object to manufacture [`UpdateApplicationOutput`](crate::output::UpdateApplicationOutput).
    pub fn builder() -> crate::output::update_application_output::Builder {
        crate::output::update_application_output::Builder::default()
    }
}

/// See [`UpdateApplicationOutput`](crate::output::UpdateApplicationOutput).
pub mod update_application_output {

    /// A builder for [`UpdateApplicationOutput`](crate::output::UpdateApplicationOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        _request_id: Option<String>,
    }
    impl Builder {
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
            self._request_id = Some(request_id.into());
            self
        }

        pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
            self._request_id = request_id;
            self
        }
        /// Consumes the builder and constructs a [`UpdateApplicationOutput`](crate::output::UpdateApplicationOutput).
        pub fn build(self) -> crate::output::UpdateApplicationOutput {
            crate::output::UpdateApplicationOutput {
                _request_id: self._request_id,
            }
        }
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourceOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for UntagResourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {

    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        _request_id: Option<String>,
    }
    impl Builder {
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
            self._request_id = Some(request_id.into());
            self
        }

        pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
            self._request_id = request_id;
            self
        }
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {
                _request_id: self._request_id,
            }
        }
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourceOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for TagResourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {

    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        _request_id: Option<String>,
    }
    impl Builder {
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
            self._request_id = Some(request_id.into());
            self
        }

        pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
            self._request_id = request_id;
            self
        }
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {
                _request_id: self._request_id,
            }
        }
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsForResourceOutput {
    /// <p>The list of tags assigned to the resource.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    _request_id: Option<String>,
}
impl ListTagsForResourceOutput {
    /// <p>The list of tags assigned to the resource.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl aws_http::request_id::RequestId for ListTagsForResourceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListTagsForResourceOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    pub fn builder() -> crate::output::list_tags_for_resource_output::Builder {
        crate::output::list_tags_for_resource_output::Builder::default()
    }
}

/// See [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
pub mod list_tags_for_resource_output {

    /// A builder for [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        _request_id: Option<String>,
    }
    impl Builder {
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>The list of tags assigned to the resource.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>The list of tags assigned to the resource.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
            self._request_id = Some(request_id.into());
            self
        }

        pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
            self._request_id = request_id;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsForResourceOutput`](crate::output::ListTagsForResourceOutput).
        pub fn build(self) -> crate::output::ListTagsForResourceOutput {
            crate::output::ListTagsForResourceOutput {
                tags: self.tags,
                _request_id: self._request_id,
            }
        }
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListApplicationsOutput {
    /// <p>An array of objects that provide summaries of information about the web applications in the list.</p>
    #[doc(hidden)]
    pub application_summaries: std::option::Option<std::vec::Vec<crate::model::ApplicationSummary>>,
    /// <p>A token used to get the next set of results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl ListApplicationsOutput {
    /// <p>An array of objects that provide summaries of information about the web applications in the list.</p>
    pub fn application_summaries(
        &self,
    ) -> std::option::Option<&[crate::model::ApplicationSummary]> {
        self.application_summaries.as_deref()
    }
    /// <p>A token used to get the next set of results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl aws_http::request_id::RequestId for ListApplicationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListApplicationsOutput {
    /// Creates a new builder-style object to manufacture [`ListApplicationsOutput`](crate::output::ListApplicationsOutput).
    pub fn builder() -> crate::output::list_applications_output::Builder {
        crate::output::list_applications_output::Builder::default()
    }
}

/// See [`ListApplicationsOutput`](crate::output::ListApplicationsOutput).
pub mod list_applications_output {

    /// A builder for [`ListApplicationsOutput`](crate::output::ListApplicationsOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) application_summaries:
            std::option::Option<std::vec::Vec<crate::model::ApplicationSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// Appends an item to `application_summaries`.
        ///
        /// To override the contents of this collection use [`set_application_summaries`](Self::set_application_summaries).
        ///
        /// <p>An array of objects that provide summaries of information about the web applications in the list.</p>
        pub fn application_summaries(mut self, input: crate::model::ApplicationSummary) -> Self {
            let mut v = self.application_summaries.unwrap_or_default();
            v.push(input);
            self.application_summaries = Some(v);
            self
        }
        /// <p>An array of objects that provide summaries of information about the web applications in the list.</p>
        pub fn set_application_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::ApplicationSummary>>,
        ) -> Self {
            self.application_summaries = input;
            self
        }
        /// <p>A token used to get the next set of results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>A token used to get the next set of results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
            self._request_id = Some(request_id.into());
            self
        }

        pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
            self._request_id = request_id;
            self
        }
        /// Consumes the builder and constructs a [`ListApplicationsOutput`](crate::output::ListApplicationsOutput).
        pub fn build(self) -> crate::output::ListApplicationsOutput {
            crate::output::ListApplicationsOutput {
                application_summaries: self.application_summaries,
                next_token: self.next_token,
                _request_id: self._request_id,
            }
        }
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeApplicationOutput {
    /// <p>The unique Id of the web application.</p>
    #[doc(hidden)]
    pub application_id: std::option::Option<std::string::String>,
    /// <p>The ARN of the web application.</p>
    #[doc(hidden)]
    pub application_arn: std::option::Option<std::string::String>,
    /// <p>The name of the web application.</p>
    #[doc(hidden)]
    pub application_name: std::option::Option<std::string::String>,
    /// <p>An optional description of the web application.</p>
    #[doc(hidden)]
    pub application_description: std::option::Option<std::string::String>,
    /// <p>The URL of the web application.</p>
    #[doc(hidden)]
    pub application_url: std::option::Option<std::string::String>,
    /// <p>The current state of the web application.</p>
    #[doc(hidden)]
    pub application_state: std::option::Option<crate::model::ApplicationState>,
    /// <p>The date (in Unix epoch time) when the application was created.</p>
    #[doc(hidden)]
    pub application_creation_date: i64,
    /// <p>The date (in Unix epoch time) when the application was last updated.</p>
    #[doc(hidden)]
    pub application_last_update_date: i64,
    /// <p>The ARN of the role that the web application assumes when it interacts with AWS IoT Core.</p>
    #[doc(hidden)]
    pub role_arn: std::option::Option<std::string::String>,
    /// <p>The Id of the single sign-on client that you use to authenticate and authorize users on the web application.</p>
    #[doc(hidden)]
    pub sso_client_id: std::option::Option<std::string::String>,
    /// <p>A message indicating why the <code>DescribeApplication</code> API failed.</p>
    #[doc(hidden)]
    pub error_message: std::option::Option<std::string::String>,
    /// <p>A set of key/value pairs that you can use to manage the web application resource.</p>
    #[doc(hidden)]
    pub tags:
        std::option::Option<std::collections::HashMap<std::string::String, std::string::String>>,
    _request_id: Option<String>,
}
impl DescribeApplicationOutput {
    /// <p>The unique Id of the web application.</p>
    pub fn application_id(&self) -> std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The ARN of the web application.</p>
    pub fn application_arn(&self) -> std::option::Option<&str> {
        self.application_arn.as_deref()
    }
    /// <p>The name of the web application.</p>
    pub fn application_name(&self) -> std::option::Option<&str> {
        self.application_name.as_deref()
    }
    /// <p>An optional description of the web application.</p>
    pub fn application_description(&self) -> std::option::Option<&str> {
        self.application_description.as_deref()
    }
    /// <p>The URL of the web application.</p>
    pub fn application_url(&self) -> std::option::Option<&str> {
        self.application_url.as_deref()
    }
    /// <p>The current state of the web application.</p>
    pub fn application_state(&self) -> std::option::Option<&crate::model::ApplicationState> {
        self.application_state.as_ref()
    }
    /// <p>The date (in Unix epoch time) when the application was created.</p>
    pub fn application_creation_date(&self) -> i64 {
        self.application_creation_date
    }
    /// <p>The date (in Unix epoch time) when the application was last updated.</p>
    pub fn application_last_update_date(&self) -> i64 {
        self.application_last_update_date
    }
    /// <p>The ARN of the role that the web application assumes when it interacts with AWS IoT Core.</p>
    pub fn role_arn(&self) -> std::option::Option<&str> {
        self.role_arn.as_deref()
    }
    /// <p>The Id of the single sign-on client that you use to authenticate and authorize users on the web application.</p>
    pub fn sso_client_id(&self) -> std::option::Option<&str> {
        self.sso_client_id.as_deref()
    }
    /// <p>A message indicating why the <code>DescribeApplication</code> API failed.</p>
    pub fn error_message(&self) -> std::option::Option<&str> {
        self.error_message.as_deref()
    }
    /// <p>A set of key/value pairs that you can use to manage the web application resource.</p>
    pub fn tags(
        &self,
    ) -> std::option::Option<&std::collections::HashMap<std::string::String, std::string::String>>
    {
        self.tags.as_ref()
    }
}
impl aws_http::request_id::RequestId for DescribeApplicationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeApplicationOutput {
    /// Creates a new builder-style object to manufacture [`DescribeApplicationOutput`](crate::output::DescribeApplicationOutput).
    pub fn builder() -> crate::output::describe_application_output::Builder {
        crate::output::describe_application_output::Builder::default()
    }
}

/// See [`DescribeApplicationOutput`](crate::output::DescribeApplicationOutput).
pub mod describe_application_output {

    /// A builder for [`DescribeApplicationOutput`](crate::output::DescribeApplicationOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) application_id: std::option::Option<std::string::String>,
        pub(crate) application_arn: std::option::Option<std::string::String>,
        pub(crate) application_name: std::option::Option<std::string::String>,
        pub(crate) application_description: std::option::Option<std::string::String>,
        pub(crate) application_url: std::option::Option<std::string::String>,
        pub(crate) application_state: std::option::Option<crate::model::ApplicationState>,
        pub(crate) application_creation_date: std::option::Option<i64>,
        pub(crate) application_last_update_date: std::option::Option<i64>,
        pub(crate) role_arn: std::option::Option<std::string::String>,
        pub(crate) sso_client_id: std::option::Option<std::string::String>,
        pub(crate) error_message: std::option::Option<std::string::String>,
        pub(crate) tags: std::option::Option<
            std::collections::HashMap<std::string::String, std::string::String>,
        >,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>The unique Id of the web application.</p>
        pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_id = Some(input.into());
            self
        }
        /// <p>The unique Id of the web application.</p>
        pub fn set_application_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_id = input;
            self
        }
        /// <p>The ARN of the web application.</p>
        pub fn application_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the web application.</p>
        pub fn set_application_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_arn = input;
            self
        }
        /// <p>The name of the web application.</p>
        pub fn application_name(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_name = Some(input.into());
            self
        }
        /// <p>The name of the web application.</p>
        pub fn set_application_name(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_name = input;
            self
        }
        /// <p>An optional description of the web application.</p>
        pub fn application_description(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_description = Some(input.into());
            self
        }
        /// <p>An optional description of the web application.</p>
        pub fn set_application_description(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_description = input;
            self
        }
        /// <p>The URL of the web application.</p>
        pub fn application_url(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_url = Some(input.into());
            self
        }
        /// <p>The URL of the web application.</p>
        pub fn set_application_url(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_url = input;
            self
        }
        /// <p>The current state of the web application.</p>
        pub fn application_state(mut self, input: crate::model::ApplicationState) -> Self {
            self.application_state = Some(input);
            self
        }
        /// <p>The current state of the web application.</p>
        pub fn set_application_state(
            mut self,
            input: std::option::Option<crate::model::ApplicationState>,
        ) -> Self {
            self.application_state = input;
            self
        }
        /// <p>The date (in Unix epoch time) when the application was created.</p>
        pub fn application_creation_date(mut self, input: i64) -> Self {
            self.application_creation_date = Some(input);
            self
        }
        /// <p>The date (in Unix epoch time) when the application was created.</p>
        pub fn set_application_creation_date(mut self, input: std::option::Option<i64>) -> Self {
            self.application_creation_date = input;
            self
        }
        /// <p>The date (in Unix epoch time) when the application was last updated.</p>
        pub fn application_last_update_date(mut self, input: i64) -> Self {
            self.application_last_update_date = Some(input);
            self
        }
        /// <p>The date (in Unix epoch time) when the application was last updated.</p>
        pub fn set_application_last_update_date(mut self, input: std::option::Option<i64>) -> Self {
            self.application_last_update_date = input;
            self
        }
        /// <p>The ARN of the role that the web application assumes when it interacts with AWS IoT Core.</p>
        pub fn role_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.role_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the role that the web application assumes when it interacts with AWS IoT Core.</p>
        pub fn set_role_arn(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.role_arn = input;
            self
        }
        /// <p>The Id of the single sign-on client that you use to authenticate and authorize users on the web application.</p>
        pub fn sso_client_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.sso_client_id = Some(input.into());
            self
        }
        /// <p>The Id of the single sign-on client that you use to authenticate and authorize users on the web application.</p>
        pub fn set_sso_client_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.sso_client_id = input;
            self
        }
        /// <p>A message indicating why the <code>DescribeApplication</code> API failed.</p>
        pub fn error_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.error_message = Some(input.into());
            self
        }
        /// <p>A message indicating why the <code>DescribeApplication</code> API failed.</p>
        pub fn set_error_message(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.error_message = input;
            self
        }
        /// Adds a key-value pair to `tags`.
        ///
        /// To override the contents of this collection use [`set_tags`](Self::set_tags).
        ///
        /// <p>A set of key/value pairs that you can use to manage the web application resource.</p>
        pub fn tags(
            mut self,
            k: impl Into<std::string::String>,
            v: impl Into<std::string::String>,
        ) -> Self {
            let mut hash_map = self.tags.unwrap_or_default();
            hash_map.insert(k.into(), v.into());
            self.tags = Some(hash_map);
            self
        }
        /// <p>A set of key/value pairs that you can use to manage the web application resource.</p>
        pub fn set_tags(
            mut self,
            input: std::option::Option<
                std::collections::HashMap<std::string::String, std::string::String>,
            >,
        ) -> Self {
            self.tags = input;
            self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
            self._request_id = Some(request_id.into());
            self
        }

        pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
            self._request_id = request_id;
            self
        }
        /// Consumes the builder and constructs a [`DescribeApplicationOutput`](crate::output::DescribeApplicationOutput).
        pub fn build(self) -> crate::output::DescribeApplicationOutput {
            crate::output::DescribeApplicationOutput {
                application_id: self.application_id,
                application_arn: self.application_arn,
                application_name: self.application_name,
                application_description: self.application_description,
                application_url: self.application_url,
                application_state: self.application_state,
                application_creation_date: self.application_creation_date.unwrap_or_default(),
                application_last_update_date: self.application_last_update_date.unwrap_or_default(),
                role_arn: self.role_arn,
                sso_client_id: self.sso_client_id,
                error_message: self.error_message,
                tags: self.tags,
                _request_id: self._request_id,
            }
        }
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteApplicationOutput {
    _request_id: Option<String>,
}
impl aws_http::request_id::RequestId for DeleteApplicationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteApplicationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteApplicationOutput`](crate::output::DeleteApplicationOutput).
    pub fn builder() -> crate::output::delete_application_output::Builder {
        crate::output::delete_application_output::Builder::default()
    }
}

/// See [`DeleteApplicationOutput`](crate::output::DeleteApplicationOutput).
pub mod delete_application_output {

    /// A builder for [`DeleteApplicationOutput`](crate::output::DeleteApplicationOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        _request_id: Option<String>,
    }
    impl Builder {
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
            self._request_id = Some(request_id.into());
            self
        }

        pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
            self._request_id = request_id;
            self
        }
        /// Consumes the builder and constructs a [`DeleteApplicationOutput`](crate::output::DeleteApplicationOutput).
        pub fn build(self) -> crate::output::DeleteApplicationOutput {
            crate::output::DeleteApplicationOutput {
                _request_id: self._request_id,
            }
        }
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateApplicationOutput {
    /// <p>The unique Id of the web application.</p>
    #[doc(hidden)]
    pub application_id: std::option::Option<std::string::String>,
    /// <p>The ARN of the web application.</p>
    #[doc(hidden)]
    pub application_arn: std::option::Option<std::string::String>,
    _request_id: Option<String>,
}
impl CreateApplicationOutput {
    /// <p>The unique Id of the web application.</p>
    pub fn application_id(&self) -> std::option::Option<&str> {
        self.application_id.as_deref()
    }
    /// <p>The ARN of the web application.</p>
    pub fn application_arn(&self) -> std::option::Option<&str> {
        self.application_arn.as_deref()
    }
}
impl aws_http::request_id::RequestId for CreateApplicationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateApplicationOutput {
    /// Creates a new builder-style object to manufacture [`CreateApplicationOutput`](crate::output::CreateApplicationOutput).
    pub fn builder() -> crate::output::create_application_output::Builder {
        crate::output::create_application_output::Builder::default()
    }
}

/// See [`CreateApplicationOutput`](crate::output::CreateApplicationOutput).
pub mod create_application_output {

    /// A builder for [`CreateApplicationOutput`](crate::output::CreateApplicationOutput).
    #[non_exhaustive]
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) application_id: std::option::Option<std::string::String>,
        pub(crate) application_arn: std::option::Option<std::string::String>,
        _request_id: Option<String>,
    }
    impl Builder {
        /// <p>The unique Id of the web application.</p>
        pub fn application_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_id = Some(input.into());
            self
        }
        /// <p>The unique Id of the web application.</p>
        pub fn set_application_id(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_id = input;
            self
        }
        /// <p>The ARN of the web application.</p>
        pub fn application_arn(mut self, input: impl Into<std::string::String>) -> Self {
            self.application_arn = Some(input.into());
            self
        }
        /// <p>The ARN of the web application.</p>
        pub fn set_application_arn(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.application_arn = input;
            self
        }
        pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
            self._request_id = Some(request_id.into());
            self
        }

        pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
            self._request_id = request_id;
            self
        }
        /// Consumes the builder and constructs a [`CreateApplicationOutput`](crate::output::CreateApplicationOutput).
        pub fn build(self) -> crate::output::CreateApplicationOutput {
            crate::output::CreateApplicationOutput {
                application_id: self.application_id,
                application_arn: self.application_arn,
                _request_id: self._request_id,
            }
        }
    }
}
