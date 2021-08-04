use super::Possible;

impl<T> Possible<T> {
    /// Returns [`Possible::None`] if the option is [`Possible::None`], [`Possible::Void`]
    /// if the option is [`Possible::Void`], otherwise calls `predicate`
    /// with the wrapped value and returns:
    ///
    /// - [`Possible::Some(t)`] if `predicate` returns `true` (where `t` is the wrapped
    ///   value), and
    /// - [`Possible::None`] if `predicate` returns `false`.
    ///
    /// This function works similar to [`Iterator::filter()`]. You can imagine
    /// the `Possible<T>` being an iterator over one or zero elements. `filter()`
    /// lets you decide which elements to keep.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// fn is_even(n: &i32) -> bool {
    ///     n % 2 == 0
    /// }
    ///
    /// assert_eq!(Possible::None.filter(is_even), Possible::None);
    /// assert_eq!(Possible::Void.filter(is_even), Possible::Void);
    /// assert_eq!(Possible::Some(3).filter(is_even), Possible::None);
    /// assert_eq!(Possible::Some(4).filter(is_even), Possible::Some(4));
    /// ```
    ///
    /// [`Possible::Some(t)`]: Some
    #[inline]
    pub fn filter<P: FnOnce(&T) -> bool>(self, predicate: P) -> Self {
        match self {
            Possible::Void => Possible::Void,
            Possible::None => Possible::None,
            Possible::Some(x) if predicate(&x) => Possible::Some(x),
            Possible::Some(_) => Possible::None,
        }
    }
}
