// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_disable_control_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DisableControlOutput, crate::error::DisableControlError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DisableControlError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DisableControlError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DisableControlError {
            meta: generic,
            kind: crate::error::DisableControlErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ConflictException" => {
            crate::error::DisableControlError {
                meta: generic,
                kind: crate::error::DisableControlErrorKind::ConflictException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::conflict_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisableControlError::unhandled)?;
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "InternalServerException" => crate::error::DisableControlError {
            meta: generic,
            kind: crate::error::DisableControlErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::DisableControlError {
            meta: generic,
            kind: crate::error::DisableControlErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ServiceQuotaExceededException" => crate::error::DisableControlError {
            meta: generic,
            kind: crate::error::DisableControlErrorKind::ServiceQuotaExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::service_quota_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_quota_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::DisableControlError {
            meta: generic,
            kind: crate::error::DisableControlErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisableControlError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::http_serde::deser_header_disable_control_throttling_exception_retry_after_seconds(response.headers())
                                                .map_err(|_|crate::error::DisableControlError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::DisableControlError {
            meta: generic,
            kind: crate::error::DisableControlErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DisableControlError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_disable_control_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DisableControlOutput, crate::error::DisableControlError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::disable_control_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_disable_control(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DisableControlError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_enable_control_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::EnableControlOutput, crate::error::EnableControlError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::EnableControlError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::EnableControlError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::EnableControlError {
            meta: generic,
            kind: crate::error::EnableControlErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::EnableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ConflictException" => {
            crate::error::EnableControlError {
                meta: generic,
                kind: crate::error::EnableControlErrorKind::ConflictException({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::conflict_exception::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::EnableControlError::unhandled)?;
                        output.build()
                    };
                    if tmp.message.is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        "InternalServerException" => crate::error::EnableControlError {
            meta: generic,
            kind: crate::error::EnableControlErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::EnableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::EnableControlError {
            meta: generic,
            kind: crate::error::EnableControlErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::EnableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ServiceQuotaExceededException" => crate::error::EnableControlError {
            meta: generic,
            kind: crate::error::EnableControlErrorKind::ServiceQuotaExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::service_quota_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_quota_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::EnableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::EnableControlError {
            meta: generic,
            kind: crate::error::EnableControlErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::EnableControlError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::http_serde::deser_header_enable_control_throttling_exception_retry_after_seconds(response.headers())
                                                .map_err(|_|crate::error::EnableControlError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::EnableControlError {
            meta: generic,
            kind: crate::error::EnableControlErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::EnableControlError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::EnableControlError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_enable_control_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::EnableControlOutput, crate::error::EnableControlError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::enable_control_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_enable_control(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::EnableControlError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_control_operation_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetControlOperationOutput,
    crate::error::GetControlOperationError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetControlOperationError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetControlOperationError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::GetControlOperationError {
            meta: generic,
            kind: crate::error::GetControlOperationErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetControlOperationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::GetControlOperationError {
            meta: generic,
            kind: crate::error::GetControlOperationErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetControlOperationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::GetControlOperationError {
            meta: generic,
            kind: crate::error::GetControlOperationErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetControlOperationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::GetControlOperationError {
            meta: generic,
            kind: crate::error::GetControlOperationErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetControlOperationError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::http_serde::deser_header_get_control_operation_throttling_exception_retry_after_seconds(response.headers())
                                                .map_err(|_|crate::error::GetControlOperationError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::GetControlOperationError {
            meta: generic,
            kind: crate::error::GetControlOperationErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetControlOperationError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetControlOperationError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_control_operation_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetControlOperationOutput,
    crate::error::GetControlOperationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_control_operation_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_control_operation(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetControlOperationError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_enabled_controls_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::ListEnabledControlsOutput,
    crate::error::ListEnabledControlsError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListEnabledControlsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListEnabledControlsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListEnabledControlsError {
            meta: generic,
            kind: crate::error::ListEnabledControlsErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEnabledControlsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServerException" => crate::error::ListEnabledControlsError {
            meta: generic,
            kind: crate::error::ListEnabledControlsErrorKind::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEnabledControlsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::ListEnabledControlsError {
            meta: generic,
            kind: crate::error::ListEnabledControlsErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEnabledControlsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::ListEnabledControlsError {
            meta: generic,
            kind: crate::error::ListEnabledControlsErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEnabledControlsError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::http_serde::deser_header_list_enabled_controls_throttling_exception_retry_after_seconds(response.headers())
                                                .map_err(|_|crate::error::ListEnabledControlsError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListEnabledControlsError {
            meta: generic,
            kind: crate::error::ListEnabledControlsErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEnabledControlsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListEnabledControlsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_enabled_controls_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::ListEnabledControlsOutput,
    crate::error::ListEnabledControlsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_enabled_controls_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_enabled_controls(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::ListEnabledControlsError::unhandled)?;
        output.build()
    })
}
