//! Macros for hardware abstraction layer.

/// A macro to generate test cases for expressions that are expected to panic.
///
/// It creates a new test function for each entry.
///
/// # Usage
///
/// `test_should_panic!((test_name, expression, expected_panic_message), ...);`
#[macro_export]
macro_rules! test_should_panic {
    ($(($test_name:ident, $call_expr:expr, $msg:expr)),* $(,)?) => {
        $(
            #[test]
            #[should_panic(expected = $msg)]
            fn $test_name() {
                // This expression is expected to panic.
                $call_expr;
            }
        )*
    };
}
