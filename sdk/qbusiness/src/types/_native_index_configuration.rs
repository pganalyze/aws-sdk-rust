// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Configuration information for an Amazon Q index.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NativeIndexConfiguration {
    /// <p>The identifier for the Amazon Q index.</p>
    pub index_id: ::std::string::String,
}
impl NativeIndexConfiguration {
    /// <p>The identifier for the Amazon Q index.</p>
    pub fn index_id(&self) -> &str {
        use std::ops::Deref;
        self.index_id.deref()
    }
}
impl NativeIndexConfiguration {
    /// Creates a new builder-style object to manufacture [`NativeIndexConfiguration`](crate::types::NativeIndexConfiguration).
    pub fn builder() -> crate::types::builders::NativeIndexConfigurationBuilder {
        crate::types::builders::NativeIndexConfigurationBuilder::default()
    }
}

/// A builder for [`NativeIndexConfiguration`](crate::types::NativeIndexConfiguration).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct NativeIndexConfigurationBuilder {
    pub(crate) index_id: ::std::option::Option<::std::string::String>,
}
impl NativeIndexConfigurationBuilder {
    /// <p>The identifier for the Amazon Q index.</p>
    /// This field is required.
    pub fn index_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the Amazon Q index.</p>
    pub fn set_index_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_id = input;
        self
    }
    /// <p>The identifier for the Amazon Q index.</p>
    pub fn get_index_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_id
    }
    /// Consumes the builder and constructs a [`NativeIndexConfiguration`](crate::types::NativeIndexConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`index_id`](crate::types::builders::NativeIndexConfigurationBuilder::index_id)
    pub fn build(self) -> ::std::result::Result<crate::types::NativeIndexConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::NativeIndexConfiguration {
            index_id: self.index_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "index_id",
                    "index_id was not specified but it is required when building NativeIndexConfiguration",
                )
            })?,
        })
    }
}
