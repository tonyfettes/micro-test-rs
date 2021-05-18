use core::fmt::{Display, Debug, Formatter};
use core::fmt::Result as FmtResult;

#[derive(Copy, Clone, Debug)]
pub struct Metadata {
    pub target: &'static str,
    pub feature: Option<&'static str>,
}

impl Display for Metadata {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self.feature {
            Some(feature) => f.write_fmt(format_args!("{} ({})", self.target, feature)),
            None => f.write_fmt(format_args!("{}", self.target)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Error<'a> {
    pub metadata: Metadata,
    pub cause: core::fmt::Arguments<'a>,
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_fmt(format_args!("{} test failed at '{}'", self.metadata, self.cause))
    }
}

pub type TestResult<'a> = Result<Metadata, Error<'a>>;

//$crate::__private_api_process_result(::core::result::Result::Ok($metadata));

#[macro_export]
macro_rules! micro_assert {
    ($metadata:expr, $test_expr:expr $(,)?) => {
        match $test_expr {
            true => {},
            false => {
                $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                    metadata: $metadata,
                    cause: ::core::format_args!("assertion failed: `{}`", ::core::stringify!($test_expr)),
                }));
                return;
            }
        }
    };
    ($metadata:expr, $test_expr:expr, $($arg:tt)+) => {
        match $test_expr {
            true => {},
            false => {
                $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                    metadata: $metadata,
                    cause: ::core::format_args!($($arg)+),
                }));
                return;
            }
        }
    }
}

#[macro_export]
macro_rules! micro_assert_eq {
    ($metadata:expr, $left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        metadata: $metadata,
                        cause: ::core::format_args!(
                            r#"assertion failed: `(left == right)`
 left: `{:?}`,
right: `{:?}`"#,
                            left_val, right_val
                        ),
                    }));
                    return;
                }
            }
        }
    };
    ($metadata:expr, $left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        metadata: $metadata,
                        cause: ::alloc::format!(
                            r#"assertion failed: `(left == right)`
 left: `{:?}`,
right: `{:?}`: {}"#,
                            left_val, right_val, $($arg)+
                        ),
                    }));
                    return;
                }
            }
        }
    }
}

#[macro_export]
macro_rules! micro_assert_ne {
    ($metadata:expr, $left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        metadata: $metadata,
                        cause: ::core::format_args!(
                            r#"assertion failed: `(left != right)`
 left: `{:?}`,
right: `{:?}`"#,
                            left_val, right_val
                        ),
                    }));
                    return;
                }
            }
        }
    };
    ($metadata:expr, $left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val != *right_val) {
                    $crate::__private_api_process_result(::core::result::Result::Err($crate::Error {
                        metadata: $metadata,
                        cause: ::alloc::format!(
                            r#"assertion failed: `(left != right)`
 left: `{:?}`,
right: `{:?}`: {}"#,
                            left_val, right_val, $($arg)+
                        ),
                    }));
                    return;
                }
            }
        }
    }
}
