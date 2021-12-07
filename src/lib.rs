mod unix;

//#[macro_use]
//extern crate napi;
#[macro_use]
extern crate napi_derive;
#[macro_use]
extern crate serde_derive;

/// Custom macro for simpler errors.
/// Returns an error enum with generic failure and a message provided by the string literal
#[macro_export]
macro_rules! err {
    ( $( $msg:expr ),* ) => {
        {
            $(Err(napi::Error::new(napi::Status::GenericFailure, $msg.to_string())))*
        }
    };
}
