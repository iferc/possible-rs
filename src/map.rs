use super::Possible;

impl<T> Possible<T> {
    /// Maps a `Possible<T>` to another `Possible<U>` by applying a function to the contained value.
    ///
    /// # Examples
    ///
    /// Converts an `Possible<`[`String`]`>` into an `Possible<`[`usize`]`>`, consuming the original:
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let maybe_some_string = Possible::Some(String::from("Hello, World!"));
    /// // `Possible::map` takes self *by value*, consuming `maybe_some_string`
    /// let maybe_some_len = maybe_some_string.map(|s| s.len());
    ///
    /// assert_eq!(maybe_some_len, Possible::Some(13));
    /// ```
    #[inline]
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Possible<U> {
        match self {
            Possible::Some(x) => Possible::Some(f(x)),
            Possible::None => Possible::None,
            Possible::Void => Possible::Void,
        }
    }

    /// Returns the provided default result (if none or void),
    /// or applies a function to the contained value (if any).
    ///
    /// Arguments passed to `map_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`map_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`map_or_else`]: Possible::map_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some("foo");
    /// assert_eq!(x.map_or(42, |v| v.len()), 3);
    ///
    /// let x: Possible<&str> = Possible::None;
    /// assert_eq!(x.map_or(42, |v| v.len()), 42);
    ///
    /// let x: Possible<&str> = Possible::Void;
    /// assert_eq!(x.map_or(42, |v| v.len()), 42);
    /// ```
    #[inline]
    pub fn map_or<U, F: FnOnce(T) -> U>(self, default: U, f: F) -> U {
        match self {
            Possible::Some(t) => f(t),
            Possible::None | Possible::Void => default,
        }
    }

    /// Computes a default function result (if none or void), or
    /// applies a different function to the contained value (if some).
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let k = 21;
    ///
    /// let x = Possible::Some("foo");
    /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 3);
    ///
    /// let x: Possible<&str> = Possible::None;
    /// assert_eq!(x.map_or_else(|| 2 * k, |v| v.len()), 42);
    /// ```
    #[inline]
    pub fn map_or_else<U, D: FnOnce() -> U, F: FnOnce(T) -> U>(self, default: D, f: F) -> U {
        match self {
            Possible::Some(t) => f(t),
            Possible::None | Possible::Void => default(),
        }
    }

    /// Transforms the `Possible<T>` into a [`Result<T, E>`], mapping [`Possible::Some(v)`] to
    /// [`Ok(v)`], [`Possible::None`] to [`Err(err)`], and [`Possible::Void`] to [`Err(err)`].
    ///
    /// Arguments passed to `ok_or` are eagerly evaluated; if you are passing the
    /// result of a function call, it is recommended to use [`ok_or_else`], which is
    /// lazily evaluated.
    ///
    /// [`Ok(v)`]: Ok
    /// [`Err(err)`]: Err
    /// [`Possible::Some(v)`]: Some
    /// [`ok_or_else`]: Possible::ok_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some("foo");
    /// assert_eq!(x.ok_or(0), Ok("foo"));
    ///
    /// let x: Possible<&str> = Possible::None;
    /// assert_eq!(x.ok_or(0), Err(0));
    ///
    /// let x: Possible<&str> = Possible::Void;
    /// assert_eq!(x.ok_or(1), Err(1));
    /// ```
    #[inline]
    pub fn ok_or<E>(self, err: E) -> Result<T, E> {
        match self {
            Possible::Some(v) => Ok(v),
            Possible::None | Possible::Void => Err(err),
        }
    }

    /// Transforms the `Possible<T>` into a [`Result<T, E>`], mapping [`Possible::Some(v)`] to
    /// [`Ok(v)`], and [`Possible::None`] to [`Err(err())`], [`Possible::Void`] to [`Err(err())`].
    ///
    /// [`Ok(v)`]: Ok
    /// [`Err(err())`]: Err
    /// [`Possible::Some(v)`]: Some
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some("foo");
    /// assert_eq!(x.ok_or_else(|| 0), Ok("foo"));
    ///
    /// let x: Possible<&str> = Possible::None;
    /// assert_eq!(x.ok_or_else(|| 0), Err(0));
    ///
    /// let x: Possible<&str> = Possible::Void;
    /// assert_eq!(x.ok_or_else(|| 1), Err(1));
    /// ```
    #[inline]
    pub fn ok_or_else<E, F: FnOnce() -> E>(self, err: F) -> Result<T, E> {
        match self {
            Possible::Some(v) => Ok(v),
            Possible::None | Possible::Void => Err(err()),
        }
    }
}
