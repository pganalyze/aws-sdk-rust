// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CreateConfigurationSet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_configuration_set`](crate::client::Client::create_configuration_set).
///
/// See [`crate::client::fluent_builders::CreateConfigurationSet`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateConfigurationSet {
    _private: (),
}
impl CreateConfigurationSet {
    /// Creates a new builder-style object to manufacture [`CreateConfigurationSetInput`](crate::input::CreateConfigurationSetInput).
    pub fn builder() -> crate::input::create_configuration_set_input::Builder {
        crate::input::create_configuration_set_input::Builder::default()
    }
    /// Creates a new `CreateConfigurationSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateConfigurationSet {
    type Output = std::result::Result<
        crate::output::CreateConfigurationSetOutput,
        crate::error::CreateConfigurationSetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_configuration_set_error(response)
        } else {
            crate::operation_deser::parse_create_configuration_set_response(response)
        }
    }
}

/// Operation shape for `CreateConfigurationSetEventDestination`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`create_configuration_set_event_destination`](crate::client::Client::create_configuration_set_event_destination).
///
/// See [`crate::client::fluent_builders::CreateConfigurationSetEventDestination`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct CreateConfigurationSetEventDestination {
    _private: (),
}
impl CreateConfigurationSetEventDestination {
    /// Creates a new builder-style object to manufacture [`CreateConfigurationSetEventDestinationInput`](crate::input::CreateConfigurationSetEventDestinationInput).
    pub fn builder() -> crate::input::create_configuration_set_event_destination_input::Builder {
        crate::input::create_configuration_set_event_destination_input::Builder::default()
    }
    /// Creates a new `CreateConfigurationSetEventDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CreateConfigurationSetEventDestination {
    type Output = std::result::Result<
        crate::output::CreateConfigurationSetEventDestinationOutput,
        crate::error::CreateConfigurationSetEventDestinationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_create_configuration_set_event_destination_error(response)
        } else {
            crate::operation_deser::parse_create_configuration_set_event_destination_response(
                response,
            )
        }
    }
}

/// Operation shape for `DeleteConfigurationSet`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_configuration_set`](crate::client::Client::delete_configuration_set).
///
/// See [`crate::client::fluent_builders::DeleteConfigurationSet`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteConfigurationSet {
    _private: (),
}
impl DeleteConfigurationSet {
    /// Creates a new builder-style object to manufacture [`DeleteConfigurationSetInput`](crate::input::DeleteConfigurationSetInput).
    pub fn builder() -> crate::input::delete_configuration_set_input::Builder {
        crate::input::delete_configuration_set_input::Builder::default()
    }
    /// Creates a new `DeleteConfigurationSet` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteConfigurationSet {
    type Output = std::result::Result<
        crate::output::DeleteConfigurationSetOutput,
        crate::error::DeleteConfigurationSetError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_configuration_set_error(response)
        } else {
            crate::operation_deser::parse_delete_configuration_set_response(response)
        }
    }
}

/// Operation shape for `DeleteConfigurationSetEventDestination`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`delete_configuration_set_event_destination`](crate::client::Client::delete_configuration_set_event_destination).
///
/// See [`crate::client::fluent_builders::DeleteConfigurationSetEventDestination`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct DeleteConfigurationSetEventDestination {
    _private: (),
}
impl DeleteConfigurationSetEventDestination {
    /// Creates a new builder-style object to manufacture [`DeleteConfigurationSetEventDestinationInput`](crate::input::DeleteConfigurationSetEventDestinationInput).
    pub fn builder() -> crate::input::delete_configuration_set_event_destination_input::Builder {
        crate::input::delete_configuration_set_event_destination_input::Builder::default()
    }
    /// Creates a new `DeleteConfigurationSetEventDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DeleteConfigurationSetEventDestination {
    type Output = std::result::Result<
        crate::output::DeleteConfigurationSetEventDestinationOutput,
        crate::error::DeleteConfigurationSetEventDestinationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_delete_configuration_set_event_destination_error(response)
        } else {
            crate::operation_deser::parse_delete_configuration_set_event_destination_response(
                response,
            )
        }
    }
}

/// Operation shape for `GetConfigurationSetEventDestinations`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_configuration_set_event_destinations`](crate::client::Client::get_configuration_set_event_destinations).
///
/// See [`crate::client::fluent_builders::GetConfigurationSetEventDestinations`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct GetConfigurationSetEventDestinations {
    _private: (),
}
impl GetConfigurationSetEventDestinations {
    /// Creates a new builder-style object to manufacture [`GetConfigurationSetEventDestinationsInput`](crate::input::GetConfigurationSetEventDestinationsInput).
    pub fn builder() -> crate::input::get_configuration_set_event_destinations_input::Builder {
        crate::input::get_configuration_set_event_destinations_input::Builder::default()
    }
    /// Creates a new `GetConfigurationSetEventDestinations` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetConfigurationSetEventDestinations {
    type Output = std::result::Result<
        crate::output::GetConfigurationSetEventDestinationsOutput,
        crate::error::GetConfigurationSetEventDestinationsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_configuration_set_event_destinations_error(response)
        } else {
            crate::operation_deser::parse_get_configuration_set_event_destinations_response(
                response,
            )
        }
    }
}

/// Operation shape for `ListConfigurationSets`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`list_configuration_sets`](crate::client::Client::list_configuration_sets).
///
/// See [`crate::client::fluent_builders::ListConfigurationSets`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct ListConfigurationSets {
    _private: (),
}
impl ListConfigurationSets {
    /// Creates a new builder-style object to manufacture [`ListConfigurationSetsInput`](crate::input::ListConfigurationSetsInput).
    pub fn builder() -> crate::input::list_configuration_sets_input::Builder {
        crate::input::list_configuration_sets_input::Builder::default()
    }
    /// Creates a new `ListConfigurationSets` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for ListConfigurationSets {
    type Output = std::result::Result<
        crate::output::ListConfigurationSetsOutput,
        crate::error::ListConfigurationSetsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_list_configuration_sets_error(response)
        } else {
            crate::operation_deser::parse_list_configuration_sets_response(response)
        }
    }
}

/// Operation shape for `SendVoiceMessage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`send_voice_message`](crate::client::Client::send_voice_message).
///
/// See [`crate::client::fluent_builders::SendVoiceMessage`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct SendVoiceMessage {
    _private: (),
}
impl SendVoiceMessage {
    /// Creates a new builder-style object to manufacture [`SendVoiceMessageInput`](crate::input::SendVoiceMessageInput).
    pub fn builder() -> crate::input::send_voice_message_input::Builder {
        crate::input::send_voice_message_input::Builder::default()
    }
    /// Creates a new `SendVoiceMessage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for SendVoiceMessage {
    type Output = std::result::Result<
        crate::output::SendVoiceMessageOutput,
        crate::error::SendVoiceMessageError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_send_voice_message_error(response)
        } else {
            crate::operation_deser::parse_send_voice_message_response(response)
        }
    }
}

/// Operation shape for `UpdateConfigurationSetEventDestination`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`update_configuration_set_event_destination`](crate::client::Client::update_configuration_set_event_destination).
///
/// See [`crate::client::fluent_builders::UpdateConfigurationSetEventDestination`] for more details about the operation.
#[derive(std::clone::Clone, std::default::Default, std::fmt::Debug)]
pub struct UpdateConfigurationSetEventDestination {
    _private: (),
}
impl UpdateConfigurationSetEventDestination {
    /// Creates a new builder-style object to manufacture [`UpdateConfigurationSetEventDestinationInput`](crate::input::UpdateConfigurationSetEventDestinationInput).
    pub fn builder() -> crate::input::update_configuration_set_event_destination_input::Builder {
        crate::input::update_configuration_set_event_destination_input::Builder::default()
    }
    /// Creates a new `UpdateConfigurationSetEventDestination` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for UpdateConfigurationSetEventDestination {
    type Output = std::result::Result<
        crate::output::UpdateConfigurationSetEventDestinationOutput,
        crate::error::UpdateConfigurationSetEventDestinationError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_update_configuration_set_event_destination_error(response)
        } else {
            crate::operation_deser::parse_update_configuration_set_event_destination_response(
                response,
            )
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
