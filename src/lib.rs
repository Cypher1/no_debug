#![doc = include_str!("../README.md")]

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
