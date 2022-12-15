// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_operation_crate_operation_create_access_point(
    input: &crate::input::CreateAccessPointInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("CreateAccessPointRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_create_access_point_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_access_point_for_object_lambda(
    input: &crate::input::CreateAccessPointForObjectLambdaInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("CreateAccessPointForObjectLambdaRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_create_access_point_for_object_lambda_input(
            input, root,
        )?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_payload_create_bucket_input(
    payload: &std::option::Option<crate::model::CreateBucketConfiguration>,
) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::operation_ser::rest_xml_unset_payload()),
    };
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3control_synthetic_create_bucket_input_create_bucket_configuration(payload)?
    )
}

pub fn serialize_operation_crate_operation_create_job(
    input: &crate::input::CreateJobInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("CreateJobRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_create_job_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_create_multi_region_access_point(
    input: &crate::input::CreateMultiRegionAccessPointInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("CreateMultiRegionAccessPointRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_create_multi_region_access_point_input(
            input, root,
        )?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_delete_multi_region_access_point(
    input: &crate::input::DeleteMultiRegionAccessPointInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("DeleteMultiRegionAccessPointRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_delete_multi_region_access_point_input(
            input, root,
        )?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_put_access_point_configuration_for_object_lambda(
    input: &crate::input::PutAccessPointConfigurationForObjectLambdaInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("PutAccessPointConfigurationForObjectLambdaRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_put_access_point_configuration_for_object_lambda_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_put_access_point_policy(
    input: &crate::input::PutAccessPointPolicyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("PutAccessPointPolicyRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_put_access_point_policy_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_put_access_point_policy_for_object_lambda(
    input: &crate::input::PutAccessPointPolicyForObjectLambdaInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("PutAccessPointPolicyForObjectLambdaRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_put_access_point_policy_for_object_lambda_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_payload_put_bucket_lifecycle_configuration_input(
    payload: &std::option::Option<crate::model::LifecycleConfiguration>,
) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::operation_ser::rest_xml_unset_payload()),
    };
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3control_synthetic_put_bucket_lifecycle_configuration_input_lifecycle_configuration(payload)?
    )
}

pub fn serialize_operation_crate_operation_put_bucket_policy(
    input: &crate::input::PutBucketPolicyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("PutBucketPolicyRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_put_bucket_policy_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_payload_put_bucket_tagging_input(
    payload: &std::option::Option<crate::model::Tagging>,
) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::operation_ser::rest_xml_unset_payload()),
    };
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3control_synthetic_put_bucket_tagging_input_tagging(payload)?
    )
}

pub fn serialize_payload_put_bucket_versioning_input(
    payload: &std::option::Option<crate::model::VersioningConfiguration>,
) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::operation_ser::rest_xml_unset_payload()),
    };
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3control_synthetic_put_bucket_versioning_input_versioning_configuration(payload)?
    )
}

pub fn serialize_operation_crate_operation_put_job_tagging(
    input: &crate::input::PutJobTaggingInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("PutJobTaggingRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_put_job_tagging_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_put_multi_region_access_point_policy(
    input: &crate::input::PutMultiRegionAccessPointPolicyInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("PutMultiRegionAccessPointPolicyRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_put_multi_region_access_point_policy_input(
            input, root,
        )?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_payload_put_public_access_block_input(
    payload: &std::option::Option<crate::model::PublicAccessBlockConfiguration>,
) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::operation_ser::rest_xml_unset_payload()),
    };
    Ok(
        crate::xml_ser::serialize_member_com_amazonaws_s3control_synthetic_put_public_access_block_input_public_access_block_configuration(payload)?
    )
}

pub fn serialize_operation_crate_operation_put_storage_lens_configuration(
    input: &crate::input::PutStorageLensConfigurationInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("PutStorageLensConfigurationRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_put_storage_lens_configuration_input(
            input, root,
        )?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn serialize_operation_crate_operation_put_storage_lens_configuration_tagging(
    input: &crate::input::PutStorageLensConfigurationTaggingInput,
) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    {
        let mut writer = aws_smithy_xml::encode::XmlWriter::new(&mut out);
        #[allow(unused_mut)]
        let mut root = writer
            .start_el("PutStorageLensConfigurationTaggingRequest")
            .write_ns("http://awss3control.amazonaws.com/doc/2018-08-20/", None);
        crate::xml_ser::serialize_structure_crate_input_put_storage_lens_configuration_tagging_input(input, root)?
    }
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

pub fn rest_xml_unset_payload() -> std::vec::Vec<u8> {
    Vec::new()
}
