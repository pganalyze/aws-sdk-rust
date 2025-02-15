// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cartesian_coordinates(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CartesianCoordinates,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("x").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.x).into()),
        );
    }
    {
        object.key("y").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((input.y).into()),
        );
    }
    if let Some(var_1) = &input.z {
        object.key("z").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_1).into()),
        );
    }
    Ok(())
}

pub(crate) fn de_cartesian_coordinates<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> Result<Option<crate::types::CartesianCoordinates>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CartesianCoordinatesBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "x" => {
                            builder =
                                builder.set_x(::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()));
                        }
                        "y" => {
                            builder =
                                builder.set_y(::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()));
                        }
                        "z" => {
                            builder =
                                builder.set_z(::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy()));
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(crate::serde_util::cartesian_coordinates_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
