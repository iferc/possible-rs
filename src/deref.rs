use super::Possible;
use core::ops::{Deref, DerefMut};

impl<T: Deref> Possible<T> {
    /// Converts from `Possible<T>` (or `&Possible<T>`) to `Possible<&T::Target>`.
    ///
    /// Leaves the original `Possible` in-place, creating a new one with a reference
    /// to the original one, additionally coercing the contents via [`Deref`].
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x: Possible<String> = Possible::Some("hey".to_owned());
    /// assert_eq!(x.as_deref(), Possible::Some("hey"));
    ///
    /// let x: Possible<String> = Possible::None;
    /// assert_eq!(x.as_deref(), Possible::None);
    ///
    /// let x: Possible<String> = Possible::Void;
    /// assert_eq!(x.as_deref(), Possible::Void);
    /// ```
    pub fn as_deref(&self) -> Possible<&T::Target> {
        self.as_ref().map(|t| t.deref())
    }
}

impl<T: DerefMut> Possible<T> {
    /// Converts from `Possible<T>` (or `&mut Possible<T>`) to `Possible<&mut T::Target>`.
    ///
    /// Leaves the original `Possible` in-place, creating a new one containing a mutable reference to
    /// the inner type's `Deref::Target` type.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x: Possible<String> = Possible::Some("hey".to_owned());
    /// assert_eq!(x.as_deref_mut().map(|x| {
    ///     x.make_ascii_uppercase();
    ///     x
    /// }), Possible::Some("HEY".to_owned().as_mut_str()));
    /// ```
    pub fn as_deref_mut(&mut self) -> Possible<&mut T::Target> {
        self.as_mut().map(|t| t.deref_mut())
    }
}
