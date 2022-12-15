// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_cancel_rotate_secret_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CancelRotateSecretInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.secret_id {
        object.key("SecretId").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_secret_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSecretInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.description {
        object.key("Description").string(var_4.as_str());
    }
    if let Some(var_5) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.secret_binary {
        object
            .key("SecretBinary")
            .string_unchecked(&aws_smithy_types::base64::encode(var_6));
    }
    if let Some(var_7) = &input.secret_string {
        object.key("SecretString").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("Tags").start_array();
        for item_10 in var_8 {
            {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.add_replica_regions {
        let mut array_13 = object.key("AddReplicaRegions").start_array();
        for item_14 in var_12 {
            {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::json_ser::serialize_structure_crate_model_replica_region_type(
                    &mut object_15,
                    item_14,
                )?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if input.force_overwrite_replica_secret {
        object
            .key("ForceOverwriteReplicaSecret")
            .boolean(input.force_overwrite_replica_secret);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_16) = &input.secret_id {
        object.key("SecretId").string(var_16.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_secret_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSecretInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_17) = &input.secret_id {
        object.key("SecretId").string(var_17.as_str());
    }
    if let Some(var_18) = &input.recovery_window_in_days {
        object.key("RecoveryWindowInDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_18).into()),
        );
    }
    if let Some(var_19) = &input.force_delete_without_recovery {
        object.key("ForceDeleteWithoutRecovery").boolean(*var_19);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_secret_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSecretInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_20) = &input.secret_id {
        object.key("SecretId").string(var_20.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_random_password_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRandomPasswordInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_21) = &input.password_length {
        object.key("PasswordLength").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_21).into()),
        );
    }
    if let Some(var_22) = &input.exclude_characters {
        object.key("ExcludeCharacters").string(var_22.as_str());
    }
    if let Some(var_23) = &input.exclude_numbers {
        object.key("ExcludeNumbers").boolean(*var_23);
    }
    if let Some(var_24) = &input.exclude_punctuation {
        object.key("ExcludePunctuation").boolean(*var_24);
    }
    if let Some(var_25) = &input.exclude_uppercase {
        object.key("ExcludeUppercase").boolean(*var_25);
    }
    if let Some(var_26) = &input.exclude_lowercase {
        object.key("ExcludeLowercase").boolean(*var_26);
    }
    if let Some(var_27) = &input.include_space {
        object.key("IncludeSpace").boolean(*var_27);
    }
    if let Some(var_28) = &input.require_each_included_type {
        object.key("RequireEachIncludedType").boolean(*var_28);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_29) = &input.secret_id {
        object.key("SecretId").string(var_29.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_secret_value_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetSecretValueInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_30) = &input.secret_id {
        object.key("SecretId").string(var_30.as_str());
    }
    if let Some(var_31) = &input.version_id {
        object.key("VersionId").string(var_31.as_str());
    }
    if let Some(var_32) = &input.version_stage {
        object.key("VersionStage").string(var_32.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_secrets_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSecretsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_33) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_33).into()),
        );
    }
    if let Some(var_34) = &input.next_token {
        object.key("NextToken").string(var_34.as_str());
    }
    if let Some(var_35) = &input.filters {
        let mut array_36 = object.key("Filters").start_array();
        for item_37 in var_35 {
            {
                #[allow(unused_mut)]
                let mut object_38 = array_36.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_38, item_37)?;
                object_38.finish();
            }
        }
        array_36.finish();
    }
    if let Some(var_39) = &input.sort_order {
        object.key("SortOrder").string(var_39.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_secret_version_ids_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListSecretVersionIdsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_40) = &input.secret_id {
        object.key("SecretId").string(var_40.as_str());
    }
    if let Some(var_41) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_41).into()),
        );
    }
    if let Some(var_42) = &input.next_token {
        object.key("NextToken").string(var_42.as_str());
    }
    if let Some(var_43) = &input.include_deprecated {
        object.key("IncludeDeprecated").boolean(*var_43);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_44) = &input.secret_id {
        object.key("SecretId").string(var_44.as_str());
    }
    if let Some(var_45) = &input.resource_policy {
        object.key("ResourcePolicy").string(var_45.as_str());
    }
    if let Some(var_46) = &input.block_public_policy {
        object.key("BlockPublicPolicy").boolean(*var_46);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_put_secret_value_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::PutSecretValueInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_47) = &input.secret_id {
        object.key("SecretId").string(var_47.as_str());
    }
    if let Some(var_48) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_48.as_str());
    }
    if let Some(var_49) = &input.secret_binary {
        object
            .key("SecretBinary")
            .string_unchecked(&aws_smithy_types::base64::encode(var_49));
    }
    if let Some(var_50) = &input.secret_string {
        object.key("SecretString").string(var_50.as_str());
    }
    if let Some(var_51) = &input.version_stages {
        let mut array_52 = object.key("VersionStages").start_array();
        for item_53 in var_51 {
            {
                array_52.value().string(item_53.as_str());
            }
        }
        array_52.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_remove_regions_from_replication_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RemoveRegionsFromReplicationInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_54) = &input.secret_id {
        object.key("SecretId").string(var_54.as_str());
    }
    if let Some(var_55) = &input.remove_replica_regions {
        let mut array_56 = object.key("RemoveReplicaRegions").start_array();
        for item_57 in var_55 {
            {
                array_56.value().string(item_57.as_str());
            }
        }
        array_56.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_replicate_secret_to_regions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ReplicateSecretToRegionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_58) = &input.secret_id {
        object.key("SecretId").string(var_58.as_str());
    }
    if let Some(var_59) = &input.add_replica_regions {
        let mut array_60 = object.key("AddReplicaRegions").start_array();
        for item_61 in var_59 {
            {
                #[allow(unused_mut)]
                let mut object_62 = array_60.value().start_object();
                crate::json_ser::serialize_structure_crate_model_replica_region_type(
                    &mut object_62,
                    item_61,
                )?;
                object_62.finish();
            }
        }
        array_60.finish();
    }
    if input.force_overwrite_replica_secret {
        object
            .key("ForceOverwriteReplicaSecret")
            .boolean(input.force_overwrite_replica_secret);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_restore_secret_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RestoreSecretInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_63) = &input.secret_id {
        object.key("SecretId").string(var_63.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_rotate_secret_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::RotateSecretInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_64) = &input.secret_id {
        object.key("SecretId").string(var_64.as_str());
    }
    if let Some(var_65) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_65.as_str());
    }
    if let Some(var_66) = &input.rotation_lambda_arn {
        object.key("RotationLambdaARN").string(var_66.as_str());
    }
    if let Some(var_67) = &input.rotation_rules {
        #[allow(unused_mut)]
        let mut object_68 = object.key("RotationRules").start_object();
        crate::json_ser::serialize_structure_crate_model_rotation_rules_type(
            &mut object_68,
            var_67,
        )?;
        object_68.finish();
    }
    if let Some(var_69) = &input.rotate_immediately {
        object.key("RotateImmediately").boolean(*var_69);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_stop_replication_to_replica_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::StopReplicationToReplicaInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_70) = &input.secret_id {
        object.key("SecretId").string(var_70.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_71) = &input.secret_id {
        object.key("SecretId").string(var_71.as_str());
    }
    if let Some(var_72) = &input.tags {
        let mut array_73 = object.key("Tags").start_array();
        for item_74 in var_72 {
            {
                #[allow(unused_mut)]
                let mut object_75 = array_73.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_75, item_74)?;
                object_75.finish();
            }
        }
        array_73.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_76) = &input.secret_id {
        object.key("SecretId").string(var_76.as_str());
    }
    if let Some(var_77) = &input.tag_keys {
        let mut array_78 = object.key("TagKeys").start_array();
        for item_79 in var_77 {
            {
                array_78.value().string(item_79.as_str());
            }
        }
        array_78.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_secret_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSecretInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_80) = &input.secret_id {
        object.key("SecretId").string(var_80.as_str());
    }
    if let Some(var_81) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_81.as_str());
    }
    if let Some(var_82) = &input.description {
        object.key("Description").string(var_82.as_str());
    }
    if let Some(var_83) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_83.as_str());
    }
    if let Some(var_84) = &input.secret_binary {
        object
            .key("SecretBinary")
            .string_unchecked(&aws_smithy_types::base64::encode(var_84));
    }
    if let Some(var_85) = &input.secret_string {
        object.key("SecretString").string(var_85.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_secret_version_stage_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSecretVersionStageInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.secret_id {
        object.key("SecretId").string(var_86.as_str());
    }
    if let Some(var_87) = &input.version_stage {
        object.key("VersionStage").string(var_87.as_str());
    }
    if let Some(var_88) = &input.remove_from_version_id {
        object.key("RemoveFromVersionId").string(var_88.as_str());
    }
    if let Some(var_89) = &input.move_to_version_id {
        object.key("MoveToVersionId").string(var_89.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_validate_resource_policy_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ValidateResourcePolicyInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_90) = &input.secret_id {
        object.key("SecretId").string(var_90.as_str());
    }
    if let Some(var_91) = &input.resource_policy {
        object.key("ResourcePolicy").string(var_91.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_92) = &input.key {
        object.key("Key").string(var_92.as_str());
    }
    if let Some(var_93) = &input.value {
        object.key("Value").string(var_93.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_replica_region_type(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ReplicaRegionType,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_94) = &input.region {
        object.key("Region").string(var_94.as_str());
    }
    if let Some(var_95) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_95.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_96) = &input.key {
        object.key("Key").string(var_96.as_str());
    }
    if let Some(var_97) = &input.values {
        let mut array_98 = object.key("Values").start_array();
        for item_99 in var_97 {
            {
                array_98.value().string(item_99.as_str());
            }
        }
        array_98.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_rotation_rules_type(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::RotationRulesType,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_100) = &input.automatically_after_days {
        object.key("AutomaticallyAfterDays").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_100).into()),
        );
    }
    if let Some(var_101) = &input.duration {
        object.key("Duration").string(var_101.as_str());
    }
    if let Some(var_102) = &input.schedule_expression {
        object.key("ScheduleExpression").string(var_102.as_str());
    }
    Ok(())
}
