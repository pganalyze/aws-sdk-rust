// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_input_get_personalized_ranking_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetPersonalizedRankingInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.campaign_arn {
        object.key("campaignArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.context {
        #[allow(unused_mut)]
        let mut object_3 = object.key("context").start_object();
        for (key_4, value_5) in var_2 {
            {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.filter_arn {
        object.key("filterArn").string(var_6.as_str());
    }
    if let Some(var_7) = &input.filter_values {
        #[allow(unused_mut)]
        let mut object_8 = object.key("filterValues").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.input_list {
        let mut array_12 = object.key("inputList").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.user_id {
        object.key("userId").string(var_14.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_input_get_recommendations_input(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::input::GetRecommendationsInput,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_15) = &input.campaign_arn {
        object.key("campaignArn").string(var_15.as_str());
    }
    if let Some(var_16) = &input.context {
        #[allow(unused_mut)]
        let mut object_17 = object.key("context").start_object();
        for (key_18, value_19) in var_16 {
            {
                object_17.key(key_18.as_str()).string(value_19.as_str());
            }
        }
        object_17.finish();
    }
    if let Some(var_20) = &input.filter_arn {
        object.key("filterArn").string(var_20.as_str());
    }
    if let Some(var_21) = &input.filter_values {
        #[allow(unused_mut)]
        let mut object_22 = object.key("filterValues").start_object();
        for (key_23, value_24) in var_21 {
            {
                object_22.key(key_23.as_str()).string(value_24.as_str());
            }
        }
        object_22.finish();
    }
    if let Some(var_25) = &input.item_id {
        object.key("itemId").string(var_25.as_str());
    }
    if input.num_results != 0 {
        object.key("numResults").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.num_results).into()),
        );
    }
    if let Some(var_26) = &input.promotions {
        let mut array_27 = object.key("promotions").start_array();
        for item_28 in var_26 {
            {
                #[allow(unused_mut)]
                let mut object_29 = array_27.value().start_object();
                crate::json_ser::serialize_structure_crate_model_promotion(
                    &mut object_29,
                    item_28,
                )?;
                object_29.finish();
            }
        }
        array_27.finish();
    }
    if let Some(var_30) = &input.recommender_arn {
        object.key("recommenderArn").string(var_30.as_str());
    }
    if let Some(var_31) = &input.user_id {
        object.key("userId").string(var_31.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_promotion(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::Promotion,
) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_32) = &input.name {
        object.key("name").string(var_32.as_str());
    }
    if input.percent_promoted_items != 0 {
        object.key("percentPromotedItems").number(
            #[allow(clippy::useless_conversion)]
            aws_smithy_types::Number::NegInt((input.percent_promoted_items).into()),
        );
    }
    if let Some(var_33) = &input.filter_arn {
        object.key("filterArn").string(var_33.as_str());
    }
    if let Some(var_34) = &input.filter_values {
        #[allow(unused_mut)]
        let mut object_35 = object.key("filterValues").start_object();
        for (key_36, value_37) in var_34 {
            {
                object_35.key(key_36.as_str()).string(value_37.as_str());
            }
        }
        object_35.finish();
    }
    Ok(())
}
