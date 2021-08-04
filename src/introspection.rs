use super::Possible;

impl<T> Possible<T> {
    /// Returns `true` if the option is a [`Possible::Some`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x: Possible<u32> = Possible::Some(2);
    /// assert_eq!(x.is_some(), true);
    ///
    /// let x: Possible<u32> = Possible::None;
    /// assert_eq!(x.is_some(), false);
    ///
    /// let x: Possible<u32> = Possible::Void;
    /// assert_eq!(x.is_some(), false);
    /// ```
    #[must_use = "if you intended to assert that this has a value, consider `.unwrap()` instead"]
    #[inline]
    pub fn is_some(&self) -> bool {
        matches!(*self, Possible::Some(_))
    }

    /// Returns `true` if the option is a [`Possible::None`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x: Possible<u32> = Possible::Some(2);
    /// assert_eq!(x.is_none(), false);
    ///
    /// let x: Possible<u32> = Possible::None;
    /// assert_eq!(x.is_none(), true);
    ///
    /// let x: Possible<u32> = Possible::Void;
    /// assert_eq!(x.is_none(), false);
    /// ```
    #[must_use = "if you intended to assert that this doesn't have a value, consider \
                 `.and_then(|_| panic!(\"`Possible` had a value when expected `Possible::None`\"))` instead"]
    #[inline]
    pub fn is_none(&self) -> bool {
        matches!(*self, Possible::None)
    }

    /// Returns `true` if the option is a [`Possible::Void`] value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x: Possible<u32> = Possible::Some(2);
    /// assert_eq!(x.is_void(), false);
    ///
    /// let x: Possible<u32> = Possible::None;
    /// assert_eq!(x.is_void(), false);
    ///
    /// let x: Possible<u32> = Possible::Void;
    /// assert_eq!(x.is_void(), true);
    /// ```
    #[must_use = "if you intended to assert that this doesn't have a value, consider \
                 `.and_then(|_| panic!(\"`Possible` had a value when expected `Possible::Void`\"))` instead"]
    #[inline]
    pub fn is_void(&self) -> bool {
        matches!(*self, Possible::Void)
    }

    /// Returns `true` if the option is a [`Possible::Some`] value containing the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x: Possible<u32> = Possible::Some(2);
    /// assert_eq!(x.contains(&2), true);
    ///
    /// let x: Possible<u32> = Possible::Some(3);
    /// assert_eq!(x.contains(&2), false);
    ///
    /// let x: Possible<u32> = Possible::None;
    /// assert_eq!(x.contains(&2), false);
    ///
    /// let x: Possible<u32> = Possible::Void;
    /// assert_eq!(x.contains(&2), false);
    /// ```
    #[must_use]
    #[inline]
    pub fn contains<U>(&self, x: &U) -> bool
    where
        U: PartialEq<T>,
    {
        match self {
            Possible::Some(y) => x == y,
            Possible::None => false,
            Possible::Void => false,
        }
    }
}
