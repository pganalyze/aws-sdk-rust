// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartIdMappingJobInput {
    /// <p>The name of the ID mapping job to be retrieved.</p>
    pub workflow_name: ::std::option::Option<::std::string::String>,
}
impl StartIdMappingJobInput {
    /// <p>The name of the ID mapping job to be retrieved.</p>
    pub fn workflow_name(&self) -> ::std::option::Option<&str> {
        self.workflow_name.as_deref()
    }
}
impl StartIdMappingJobInput {
    /// Creates a new builder-style object to manufacture [`StartIdMappingJobInput`](crate::operation::start_id_mapping_job::StartIdMappingJobInput).
    pub fn builder() -> crate::operation::start_id_mapping_job::builders::StartIdMappingJobInputBuilder {
        crate::operation::start_id_mapping_job::builders::StartIdMappingJobInputBuilder::default()
    }
}

/// A builder for [`StartIdMappingJobInput`](crate::operation::start_id_mapping_job::StartIdMappingJobInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StartIdMappingJobInputBuilder {
    pub(crate) workflow_name: ::std::option::Option<::std::string::String>,
}
impl StartIdMappingJobInputBuilder {
    /// <p>The name of the ID mapping job to be retrieved.</p>
    /// This field is required.
    pub fn workflow_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.workflow_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the ID mapping job to be retrieved.</p>
    pub fn set_workflow_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.workflow_name = input;
        self
    }
    /// <p>The name of the ID mapping job to be retrieved.</p>
    pub fn get_workflow_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.workflow_name
    }
    /// Consumes the builder and constructs a [`StartIdMappingJobInput`](crate::operation::start_id_mapping_job::StartIdMappingJobInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::start_id_mapping_job::StartIdMappingJobInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::start_id_mapping_job::StartIdMappingJobInput {
            workflow_name: self.workflow_name,
        })
    }
}
