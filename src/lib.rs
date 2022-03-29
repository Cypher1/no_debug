//! Helper types to omit debug info for select values.
//!
//! Provides wrapper structs with default `Debug` impls.
//! This allows you to use the default implementation of `Debug` for large structs while enabling you
//! to:
//! - avoid using `Debug` impls that traverse deeply nested or overly large structures,
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
//! use no_debug::{NoDebug, Hidden};
//!
//! #[derive(Debug)]
//! struct UserInfo {
//!   username: String,
//!   password: NoDebug<String>,
//!   posts: NoDebug<Vec<String>, Hidden>,
//! }
//!
//! let user = UserInfo {
//!     username: "Cypher1".to_string(),
//!     password: "hunter2".to_string().into(),
//!     posts: vec![
//!         "long post 1...".to_string(),
//!         "long post 2...".to_string(),
//!         "long post 3...".to_string(),
//!     ].into()
//! };
//!
//! // The password is hidden by default
//! assert_eq!(
//!     format!("{:?}", user),
//!     r#"UserInfo { username: "Cypher1", password: <no debug: alloc::string::String>, posts: ... }"#
//! );
//! // And when accessed
//! assert_eq!(format!("{:?}", user.password), r#"<no debug: alloc::string::String>"#);
//! // But it can be extracted easily for operating on the data inside, at which point it is
//! // visible again.
//! assert_eq!(format!("{:?}", *user.password), r#""hunter2""#);

use std::fmt::Debug;
use std::ops::{Deref, DerefMut};

pub trait Msg<T> {
    fn fmt(value: &T, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>;
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct WithTypeInfo;

impl <T> Msg<T> for WithTypeInfo {
    fn fmt(_value: &T, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "<no debug: {}>", std::any::type_name::<T>())
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Hidden;

impl <T> Msg<T> for Hidden {
    fn fmt(_value: &T, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "...")
    }
}

/// Wraps a type `T` and provides a `Debug` impl that does not rely on `T` being `Debug`.
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct NoDebug<T, M: Msg<T>=WithTypeInfo>(T, std::marker::PhantomData<M>);

impl<T> NoDebug<T, WithTypeInfo> {
    pub fn new(value: T) -> Self {
        value.into()
    }
}

impl<T, M: Msg<T>> From<T> for NoDebug<T, M> {
    fn from(value: T) -> Self {
        Self(value, std::marker::PhantomData::default())
    }
}

impl<T, M: Msg<T>> Debug for NoDebug<T, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        M::fmt(&self.0, f)
    }
}

impl<T, M: Msg<T>> Deref for NoDebug<T, M> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, M: Msg<T>> DerefMut for NoDebug<T, M> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cannot_debug_nodebug() {
        let value = NoDebug::new(3);
        assert_eq!(format!("{:?}", value), "<no debug: i32>")
    }

    #[test]
    fn cannot_debug_nodebug_via_into() {
        let value: NoDebug<i32> = 3.into();
        assert_eq!(format!("{:?}", value), "<no debug: i32>")
    }

    #[test]
    fn can_show_type_info() {
        let value: NoDebug<i32, WithTypeInfo> = 3.into();
        assert_eq!(format!("{:?}", value), "<no debug: i32>")
    }

    #[test]
    fn can_show_custom_message() {
        let value: NoDebug<i32, Hidden> = 3.into();
        assert_eq!(format!("{:?}", value), "...")
    }

    #[test]
    fn dereferences_nodebug() {
        let value = NoDebug::new(3);
        assert_eq!(format!("{:?}", value), "<no debug: i32>");
        assert_eq!(format!("{:?}", *value), "3");
    }

    #[test]
    fn mut_dereferences_nodebug() {
        let mut value = NoDebug::new(3);
        *value = 4;
        assert_eq!(format!("{:?}", value), "<no debug: i32>");
        assert_eq!(format!("{:?}", *value), "4");
    }

    #[test]
    fn has_eq_with_inner() {
        let value = NoDebug::new(3);
        assert_eq!(*value, 3);
    }

    #[test]
    fn has_eq_with_raw_value_into_no_debug() {
        let value = NoDebug::new(3);
        assert_eq!(value, 3.into());
    }

    #[test]
    fn has_eq_with_another_no_debug() {
        let value = NoDebug::new(3);
        let other = NoDebug::new(3);
        assert_eq!(value, other);
    }
}
