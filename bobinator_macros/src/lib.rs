/// Print a tracer to stdout if feature "trace" is enabled.
#[macro_export]
macro_rules! leave_trace {
    ($title:literal | $($arg:tt)+) => {
        #[cfg(feature="trace")]
        println!(
            "\u{1f4ad} {} | {}",
            $title,
            conch::Modifier::colour("Grayscale13").unwrap().wraps(
                &format!($($arg)*)
            )
        );
    };
}

/// Generate an async function, mapped to a GET endpoint to a return struct.
#[macro_export]
macro_rules! map_get_to_struct {

    (
        $func_name:ident,
        $doc:literal,
        $end_point:literal,
        $(($param:ident: $type:ident),)*
        $op:ident() -> $struct:ident
        $(,)?
    ) => {
        use bobinator_models;

        #[doc = $doc]
        pub async fn $func_name(
            conn: &Client,
            $(
                $param: $type,
            )*
        ) -> Result<$struct, bobinator_models::structs::BobinatorError> {

            let resp = conn
                .get(format!($end_point))
                .send()
                .await
                .map_err(|err| bobinator_models::structs::BobinatorError::ClientConnectionError(err))?;

            let de_result = bobinator_models::func::handle_response(resp);
            if let Ok(resp) = de_result {
                let data: $struct = resp
                    .$op()
                    .await
                    .map_err(|de_err| bobinator_models::structs::BobinatorError::DataJSONDecodeError(de_err.to_string()))?;

                Ok(data)
            } else {
                Err(de_result.unwrap_err())
            }
        }
    };
}

/// Generate an async function, mapped to a POST endpoint and a return struct.
#[macro_export]
macro_rules! map_post_to_struct {
    (
        $func_name:ident,
        $doc:literal,
        $end_point:literal,
        $(($param:ident: $type:ident),)*
        $post_struct:ident,
        $op:ident() -> $return_struct:ident
        $(,)?
    ) => {
        use bobinator_models::*;

        #[doc = $doc]
        pub async fn $func_name(
            conn: &Client,
            $(
                $param: $type,
            )*
            data: $post_struct,
        ) -> Result<$return_struct, bobinator_models::structs::BobinatorError> {
            let resp = conn
                .post(format!($end_point))
                .json(&data)
                .send()
                .await
                .map_err(|err| bobinator_models::structs::BobinatorError::ClientConnectionError(err))?;

            let de_result = bobinator_models::func::handle_response(resp);
            if let Ok(resp) = de_result {
                let data: $return_struct = resp
                    .$op()
                    .await
                    .map_err(|de_err| bobinator_models::structs::BobinatorError::DataJSONDecodeError(de_err.to_string()))?;

                Ok(data)
            } else {
                Err(de_result.unwrap_err())
            }
        }
    };
}

/// Generate an async function, mapped to a PUT endpoint and no returns.
#[macro_export]
macro_rules! map_put_to_struct {
    (
        $func_name:ident,
        $doc:literal,
        $end_point:literal,
        $(($param:ident: $type:ident),)*
        $put_struct:ident
        $(,)?
    ) => {
        use bobinator_models::*;

        #[doc = $doc]
        pub async fn $func_name(
            conn: &Client,
            $(
                $param: $type,
            )*
            data: $put_struct,
        ) -> Result<(), bobinator_models::structs::BobinatorError> {
            let resp = conn
                .put(format!($end_point))
                .json(&data)
                .send()
                .await
                .map_err(|err| bobinator_models::structs::BobinatorError::ClientConnectionError(err))?;

                let de_result = bobinator_models::func::handle_response(resp);
                de_result.and(Ok(()))
        }
    };
}
