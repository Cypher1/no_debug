use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Deref;
use std::ops::DerefMut;

pub struct NoDebug<T>(pub T);

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

pub struct NoDisplay<T>(pub T);

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
        let result = NoDebug(3);
        assert_eq!(format!("{:?}", result), "<no debug: i32>")
    }

    #[test]
    fn dereferences_nodebug() {
        let result = NoDebug(3);
        assert_eq!(format!("{:?}", result), "<no debug: i32>");
        assert_eq!(format!("{:?}", *result), "3");
    }

    #[test]
    fn mut_dereferences_nodebug() {
        let mut result = NoDebug(3);
        *result = 4;
        assert_eq!(format!("{:?}", result), "<no debug: i32>");
        assert_eq!(format!("{:?}", *result), "4");
    }

    #[test]
    fn cannot_display_nodisplay() {
        let result = NoDisplay(3);
        assert_eq!(format!("{}", result), "<no display: i32>")
    }

    #[test]
    fn dereferences_nodisplay() {
        let result = NoDisplay(3);
        assert_eq!(format!("{}", result), "<no display: i32>");
        assert_eq!(format!("{}", *result), "3");
    }

    #[test]
    fn mut_dereferences_nodisplay() {
        let mut result = NoDisplay(3);
        *result = 4;
        assert_eq!(format!("{}", result), "<no display: i32>");
        assert_eq!(format!("{}", *result), "4");
    }
}
