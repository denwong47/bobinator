// Deserialise Value::Number.
#[macro_export]
macro_rules! deserialize_num_field {
    ($mapping:expr, $key:literal, $as_type:ident) => {{
        let value =
            $mapping
                .get($key)
                .ok_or(D::Error::custom(BobinatorError::RecordFieldMissing(
                    $key.to_string(),
                )))?;

        if let serde_json::Value::Number(v) = value {
            v.$as_type().ok_or(())
        } else {
            Err(())
        }
        .map_err(|_| {
            D::Error::custom(BobinatorError::RecordFieldInvalid(
                $key.to_string(),
                value.clone(),
            ))
        })
    }};
}

// Deserialise Value::String.
#[macro_export]
macro_rules! deserialize_str_field {
    ($mapping:expr, $key:literal) => {{
        let value =
            $mapping
                .get($key)
                .ok_or(D::Error::custom(BobinatorError::RecordFieldMissing(
                    $key.to_string(),
                )))?;

        if let serde_json::Value::String(v) = value {
            Ok(v.to_owned())
        } else {
            Err(D::Error::custom(BobinatorError::RecordFieldInvalid(
                $key.to_string(),
                value.clone(),
            )))
        }
    }};
}
