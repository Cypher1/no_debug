//! Helper types to omit debug and display info for select values.
//!
//! Provides wrapper structs with default `Debug` and Display impls.
//! This allows you to use the default implementation of `Debug` for large structs while enabling you
//! to:
//! - avoid using `Debug` impls that construct deeply nested or overly large structures,
//! - avoid using a `Debug` impl that leaks info that should not be logged.
//!
//! This can improve:
//! - readability (logs can focus on the information you care about),
//! - debuggability & security (logs can be more complete without accidentally leaking private
//! info),
//! - and performance (complex data structures don't need to be traversed for debugging unless intentionally requested via `Deref`).
//!
//! Example usage: Hiding a user's password from logs.
//! ```rust
//! use no_debug::NoDebug;
//!
//! #[derive(Debug)]
//! struct UserInfo {
//!   username: String,
//!   password: NoDebug<String>,
//! }
//!
//! let user = UserInfo {
//!     username: "Cypher1".to_string(),
//!     password: NoDebug::new("hunter2".to_string())
//! };
//!
//! // The password is hidden by default
//! assert_eq!(
//!     format!("{:?}", user),
//!     r#"UserInfo { username: "Cypher1", password: <no debug: alloc::string::String> }"#
//! );
//! // Even when accessed
//! assert_eq!(format!("{:?}", user.password), r#"<no debug: alloc::string::String>"#);
//! // But is can be extracted easily for operating on the data inside, at which point it is
//! // visible again.
//! assert_eq!(format!("{:?}", *user.password), r#""hunter2""#);

use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};

/// Wraps a type `T` and provides a `Debug` impl that does not rely on `T` being `Debug`.
pub struct NoDebug<T>(T);

impl<T> NoDebug<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Debug for NoDebug<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "<no debug: {}>", std::any::type_name::<T>())
    }
}

impl<T> Deref for NoDebug<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for NoDebug<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Wraps a type `T` and provides a `Display` impl that does not rely on `T` being `Display`.
///
/// Provided for completeness (to mirror `NoDebug`).
pub struct NoDisplay<T>(T);

impl<T> NoDisplay<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Display for NoDisplay<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "<no display: {}>", std::any::type_name::<T>())
    }
}

impl<T> Deref for NoDisplay<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for NoDisplay<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_debug_nodebug() {
        let result = NoDebug::new(3);
        assert_eq!(format!("{:?}", result), "<no debug: i32>")
    }

    #[test]
    fn dereferences_nodebug() {
        let result = NoDebug::new(3);
        assert_eq!(format!("{:?}", result), "<no debug: i32>");
        assert_eq!(format!("{:?}", *result), "3");
    }

    #[test]
    fn mut_dereferences_nodebug() {
        let mut result = NoDebug::new(3);
        *result = 4;
        assert_eq!(format!("{:?}", result), "<no debug: i32>");
        assert_eq!(format!("{:?}", *result), "4");
    }

    #[test]
    fn cannot_display_nodisplay() {
        let result = NoDisplay::new(3);
        assert_eq!(format!("{}", result), "<no display: i32>")
    }

    #[test]
    fn dereferences_nodisplay() {
        let result = NoDisplay::new(3);
        assert_eq!(format!("{}", result), "<no display: i32>");
        assert_eq!(format!("{}", *result), "3");
    }

    #[test]
    fn mut_dereferences_nodisplay() {
        let mut result = NoDisplay::new(3);
        *result = 4;
        assert_eq!(format!("{}", result), "<no display: i32>");
        assert_eq!(format!("{}", *result), "4");
    }
}
