// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TestMappingInput {
    /// <p>Specify the EDI (electronic data interchange) file that is used as input for the transform.</p>
    pub input_file_content: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub mapping_template: ::std::option::Option<::std::string::String>,
    /// <p>Specifies that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub file_format: ::std::option::Option<crate::types::FileFormat>,
}
impl TestMappingInput {
    /// <p>Specify the EDI (electronic data interchange) file that is used as input for the transform.</p>
    pub fn input_file_content(&self) -> ::std::option::Option<&str> {
        self.input_file_content.as_deref()
    }
    /// <p>Specifies the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub fn mapping_template(&self) -> ::std::option::Option<&str> {
        self.mapping_template.as_deref()
    }
    /// <p>Specifies that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub fn file_format(&self) -> ::std::option::Option<&crate::types::FileFormat> {
        self.file_format.as_ref()
    }
}
impl TestMappingInput {
    /// Creates a new builder-style object to manufacture [`TestMappingInput`](crate::operation::test_mapping::TestMappingInput).
    pub fn builder() -> crate::operation::test_mapping::builders::TestMappingInputBuilder {
        crate::operation::test_mapping::builders::TestMappingInputBuilder::default()
    }
}

/// A builder for [`TestMappingInput`](crate::operation::test_mapping::TestMappingInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TestMappingInputBuilder {
    pub(crate) input_file_content: ::std::option::Option<::std::string::String>,
    pub(crate) mapping_template: ::std::option::Option<::std::string::String>,
    pub(crate) file_format: ::std::option::Option<crate::types::FileFormat>,
}
impl TestMappingInputBuilder {
    /// <p>Specify the EDI (electronic data interchange) file that is used as input for the transform.</p>
    /// This field is required.
    pub fn input_file_content(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.input_file_content = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify the EDI (electronic data interchange) file that is used as input for the transform.</p>
    pub fn set_input_file_content(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.input_file_content = input;
        self
    }
    /// <p>Specify the EDI (electronic data interchange) file that is used as input for the transform.</p>
    pub fn get_input_file_content(&self) -> &::std::option::Option<::std::string::String> {
        &self.input_file_content
    }
    /// <p>Specifies the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    /// This field is required.
    pub fn mapping_template(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mapping_template = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specifies the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub fn set_mapping_template(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mapping_template = input;
        self
    }
    /// <p>Specifies the name of the mapping template for the transformer. This template is used to convert the input document into the correct set of objects.</p>
    pub fn get_mapping_template(&self) -> &::std::option::Option<::std::string::String> {
        &self.mapping_template
    }
    /// <p>Specifies that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    /// This field is required.
    pub fn file_format(mut self, input: crate::types::FileFormat) -> Self {
        self.file_format = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub fn set_file_format(mut self, input: ::std::option::Option<crate::types::FileFormat>) -> Self {
        self.file_format = input;
        self
    }
    /// <p>Specifies that the currently supported file formats for EDI transformations are <code>JSON</code> and <code>XML</code>.</p>
    pub fn get_file_format(&self) -> &::std::option::Option<crate::types::FileFormat> {
        &self.file_format
    }
    /// Consumes the builder and constructs a [`TestMappingInput`](crate::operation::test_mapping::TestMappingInput).
    pub fn build(self) -> ::std::result::Result<crate::operation::test_mapping::TestMappingInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::test_mapping::TestMappingInput {
            input_file_content: self.input_file_content,
            mapping_template: self.mapping_template,
            file_format: self.file_format,
        })
    }
}
