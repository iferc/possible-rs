use super::Possible;
use core::{hint, mem};

impl<T> Possible<T> {
    /// Takes the value out of the option, leaving a [`Possible::Void`] in its place.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::Some(2);
    /// let y = x.take();
    /// assert_eq!(x, Possible::Void);
    /// assert_eq!(y, Possible::Some(2));
    ///
    /// let mut x: Possible<u32> = Possible::None;
    /// let y = x.take();
    /// assert_eq!(x, Possible::Void);
    /// assert_eq!(y, Possible::None);
    /// ```
    #[inline]
    pub fn take(&mut self) -> Possible<T> {
        mem::take(self)
    }

    /// Inserts `value` into the `Possible` then returns a mutable reference to it.
    ///
    /// If the `Possible` already contains a value, the old value is dropped.
    ///
    /// See also [`Possible::get_or_insert`], which doesn't update the value if
    /// the option already contains [`Possible::Some`].
    ///
    /// # Example
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut opt = Possible::None;
    /// let val = opt.insert(1);
    /// assert_eq!(*val, 1);
    /// assert_eq!(opt.unwrap(), 1);
    /// let val = opt.insert(2);
    /// assert_eq!(*val, 2);
    /// *val = 3;
    /// assert_eq!(opt.unwrap(), 3);
    /// ```
    #[must_use = "if you intended to set a value, consider assignment instead"]
    #[inline]
    pub fn insert(&mut self, value: T) -> &mut T {
        *self = Possible::Some(value);

        match self {
            Possible::Some(v) => v,
            // SAFETY: the code above just filled the option
            Possible::None | Possible::Void => unsafe { hint::unreachable_unchecked() },
        }
    }

    /// Inserts `value` into the `Possible` if it is [`Possible::None`] or
    /// [`Possible::Void`], then returns a mutable reference to the contained value.
    ///
    /// See also [`Possible::insert`], which updates the value even if
    /// the `Possible` already contains [`Some`].
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::None;
    ///
    /// {
    ///     let y: &mut u32 = x.get_or_insert(5);
    ///     assert_eq!(y, &5);
    ///
    ///     *y = 7;
    /// }
    ///
    /// assert_eq!(x, Possible::Some(7));
    /// ```
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::Void;
    ///
    /// {
    ///     let y: &mut u32 = x.get_or_insert(3);
    ///     assert_eq!(y, &3);
    ///
    ///     *y = 4;
    /// }
    ///
    /// assert_eq!(x, Possible::Some(4));
    /// ```
    #[inline]
    pub fn get_or_insert(&mut self, value: T) -> &mut T {
        self.get_or_insert_with(|| value)
    }

    /// Inserts the default value into the option if it is [`Possible::None`] or
    /// or [`Possible::Void`], then returns a mutable reference to the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::None;
    ///
    /// {
    ///     let y: &mut u32 = x.get_or_insert_default();
    ///     assert_eq!(y, &0);
    ///
    ///     *y = 7;
    /// }
    ///
    /// assert_eq!(x, Possible::Some(7));
    /// ```
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::Void;
    ///
    /// {
    ///     let y: &mut u32 = x.get_or_insert_default();
    ///     assert_eq!(y, &0);
    ///
    ///     *y = 4;
    /// }
    ///
    /// assert_eq!(x, Possible::Some(4));
    /// ```
    #[inline]
    pub fn get_or_insert_default(&mut self) -> &mut T
    where
        T: Default,
    {
        self.get_or_insert_with(Default::default)
    }

    /// Inserts a value computed from `f` into the `Possible` if it is [`Possible::None`]
    /// or [`Possible::Void`] then returns a mutable reference to the contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::None;
    ///
    /// {
    ///     let y: &mut u32 = x.get_or_insert_with(|| 5);
    ///     assert_eq!(y, &5);
    ///
    ///     *y = 7;
    /// }
    ///
    /// assert_eq!(x, Possible::Some(7));
    /// ```
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::Void;
    ///
    /// {
    ///     let y: &mut u32 = x.get_or_insert_with(|| 3);
    ///     assert_eq!(y, &3);
    ///
    ///     *y = 4;
    /// }
    ///
    /// assert_eq!(x, Possible::Some(4));
    /// ```
    #[inline]
    pub fn get_or_insert_with<F: FnOnce() -> T>(&mut self, f: F) -> &mut T {
        if let Possible::None | Possible::Void = *self {
            *self = Possible::Some(f());
        }

        match self {
            Possible::Some(v) => v,
            // SAFETY: a `Possible::None` or a `Possible::Void` variant for `self`
            // would have been replaced by a `Some` variant in the code above.
            Possible::None | Possible::Void => unsafe { hint::unreachable_unchecked() },
        }
    }

    /// Replaces the actual value in the option by the value given in parameter,
    /// returning the old value if present,
    /// leaving a [`Some`] in its place without deinitializing either one.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::Some(2);
    /// let old = x.replace(5);
    /// assert_eq!(x, Possible::Some(5));
    /// assert_eq!(old, Possible::Some(2));
    /// ```
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::None;
    /// let old = x.replace(3);
    /// assert_eq!(x, Possible::Some(3));
    /// assert_eq!(old, Possible::None);
    /// ```
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::Void;
    /// let old = x.replace(7);
    /// assert_eq!(x, Possible::Some(7));
    /// assert_eq!(old, Possible::Void);
    /// ```
    #[inline]
    pub fn replace(&mut self, value: T) -> Possible<T> {
        mem::replace(self, Possible::Some(value))
    }
}
