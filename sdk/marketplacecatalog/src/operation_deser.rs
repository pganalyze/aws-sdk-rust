// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_cancel_change_set_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CancelChangeSetOutput, crate::error::CancelChangeSetError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::CancelChangeSetError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::CancelChangeSetError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::CancelChangeSetError {
            meta: generic,
            kind: crate::error::CancelChangeSetErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CancelChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::CancelChangeSetError {
            meta: generic,
            kind: crate::error::CancelChangeSetErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CancelChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceInUseException" => crate::error::CancelChangeSetError {
            meta: generic,
            kind: crate::error::CancelChangeSetErrorKind::ResourceInUseException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CancelChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::CancelChangeSetError {
            meta: generic,
            kind: crate::error::CancelChangeSetErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CancelChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::CancelChangeSetError {
            meta: generic,
            kind: crate::error::CancelChangeSetErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CancelChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::CancelChangeSetError {
            meta: generic,
            kind: crate::error::CancelChangeSetErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CancelChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::CancelChangeSetError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_cancel_change_set_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CancelChangeSetOutput, crate::error::CancelChangeSetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::cancel_change_set_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_cancel_change_set(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::CancelChangeSetError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_change_set_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeChangeSetOutput, crate::error::DescribeChangeSetError>
{
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeChangeSetError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeChangeSetError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DescribeChangeSetError {
            meta: generic,
            kind: crate::error::DescribeChangeSetErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::DescribeChangeSetError {
            meta: generic,
            kind: crate::error::DescribeChangeSetErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::DescribeChangeSetError {
            meta: generic,
            kind: crate::error::DescribeChangeSetErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::DescribeChangeSetError {
            meta: generic,
            kind: crate::error::DescribeChangeSetErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::DescribeChangeSetError {
            meta: generic,
            kind: crate::error::DescribeChangeSetErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeChangeSetError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_change_set_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeChangeSetOutput, crate::error::DescribeChangeSetError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_change_set_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_change_set(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeChangeSetError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_entity_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeEntityOutput, crate::error::DescribeEntityError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DescribeEntityError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::DescribeEntityError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DescribeEntityError {
            meta: generic,
            kind: crate::error::DescribeEntityErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeEntityError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::DescribeEntityError {
            meta: generic,
            kind: crate::error::DescribeEntityErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeEntityError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::DescribeEntityError {
            meta: generic,
            kind: crate::error::DescribeEntityErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeEntityError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotSupportedException" => crate::error::DescribeEntityError {
            meta: generic,
            kind: crate::error::DescribeEntityErrorKind::ResourceNotSupportedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::resource_not_supported_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_supported_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeEntityError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::DescribeEntityError {
            meta: generic,
            kind: crate::error::DescribeEntityErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeEntityError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::DescribeEntityError {
            meta: generic,
            kind: crate::error::DescribeEntityErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeEntityError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::DescribeEntityError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_describe_entity_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DescribeEntityOutput, crate::error::DescribeEntityError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_entity_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_describe_entity(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::DescribeEntityError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_change_sets_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListChangeSetsOutput, crate::error::ListChangeSetsError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListChangeSetsError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListChangeSetsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListChangeSetsError {
            meta: generic,
            kind: crate::error::ListChangeSetsErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListChangeSetsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::ListChangeSetsError {
            meta: generic,
            kind: crate::error::ListChangeSetsErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListChangeSetsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::ListChangeSetsError {
            meta: generic,
            kind: crate::error::ListChangeSetsErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListChangeSetsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListChangeSetsError {
            meta: generic,
            kind: crate::error::ListChangeSetsErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListChangeSetsError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListChangeSetsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_change_sets_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListChangeSetsOutput, crate::error::ListChangeSetsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_change_sets_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_change_sets(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::ListChangeSetsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_entities_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListEntitiesOutput, crate::error::ListEntitiesError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::ListEntitiesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::ListEntitiesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ListEntitiesError {
            meta: generic,
            kind: crate::error::ListEntitiesErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEntitiesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::ListEntitiesError {
            meta: generic,
            kind: crate::error::ListEntitiesErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEntitiesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::ListEntitiesError {
            meta: generic,
            kind: crate::error::ListEntitiesErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEntitiesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::ListEntitiesError {
            meta: generic,
            kind: crate::error::ListEntitiesErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEntitiesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::ListEntitiesError {
            meta: generic,
            kind: crate::error::ListEntitiesErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListEntitiesError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::ListEntitiesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_list_entities_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::ListEntitiesOutput, crate::error::ListEntitiesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_entities_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_list_entities(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::ListEntitiesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_change_set_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::StartChangeSetOutput, crate::error::StartChangeSetError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::StartChangeSetError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::StartChangeSetError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::StartChangeSetError {
            meta: generic,
            kind: crate::error::StartChangeSetErrorKind::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "InternalServiceException" => crate::error::StartChangeSetError {
            meta: generic,
            kind: crate::error::StartChangeSetErrorKind::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceInUseException" => crate::error::StartChangeSetError {
            meta: generic,
            kind: crate::error::StartChangeSetErrorKind::ResourceInUseException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ResourceNotFoundException" => crate::error::StartChangeSetError {
            meta: generic,
            kind: crate::error::StartChangeSetErrorKind::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ServiceQuotaExceededException" => crate::error::StartChangeSetError {
            meta: generic,
            kind: crate::error::StartChangeSetErrorKind::ServiceQuotaExceededException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output =
                        crate::error::service_quota_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_service_quota_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingException" => crate::error::StartChangeSetError {
            meta: generic,
            kind: crate::error::StartChangeSetErrorKind::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ValidationException" => crate::error::StartChangeSetError {
            meta: generic,
            kind: crate::error::StartChangeSetErrorKind::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartChangeSetError::unhandled)?;
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::StartChangeSetError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_start_change_set_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::StartChangeSetOutput, crate::error::StartChangeSetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_change_set_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_start_change_set(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::StartChangeSetError::unhandled)?;
        output.build()
    })
}
