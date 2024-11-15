//! Terminal status handling.
//!
//! Presently provides a Cargo-like visual style. Hopefully in future versions
//! this can be made configurable.
//!
//! # `status_ok!`: Successful status messages
//!
//! ```ignore
//! // Print a Cargo-like justified status to STDOUT
//! status_ok!("Loaded", "app loaded successfully");
//! ```
//!
//! # `status_err!`: Error messages
//!
//! ```ignore
//! // Print an error message
//! status_err!("something bad happened");
//! ```
//!
//! # `status_attr_ok!`: Successful attributes
//!
//! ```ignore
//! // Print an indented attribute to STDOUT
//! status_attr_ok!("good", "yep");
//! ```
//!
//! # `status_attr_error!`: Error attributes
//!
//! ```ignore
//! // Print an error attribute to STDERR
//! status_attr_err!("error", "yep");
//! ```

/// Print a success status message (in green if colors are enabled)
///
/// ```ignore
/// // Print a Cargo-like justified status to STDOUT
/// status_ok!("Loaded", "app loaded successfully");
/// ```
#[macro_export]
macro_rules! status_ok {
    ($status:expr, $msg:expr) => {
        $crate::terminal::tracing::info!("{}: {}", $status, $msg);
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_ok!($status, format!($fmt, $($arg)+));
    };
}

/// Print an informational status message (in cyan if colors are enabled)
///
/// ```ignore
/// // Print a Cargo-like justified status to STDOUT
/// status_info!("Info", "you may care to know about");
/// ```
#[macro_export]
macro_rules! status_info {
    ($status:expr, $msg:expr) => {
        $crate::terminal::tracing::info!("{}: {}", $status, $msg);
    };
    ($status:expr, $fmt:expr, $($arg:tt)+) => {
        //the error comes here
        //cant find tracing in the list of imported crates
        //but i already imported tracing
        $crate::status_ok!($status, format!($fmt, $($arg)+));
    };
}

/// Print a warning status message (in yellow if colors are enabled)
///
/// ```ignore
/// // Print a Cargo-like justified status to STDOUT
/// status_warn!("heads up, there's something you should know");
/// ```
#[macro_export]
macro_rules! status_warn {
    ($msg:expr) => {
        $crate::terminal::tracing::warn!("{}: {}", "warning:", $msg);
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::status_warn!(format!($fmt, $($arg)+));
    };
}

/// Print an error message (in red if colors are enabled)
///
/// ```ignore
/// // Print an error message
/// status_err!("something bad happened");
/// ```
#[macro_export]
macro_rules! status_err {
    ($msg:expr) => {
        $crate::terminal::tracing::error!("{}: {}", "error:", $msg);
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::status_err!(format!($fmt, $($arg)+));
    };
}

/// Print a tab-delimited status attribute (in green if colors are enabled)
///
/// ```ignore
/// // Print an indented attribute to STDOUT
/// status_attr_ok!("good", "yep");
/// ```
#[macro_export]
macro_rules! status_attr_ok {
    ($attr:expr, $msg:expr) => {
        // TODO(tarcieri): hax... use a better format string?
        let attr_delimited = if $attr.len() >= 7 {
            format!("{}:", $attr)
        } else {
            format!("{}:\t", $attr)
        };


        $crate::tracing::info!("{}: {}", attr_delimited, $msg);
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_attr_ok!($attr, format!($fmt, $($arg)+));
    }
}

/// Print a tab-delimited status attribute (in red if colors are enabled)
///
/// ```ignore
/// // Print an error attribute to STDERR
/// status_attr_err!("error", "yep");
/// ```
#[macro_export]
macro_rules! status_attr_err {
    ($attr:expr, $msg:expr) => {
        // TODO(tarcieri): hax... use a better format string?
        let attr_delimited = if $attr.len() >= 7 {
            format!("{}:", $attr)
        } else {
            format!("{}:\t", $attr)
        };


        $crate::tracing::error!("{}: {}", attr_delimited, $msg);
    };
    ($attr: expr, $fmt:expr, $($arg:tt)+) => {
        $crate::status_attr_err!($attr, format!($fmt, $($arg)+));
    }
}
