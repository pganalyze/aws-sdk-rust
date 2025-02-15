// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a vehicle.</p>
/// <p>To return this information about vehicles in your account, you can use the API operation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VehicleSummary {
    /// <p>The unique ID of the vehicle.</p>
    pub vehicle_name: ::std::string::String,
    /// <p>The Amazon Resource Name (ARN) of the vehicle.</p>
    pub arn: ::std::string::String,
    /// <p>The ARN of a vehicle model (model manifest) associated with the vehicle.</p>
    pub model_manifest_arn: ::std::string::String,
    /// <p>The ARN of a decoder manifest associated with the vehicle.</p>
    pub decoder_manifest_arn: ::std::string::String,
    /// <p>The time the vehicle was created in seconds since epoch (January 1, 1970 at midnight UTC time).</p>
    pub creation_time: ::aws_smithy_types::DateTime,
    /// <p>The time the vehicle was last updated in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub last_modification_time: ::aws_smithy_types::DateTime,
}
impl VehicleSummary {
    /// <p>The unique ID of the vehicle.</p>
    pub fn vehicle_name(&self) -> &str {
        use std::ops::Deref;
        self.vehicle_name.deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the vehicle.</p>
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }
    /// <p>The ARN of a vehicle model (model manifest) associated with the vehicle.</p>
    pub fn model_manifest_arn(&self) -> &str {
        use std::ops::Deref;
        self.model_manifest_arn.deref()
    }
    /// <p>The ARN of a decoder manifest associated with the vehicle.</p>
    pub fn decoder_manifest_arn(&self) -> &str {
        use std::ops::Deref;
        self.decoder_manifest_arn.deref()
    }
    /// <p>The time the vehicle was created in seconds since epoch (January 1, 1970 at midnight UTC time).</p>
    pub fn creation_time(&self) -> &::aws_smithy_types::DateTime {
        &self.creation_time
    }
    /// <p>The time the vehicle was last updated in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub fn last_modification_time(&self) -> &::aws_smithy_types::DateTime {
        &self.last_modification_time
    }
}
impl VehicleSummary {
    /// Creates a new builder-style object to manufacture [`VehicleSummary`](crate::types::VehicleSummary).
    pub fn builder() -> crate::types::builders::VehicleSummaryBuilder {
        crate::types::builders::VehicleSummaryBuilder::default()
    }
}

/// A builder for [`VehicleSummary`](crate::types::VehicleSummary).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VehicleSummaryBuilder {
    pub(crate) vehicle_name: ::std::option::Option<::std::string::String>,
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) model_manifest_arn: ::std::option::Option<::std::string::String>,
    pub(crate) decoder_manifest_arn: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) last_modification_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl VehicleSummaryBuilder {
    /// <p>The unique ID of the vehicle.</p>
    /// This field is required.
    pub fn vehicle_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vehicle_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique ID of the vehicle.</p>
    pub fn set_vehicle_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vehicle_name = input;
        self
    }
    /// <p>The unique ID of the vehicle.</p>
    pub fn get_vehicle_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.vehicle_name
    }
    /// <p>The Amazon Resource Name (ARN) of the vehicle.</p>
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the vehicle.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the vehicle.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The ARN of a vehicle model (model manifest) associated with the vehicle.</p>
    /// This field is required.
    pub fn model_manifest_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.model_manifest_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of a vehicle model (model manifest) associated with the vehicle.</p>
    pub fn set_model_manifest_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.model_manifest_arn = input;
        self
    }
    /// <p>The ARN of a vehicle model (model manifest) associated with the vehicle.</p>
    pub fn get_model_manifest_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.model_manifest_arn
    }
    /// <p>The ARN of a decoder manifest associated with the vehicle.</p>
    /// This field is required.
    pub fn decoder_manifest_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.decoder_manifest_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of a decoder manifest associated with the vehicle.</p>
    pub fn set_decoder_manifest_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.decoder_manifest_arn = input;
        self
    }
    /// <p>The ARN of a decoder manifest associated with the vehicle.</p>
    pub fn get_decoder_manifest_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.decoder_manifest_arn
    }
    /// <p>The time the vehicle was created in seconds since epoch (January 1, 1970 at midnight UTC time).</p>
    /// This field is required.
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the vehicle was created in seconds since epoch (January 1, 1970 at midnight UTC time).</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The time the vehicle was created in seconds since epoch (January 1, 1970 at midnight UTC time).</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }
    /// <p>The time the vehicle was last updated in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    /// This field is required.
    pub fn last_modification_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modification_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time the vehicle was last updated in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub fn set_last_modification_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_modification_time = input;
        self
    }
    /// <p>The time the vehicle was last updated in seconds since epoch (January 1, 1970 at midnight UTC time). </p>
    pub fn get_last_modification_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_modification_time
    }
    /// Consumes the builder and constructs a [`VehicleSummary`](crate::types::VehicleSummary).
    /// This method will fail if any of the following fields are not set:
    /// - [`vehicle_name`](crate::types::builders::VehicleSummaryBuilder::vehicle_name)
    /// - [`arn`](crate::types::builders::VehicleSummaryBuilder::arn)
    /// - [`model_manifest_arn`](crate::types::builders::VehicleSummaryBuilder::model_manifest_arn)
    /// - [`decoder_manifest_arn`](crate::types::builders::VehicleSummaryBuilder::decoder_manifest_arn)
    /// - [`creation_time`](crate::types::builders::VehicleSummaryBuilder::creation_time)
    /// - [`last_modification_time`](crate::types::builders::VehicleSummaryBuilder::last_modification_time)
    pub fn build(self) -> ::std::result::Result<crate::types::VehicleSummary, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::VehicleSummary {
            vehicle_name: self.vehicle_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "vehicle_name",
                    "vehicle_name was not specified but it is required when building VehicleSummary",
                )
            })?,
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building VehicleSummary",
                )
            })?,
            model_manifest_arn: self.model_manifest_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "model_manifest_arn",
                    "model_manifest_arn was not specified but it is required when building VehicleSummary",
                )
            })?,
            decoder_manifest_arn: self.decoder_manifest_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "decoder_manifest_arn",
                    "decoder_manifest_arn was not specified but it is required when building VehicleSummary",
                )
            })?,
            creation_time: self.creation_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "creation_time",
                    "creation_time was not specified but it is required when building VehicleSummary",
                )
            })?,
            last_modification_time: self.last_modification_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "last_modification_time",
                    "last_modification_time was not specified but it is required when building VehicleSummary",
                )
            })?,
        })
    }
}
