// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_batch_update_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::BatchUpdateClusterInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster_names {
        let mut array_2 = object.key("ClusterNames").start_array();
        for item_3 in var_1 {
            {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.service_update {
        #[allow(unused_mut)]
        let mut object_5 = object.key("ServiceUpdate").start_object();
        crate::json_ser::serialize_structure_crate_model_service_update_request(
            &mut object_5,
            var_4,
        )?;
        object_5.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_copy_snapshot_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CopySnapshotInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_6) = &input.source_snapshot_name {
        object.key("SourceSnapshotName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.target_snapshot_name {
        object.key("TargetSnapshotName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.target_bucket {
        object.key("TargetBucket").string(var_8.as_str());
    }
    if let Some(var_9) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        let mut array_11 = object.key("Tags").start_array();
        for item_12 in var_10 {
            {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_acl_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateAclInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_14) = &input.acl_name {
        object.key("ACLName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.user_names {
        let mut array_16 = object.key("UserNames").start_array();
        for item_17 in var_15 {
            {
                array_16.value().string(item_17.as_str());
            }
        }
        array_16.finish();
    }
    if let Some(var_18) = &input.tags {
        let mut array_19 = object.key("Tags").start_array();
        for item_20 in var_18 {
            {
                #[allow(unused_mut)]
                let mut object_21 = array_19.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_21, item_20)?;
                object_21.finish();
            }
        }
        array_19.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateClusterInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_22) = &input.cluster_name {
        object.key("ClusterName").string(var_22.as_str());
    }
    if let Some(var_23) = &input.node_type {
        object.key("NodeType").string(var_23.as_str());
    }
    if let Some(var_24) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_24.as_str());
    }
    if let Some(var_25) = &input.description {
        object.key("Description").string(var_25.as_str());
    }
    if let Some(var_26) = &input.num_shards {
        object.key("NumShards").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_26).into()),
        );
    }
    if let Some(var_27) = &input.num_replicas_per_shard {
        object.key("NumReplicasPerShard").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_27).into()),
        );
    }
    if let Some(var_28) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_28.as_str());
    }
    if let Some(var_29) = &input.security_group_ids {
        let mut array_30 = object.key("SecurityGroupIds").start_array();
        for item_31 in var_29 {
            {
                array_30.value().string(item_31.as_str());
            }
        }
        array_30.finish();
    }
    if let Some(var_32) = &input.maintenance_window {
        object.key("MaintenanceWindow").string(var_32.as_str());
    }
    if let Some(var_33) = &input.port {
        object.key("Port").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_33).into()),
        );
    }
    if let Some(var_34) = &input.sns_topic_arn {
        object.key("SnsTopicArn").string(var_34.as_str());
    }
    if let Some(var_35) = &input.tls_enabled {
        object.key("TLSEnabled").boolean(*var_35);
    }
    if let Some(var_36) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_36.as_str());
    }
    if let Some(var_37) = &input.snapshot_arns {
        let mut array_38 = object.key("SnapshotArns").start_array();
        for item_39 in var_37 {
            {
                array_38.value().string(item_39.as_str());
            }
        }
        array_38.finish();
    }
    if let Some(var_40) = &input.snapshot_name {
        object.key("SnapshotName").string(var_40.as_str());
    }
    if let Some(var_41) = &input.snapshot_retention_limit {
        object.key("SnapshotRetentionLimit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_41).into()),
        );
    }
    if let Some(var_42) = &input.tags {
        let mut array_43 = object.key("Tags").start_array();
        for item_44 in var_42 {
            {
                #[allow(unused_mut)]
                let mut object_45 = array_43.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_45, item_44)?;
                object_45.finish();
            }
        }
        array_43.finish();
    }
    if let Some(var_46) = &input.snapshot_window {
        object.key("SnapshotWindow").string(var_46.as_str());
    }
    if let Some(var_47) = &input.acl_name {
        object.key("ACLName").string(var_47.as_str());
    }
    if let Some(var_48) = &input.engine_version {
        object.key("EngineVersion").string(var_48.as_str());
    }
    if let Some(var_49) = &input.auto_minor_version_upgrade {
        object.key("AutoMinorVersionUpgrade").boolean(*var_49);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_parameter_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateParameterGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_50) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_50.as_str());
    }
    if let Some(var_51) = &input.family {
        object.key("Family").string(var_51.as_str());
    }
    if let Some(var_52) = &input.description {
        object.key("Description").string(var_52.as_str());
    }
    if let Some(var_53) = &input.tags {
        let mut array_54 = object.key("Tags").start_array();
        for item_55 in var_53 {
            {
                #[allow(unused_mut)]
                let mut object_56 = array_54.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_56, item_55)?;
                object_56.finish();
            }
        }
        array_54.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_snapshot_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSnapshotInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_57) = &input.cluster_name {
        object.key("ClusterName").string(var_57.as_str());
    }
    if let Some(var_58) = &input.snapshot_name {
        object.key("SnapshotName").string(var_58.as_str());
    }
    if let Some(var_59) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_59.as_str());
    }
    if let Some(var_60) = &input.tags {
        let mut array_61 = object.key("Tags").start_array();
        for item_62 in var_60 {
            {
                #[allow(unused_mut)]
                let mut object_63 = array_61.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_63, item_62)?;
                object_63.finish();
            }
        }
        array_61.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_subnet_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateSubnetGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_64) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_64.as_str());
    }
    if let Some(var_65) = &input.description {
        object.key("Description").string(var_65.as_str());
    }
    if let Some(var_66) = &input.subnet_ids {
        let mut array_67 = object.key("SubnetIds").start_array();
        for item_68 in var_66 {
            {
                array_67.value().string(item_68.as_str());
            }
        }
        array_67.finish();
    }
    if let Some(var_69) = &input.tags {
        let mut array_70 = object.key("Tags").start_array();
        for item_71 in var_69 {
            {
                #[allow(unused_mut)]
                let mut object_72 = array_70.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_72, item_71)?;
                object_72.finish();
            }
        }
        array_70.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_create_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::CreateUserInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_73) = &input.user_name {
        object.key("UserName").string(var_73.as_str());
    }
    if let Some(var_74) = &input.authentication_mode {
        #[allow(unused_mut)]
        let mut object_75 = object.key("AuthenticationMode").start_object();
        crate::json_ser::serialize_structure_crate_model_authentication_mode(
            &mut object_75,
            var_74,
        )?;
        object_75.finish();
    }
    if let Some(var_76) = &input.access_string {
        object.key("AccessString").string(var_76.as_str());
    }
    if let Some(var_77) = &input.tags {
        let mut array_78 = object.key("Tags").start_array();
        for item_79 in var_77 {
            {
                #[allow(unused_mut)]
                let mut object_80 = array_78.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_80, item_79)?;
                object_80.finish();
            }
        }
        array_78.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_acl_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteAclInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_81) = &input.acl_name {
        object.key("ACLName").string(var_81.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteClusterInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_82) = &input.cluster_name {
        object.key("ClusterName").string(var_82.as_str());
    }
    if let Some(var_83) = &input.final_snapshot_name {
        object.key("FinalSnapshotName").string(var_83.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_parameter_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteParameterGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_84) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_84.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_snapshot_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSnapshotInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_85) = &input.snapshot_name {
        object.key("SnapshotName").string(var_85.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_subnet_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteSubnetGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_86) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_86.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_delete_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DeleteUserInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_87) = &input.user_name {
        object.key("UserName").string(var_87.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_ac_ls_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeAcLsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_88) = &input.acl_name {
        object.key("ACLName").string(var_88.as_str());
    }
    if let Some(var_89) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_89).into()),
        );
    }
    if let Some(var_90) = &input.next_token {
        object.key("NextToken").string(var_90.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_clusters_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeClustersInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_91) = &input.cluster_name {
        object.key("ClusterName").string(var_91.as_str());
    }
    if let Some(var_92) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_92).into()),
        );
    }
    if let Some(var_93) = &input.next_token {
        object.key("NextToken").string(var_93.as_str());
    }
    if let Some(var_94) = &input.show_shard_details {
        object.key("ShowShardDetails").boolean(*var_94);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_engine_versions_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeEngineVersionsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_95) = &input.engine_version {
        object.key("EngineVersion").string(var_95.as_str());
    }
    if let Some(var_96) = &input.parameter_group_family {
        object.key("ParameterGroupFamily").string(var_96.as_str());
    }
    if let Some(var_97) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_97).into()),
        );
    }
    if let Some(var_98) = &input.next_token {
        object.key("NextToken").string(var_98.as_str());
    }
    if input.default_only {
        object.key("DefaultOnly").boolean(input.default_only);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_events_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeEventsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_99) = &input.source_name {
        object.key("SourceName").string(var_99.as_str());
    }
    if let Some(var_100) = &input.source_type {
        object.key("SourceType").string(var_100.as_str());
    }
    if let Some(var_101) = &input.start_time {
        object
            .key("StartTime")
            .date_time(var_101, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_102) = &input.end_time {
        object
            .key("EndTime")
            .date_time(var_102, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_103) = &input.duration {
        object.key("Duration").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_103).into()),
        );
    }
    if let Some(var_104) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_104).into()),
        );
    }
    if let Some(var_105) = &input.next_token {
        object.key("NextToken").string(var_105.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_parameter_groups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeParameterGroupsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_106) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_106.as_str());
    }
    if let Some(var_107) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_107).into()),
        );
    }
    if let Some(var_108) = &input.next_token {
        object.key("NextToken").string(var_108.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_parameters_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeParametersInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_109) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_109.as_str());
    }
    if let Some(var_110) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_110).into()),
        );
    }
    if let Some(var_111) = &input.next_token {
        object.key("NextToken").string(var_111.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_service_updates_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeServiceUpdatesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_112) = &input.service_update_name {
        object.key("ServiceUpdateName").string(var_112.as_str());
    }
    if let Some(var_113) = &input.cluster_names {
        let mut array_114 = object.key("ClusterNames").start_array();
        for item_115 in var_113 {
            {
                array_114.value().string(item_115.as_str());
            }
        }
        array_114.finish();
    }
    if let Some(var_116) = &input.status {
        let mut array_117 = object.key("Status").start_array();
        for item_118 in var_116 {
            {
                array_117.value().string(item_118.as_str());
            }
        }
        array_117.finish();
    }
    if let Some(var_119) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_119).into()),
        );
    }
    if let Some(var_120) = &input.next_token {
        object.key("NextToken").string(var_120.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_snapshots_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSnapshotsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_121) = &input.cluster_name {
        object.key("ClusterName").string(var_121.as_str());
    }
    if let Some(var_122) = &input.snapshot_name {
        object.key("SnapshotName").string(var_122.as_str());
    }
    if let Some(var_123) = &input.source {
        object.key("Source").string(var_123.as_str());
    }
    if let Some(var_124) = &input.next_token {
        object.key("NextToken").string(var_124.as_str());
    }
    if let Some(var_125) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_125).into()),
        );
    }
    if let Some(var_126) = &input.show_detail {
        object.key("ShowDetail").boolean(*var_126);
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_subnet_groups_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeSubnetGroupsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_127) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_127.as_str());
    }
    if let Some(var_128) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_128).into()),
        );
    }
    if let Some(var_129) = &input.next_token {
        object.key("NextToken").string(var_129.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_describe_users_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::DescribeUsersInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_130) = &input.user_name {
        object.key("UserName").string(var_130.as_str());
    }
    if let Some(var_131) = &input.filters {
        let mut array_132 = object.key("Filters").start_array();
        for item_133 in var_131 {
            {
                #[allow(unused_mut)]
                let mut object_134 = array_132.value().start_object();
                crate::json_ser::serialize_structure_crate_model_filter(&mut object_134, item_133)?;
                object_134.finish();
            }
        }
        array_132.finish();
    }
    if let Some(var_135) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_135).into()),
        );
    }
    if let Some(var_136) = &input.next_token {
        object.key("NextToken").string(var_136.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_failover_shard_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::FailoverShardInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_137) = &input.cluster_name {
        object.key("ClusterName").string(var_137.as_str());
    }
    if let Some(var_138) = &input.shard_name {
        object.key("ShardName").string(var_138.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_allowed_node_type_updates_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListAllowedNodeTypeUpdatesInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_139) = &input.cluster_name {
        object.key("ClusterName").string(var_139.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_list_tags_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ListTagsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_140) = &input.resource_arn {
        object.key("ResourceArn").string(var_140.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_reset_parameter_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::ResetParameterGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_141) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_141.as_str());
    }
    if input.all_parameters {
        object.key("AllParameters").boolean(input.all_parameters);
    }
    if let Some(var_142) = &input.parameter_names {
        let mut array_143 = object.key("ParameterNames").start_array();
        for item_144 in var_142 {
            {
                array_143.value().string(item_144.as_str());
            }
        }
        array_143.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_tag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::TagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_145) = &input.resource_arn {
        object.key("ResourceArn").string(var_145.as_str());
    }
    if let Some(var_146) = &input.tags {
        let mut array_147 = object.key("Tags").start_array();
        for item_148 in var_146 {
            {
                #[allow(unused_mut)]
                let mut object_149 = array_147.value().start_object();
                crate::json_ser::serialize_structure_crate_model_tag(&mut object_149, item_148)?;
                object_149.finish();
            }
        }
        array_147.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_untag_resource_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UntagResourceInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_150) = &input.resource_arn {
        object.key("ResourceArn").string(var_150.as_str());
    }
    if let Some(var_151) = &input.tag_keys {
        let mut array_152 = object.key("TagKeys").start_array();
        for item_153 in var_151 {
            {
                array_152.value().string(item_153.as_str());
            }
        }
        array_152.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_acl_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateAclInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_154) = &input.acl_name {
        object.key("ACLName").string(var_154.as_str());
    }
    if let Some(var_155) = &input.user_names_to_add {
        let mut array_156 = object.key("UserNamesToAdd").start_array();
        for item_157 in var_155 {
            {
                array_156.value().string(item_157.as_str());
            }
        }
        array_156.finish();
    }
    if let Some(var_158) = &input.user_names_to_remove {
        let mut array_159 = object.key("UserNamesToRemove").start_array();
        for item_160 in var_158 {
            {
                array_159.value().string(item_160.as_str());
            }
        }
        array_159.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_cluster_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateClusterInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_161) = &input.cluster_name {
        object.key("ClusterName").string(var_161.as_str());
    }
    if let Some(var_162) = &input.description {
        object.key("Description").string(var_162.as_str());
    }
    if let Some(var_163) = &input.security_group_ids {
        let mut array_164 = object.key("SecurityGroupIds").start_array();
        for item_165 in var_163 {
            {
                array_164.value().string(item_165.as_str());
            }
        }
        array_164.finish();
    }
    if let Some(var_166) = &input.maintenance_window {
        object.key("MaintenanceWindow").string(var_166.as_str());
    }
    if let Some(var_167) = &input.sns_topic_arn {
        object.key("SnsTopicArn").string(var_167.as_str());
    }
    if let Some(var_168) = &input.sns_topic_status {
        object.key("SnsTopicStatus").string(var_168.as_str());
    }
    if let Some(var_169) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_169.as_str());
    }
    if let Some(var_170) = &input.snapshot_window {
        object.key("SnapshotWindow").string(var_170.as_str());
    }
    if let Some(var_171) = &input.snapshot_retention_limit {
        object.key("SnapshotRetentionLimit").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((*var_171).into()),
        );
    }
    if let Some(var_172) = &input.node_type {
        object.key("NodeType").string(var_172.as_str());
    }
    if let Some(var_173) = &input.engine_version {
        object.key("EngineVersion").string(var_173.as_str());
    }
    if let Some(var_174) = &input.replica_configuration {
        #[allow(unused_mut)]
        let mut object_175 = object.key("ReplicaConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_replica_configuration_request(
            &mut object_175,
            var_174,
        )?;
        object_175.finish();
    }
    if let Some(var_176) = &input.shard_configuration {
        #[allow(unused_mut)]
        let mut object_177 = object.key("ShardConfiguration").start_object();
        crate::json_ser::serialize_structure_crate_model_shard_configuration_request(
            &mut object_177,
            var_176,
        )?;
        object_177.finish();
    }
    if let Some(var_178) = &input.acl_name {
        object.key("ACLName").string(var_178.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_parameter_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateParameterGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_179) = &input.parameter_group_name {
        object.key("ParameterGroupName").string(var_179.as_str());
    }
    if let Some(var_180) = &input.parameter_name_values {
        let mut array_181 = object.key("ParameterNameValues").start_array();
        for item_182 in var_180 {
            {
                #[allow(unused_mut)]
                let mut object_183 = array_181.value().start_object();
                crate::json_ser::serialize_structure_crate_model_parameter_name_value(
                    &mut object_183,
                    item_182,
                )?;
                object_183.finish();
            }
        }
        array_181.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_subnet_group_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateSubnetGroupInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_184) = &input.subnet_group_name {
        object.key("SubnetGroupName").string(var_184.as_str());
    }
    if let Some(var_185) = &input.description {
        object.key("Description").string(var_185.as_str());
    }
    if let Some(var_186) = &input.subnet_ids {
        let mut array_187 = object.key("SubnetIds").start_array();
        for item_188 in var_186 {
            {
                array_187.value().string(item_188.as_str());
            }
        }
        array_187.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_input_update_user_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::UpdateUserInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_189) = &input.user_name {
        object.key("UserName").string(var_189.as_str());
    }
    if let Some(var_190) = &input.authentication_mode {
        #[allow(unused_mut)]
        let mut object_191 = object.key("AuthenticationMode").start_object();
        crate::json_ser::serialize_structure_crate_model_authentication_mode(
            &mut object_191,
            var_190,
        )?;
        object_191.finish();
    }
    if let Some(var_192) = &input.access_string {
        object.key("AccessString").string(var_192.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_service_update_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ServiceUpdateRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_193) = &input.service_update_name_to_apply {
        object
            .key("ServiceUpdateNameToApply")
            .string(var_193.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_tag(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Tag,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_194) = &input.key {
        object.key("Key").string(var_194.as_str());
    }
    if let Some(var_195) = &input.value {
        object.key("Value").string(var_195.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_authentication_mode(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::AuthenticationMode,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_196) = &input.r#type {
        object.key("Type").string(var_196.as_str());
    }
    if let Some(var_197) = &input.passwords {
        let mut array_198 = object.key("Passwords").start_array();
        for item_199 in var_197 {
            {
                array_198.value().string(item_199.as_str());
            }
        }
        array_198.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_filter(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Filter,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_200) = &input.name {
        object.key("Name").string(var_200.as_str());
    }
    if let Some(var_201) = &input.values {
        let mut array_202 = object.key("Values").start_array();
        for item_203 in var_201 {
            {
                array_202.value().string(item_203.as_str());
            }
        }
        array_202.finish();
    }
    Ok(())
}

pub fn serialize_structure_crate_model_replica_configuration_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ReplicaConfigurationRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.replica_count != 0 {
        object.key("ReplicaCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.replica_count).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_shard_configuration_request(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ShardConfigurationRequest,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.shard_count != 0 {
        object.key("ShardCount").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.shard_count).into()),
        );
    }
    Ok(())
}

pub fn serialize_structure_crate_model_parameter_name_value(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::ParameterNameValue,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_204) = &input.parameter_name {
        object.key("ParameterName").string(var_204.as_str());
    }
    if let Some(var_205) = &input.parameter_value {
        object.key("ParameterValue").string(var_205.as_str());
    }
    Ok(())
}
