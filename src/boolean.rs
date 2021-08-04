use super::Possible;

impl<T> Possible<T> {
    /// Returns the given parameter so long as self contains a wrapped `Some` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some(2);
    /// let y: Possible<&str> = Possible::None;
    /// assert_eq!(x.and(y), Possible::None);
    ///
    /// let x: Possible<u32> = Possible::None;
    /// let y = Possible::Some("foo");
    /// assert_eq!(x.and(y), Possible::None);
    ///
    /// let x = Possible::Some(2);
    /// let y = Possible::Some("foo");
    /// assert_eq!(x.and(y), Possible::Some("foo"));
    ///
    /// let x: Possible<u32> = Possible::None;
    /// let y: Possible<&str> = Possible::None;
    /// assert_eq!(x.and(y), Possible::None);
    ///
    /// let x: Possible<u32> = Possible::Void;
    /// let y = Possible::Some("foo");
    /// assert_eq!(x.and(y), Possible::Void);
    /// ```
    #[inline]
    pub fn and<U>(self, possible_b: Possible<U>) -> Possible<U> {
        match self {
            Possible::Some(_) => possible_b,
            Possible::None => Possible::None,
            Possible::Void => Possible::Void,
        }
    }

    /// Calls the given function with the self contained value and returns the result
    /// unless the wrapped value was a `None` or `Void`.
    ///
    /// Some languages call this operation flatmap.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// fn sq(x: u32) -> Possible<u32> { Possible::Some(x * x) }
    /// fn nope(_: u32) -> Possible<u32> { Possible::None }
    /// fn empty(_: u32) -> Possible<u32> { Possible::Void }
    ///
    /// assert_eq!(Possible::Some(2).and_then(sq).and_then(sq), Possible::Some(16));
    /// assert_eq!(Possible::Some(2).and_then(sq).and_then(nope), Possible::None);
    /// assert_eq!(Possible::Some(2).and_then(sq).and_then(empty), Possible::Void);
    /// assert_eq!(Possible::Some(2).and_then(nope).and_then(sq), Possible::None);
    /// assert_eq!(Possible::Void.and_then(sq).and_then(sq), Possible::Void);
    /// ```
    #[inline]
    pub fn and_then<U, F: FnOnce(T) -> Possible<U>>(self, f: F) -> Possible<U> {
        match self {
            Possible::Some(x) => f(x),
            Possible::None => Possible::None,
            Possible::Void => Possible::Void,
        }
    }

    /// Returns the self wrapped value if it is a `Some` value. Otherwise returns the given parameter.
    ///
    /// Arguments passed to `or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`or_else`], which is
    /// lazily evaluated.
    ///
    /// [`or_else`]: Possible::or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some(2);
    /// let y = Possible::None;
    /// assert_eq!(x.or(y), Possible::Some(2));
    ///
    /// let x = Possible::None;
    /// let y = Possible::Some(100);
    /// assert_eq!(x.or(y), Possible::Some(100));
    ///
    /// let x = Possible::Void;
    /// let y = Possible::Some(100);
    /// assert_eq!(x.or(y), Possible::Some(100));
    ///
    /// let x = Possible::Some(2);
    /// let y = Possible::Some(100);
    /// assert_eq!(x.or(y), Possible::Some(2));
    ///
    /// let x: Possible<u32> = Possible::None;
    /// let y = Possible::None;
    /// assert_eq!(x.or(y), Possible::None);
    ///
    /// let x: Possible<u32> = Possible::None;
    /// let y = Possible::Void;
    /// assert_eq!(x.or(y), Possible::Void);
    /// ```
    #[inline]
    pub fn or(self, possible_b: Possible<T>) -> Possible<T> {
        match self {
            Possible::Some(_) => self,
            Possible::None | Possible::Void => possible_b,
        }
    }

    /// Returns the self wrapped `Some` value, otherwise calls the given function and
    /// returns the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// fn nothing() -> Possible<&'static str> { Possible::Void }
    /// fn nobody() -> Possible<&'static str> { Possible::None }
    /// fn vikings() -> Possible<&'static str> { Possible::Some("vikings") }
    ///
    /// assert_eq!(Possible::Some("barbarians").or_else(vikings), Possible::Some("barbarians"));
    /// assert_eq!(Possible::None.or_else(vikings), Possible::Some("vikings"));
    /// assert_eq!(Possible::None.or_else(nobody), Possible::None);
    /// assert_eq!(Possible::None.or_else(nothing), Possible::Void);
    /// ```
    #[inline]
    pub fn or_else<F: FnOnce() -> Possible<T>>(self, f: F) -> Possible<T> {
        match self {
            Possible::Some(_) => self,
            Possible::None | Possible::Void => f(),
        }
    }
}
