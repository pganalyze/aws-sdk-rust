// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_create_cell_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCellInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cell_name {
        object.key("cellName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.cells {
        let mut array_3 = object.key("cells").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.tags {
        #[allow(unused_mut)]
        let mut object_6 = object.key("tags").start_object();
        for (key_7, value_8) in var_5 {
            {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_cross_account_authorization_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateCrossAccountAuthorizationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_9) = &input.cross_account_authorization {
        object
            .key("crossAccountAuthorization")
            .string(var_9.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_readiness_check_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateReadinessCheckInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_10) = &input.readiness_check_name {
        object.key("readinessCheckName").string(var_10.as_str());
    }
    if let Some(var_11) = &input.resource_set_name {
        object.key("resourceSetName").string(var_11.as_str());
    }
    if let Some(var_12) = &input.tags {
        #[allow(unused_mut)]
        let mut object_13 = object.key("tags").start_object();
        for (key_14, value_15) in var_12 {
            {
                object_13.key(key_14.as_str()).string(value_15.as_str());
            }
        }
        object_13.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_recovery_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateRecoveryGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_16) = &input.cells {
        let mut array_17 = object.key("cells").start_array();
        for item_18 in var_16 {
            {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.recovery_group_name {
        object.key("recoveryGroupName").string(var_19.as_str());
    }
    if let Some(var_20) = &input.tags {
        #[allow(unused_mut)]
        let mut object_21 = object.key("tags").start_object();
        for (key_22, value_23) in var_20 {
            {
                object_21.key(key_22.as_str()).string(value_23.as_str());
            }
        }
        object_21.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_resource_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateResourceSetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_24) = &input.resource_set_name {
        object.key("resourceSetName").string(var_24.as_str());
    }
    if let Some(var_25) = &input.resource_set_type {
        object.key("resourceSetType").string(var_25.as_str());
    }
    if let Some(var_26) = &input.resources {
        let mut array_27 = object.key("resources").start_array();
        for item_28 in var_26 {
            {
                #[allow(unused_mut)]
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_resource(&mut object_29, item_28)?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.tags {
        #[allow(unused_mut)]
        let mut object_31 = object.key("tags").start_object();
        for (key_32, value_33) in var_30 {
            {
                object_31.key(key_32.as_str()).string(value_33.as_str());
            }
        }
        object_31.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_34) = &input.tags {
        #[allow(unused_mut)]
        let mut object_35 = object.key("tags").start_object();
        for (key_36, value_37) in var_34 {
            {
                object_35.key(key_36.as_str()).string(value_37.as_str());
            }
        }
        object_35.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_cell_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateCellInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_38) = &input.cells {
        let mut array_39 = object.key("cells").start_array();
        for item_40 in var_38 {
            {
                array_39.value().string(item_40.as_str());
            }
        }
        array_39.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_readiness_check_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateReadinessCheckInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_41) = &input.resource_set_name {
        object.key("resourceSetName").string(var_41.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_recovery_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateRecoveryGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_42) = &input.cells {
        let mut array_43 = object.key("cells").start_array();
        for item_44 in var_42 {
            {
                array_43.value().string(item_44.as_str());
            }
        }
        array_43.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_resource_set_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateResourceSetInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_45) = &input.resource_set_type {
        object.key("resourceSetType").string(var_45.as_str());
    }
    if let Some(var_46) = &input.resources {
        let mut array_47 = object.key("resources").start_array();
        for item_48 in var_46 {
            {
                #[allow(unused_mut)]
                let mut object_49 = array_47.value().start_object();
                crate::json_ser::serialize_structure_crate_model_resource(&mut object_49, item_48)?;
                object_49.finish();
            }
        }
        array_47.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_resource(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Resource,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.component_id {
        object.key("componentId").string(var_50.as_str());
    }
    if let Some(var_51) = &input.dns_target_resource {
        #[allow(unused_mut)]
        let mut object_52 = object.key("dnsTargetResource").start_object();
        crate::json_ser::serialize_structure_crate_model_dns_target_resource(
            &mut object_52,
            var_51,
        )?;
        object_52.finish();
    }
    if let Some(var_53) = &input.readiness_scopes {
        let mut array_54 = object.key("readinessScopes").start_array();
        for item_55 in var_53 {
            {
                array_54.value().string(item_55.as_str());
            }
        }
        array_54.finish();
    }
    if let Some(var_56) = &input.resource_arn {
        object.key("resourceArn").string(var_56.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_dns_target_resource(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::DnsTargetResource,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_57) = &input.domain_name {
        object.key("domainName").string(var_57.as_str());
    }
    if let Some(var_58) = &input.hosted_zone_arn {
        object.key("hostedZoneArn").string(var_58.as_str());
    }
    if let Some(var_59) = &input.record_set_id {
        object.key("recordSetId").string(var_59.as_str());
    }
    if let Some(var_60) = &input.record_type {
        object.key("recordType").string(var_60.as_str());
    }
    if let Some(var_61) = &input.target_resource {
        #[allow(unused_mut)]
        let mut object_62 = object.key("targetResource").start_object();
        crate::json_ser::serialize_structure_crate_model_target_resource(&mut object_62, var_61)?;
        object_62.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_target_resource(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::TargetResource,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.nlb_resource {
        #[allow(unused_mut)]
        let mut object_64 = object.key("nLBResource").start_object();
        crate::json_ser::serialize_structure_crate_model_nlb_resource(&mut object_64, var_63)?;
        object_64.finish();
    }
    if let Some(var_65) = &input.r53_resource {
        #[allow(unused_mut)]
        let mut object_66 = object.key("r53Resource").start_object();
        crate::json_ser::serialize_structure_crate_model_r53_resource_record(
            &mut object_66,
            var_65,
        )?;
        object_66.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_nlb_resource(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::NlbResource,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_67) = &input.arn {
        object.key("arn").string(var_67.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_r53_resource_record(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::R53ResourceRecord,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_68) = &input.domain_name {
        object.key("domainName").string(var_68.as_str());
    }
    if let Some(var_69) = &input.record_set_id {
        object.key("recordSetId").string(var_69.as_str());
    }
    Ok(())
}
