use super::Possible;

// This is a separate function to reduce the code size of functions like Possible.expect()
#[inline(never)]
#[cold]
#[track_caller]
fn expect_failed(msg: &str) -> ! {
    panic!("{}", msg)
}

impl<T> Possible<T> {
    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the value is a [`Possible::None`] or a [`Possible::Void`] with
    /// a custom panic message provided by `msg`.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some("value");
    /// assert_eq!(x.expect("fruits are healthy"), "value");
    /// ```
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x: Possible<&str> = Possible::None;
    /// // x.expect("fruits are healthy"); // panics with `fruits are healthy`
    /// ```
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x: Possible<&str> = Possible::Void;
    /// // x.expect("fruits are healthy"); // panics with `fruits are healthy`
    /// ```
    #[inline]
    #[track_caller]
    pub fn expect(self, msg: &str) -> T {
        match self {
            Possible::Some(val) => val,
            Possible::None | Possible::Void => expect_failed(msg),
        }
    }

    /// Returns the contained [`Some`] value, consuming the `self` value.
    ///
    /// Because this function may panic, its use is generally discouraged.
    /// Instead, prefer to use pattern matching and handle the [`Possible::None`]
    /// and [`Possible::Void`] cases explicitly, or call [`unwrap_or`],
    /// [`unwrap_or_else`], or [`unwrap_or_default`].
    ///
    /// [`unwrap_or`]: Possible::unwrap_or
    /// [`unwrap_or_else`]: Possible::unwrap_or_else
    /// [`unwrap_or_default`]: Possible::unwrap_or_default
    ///
    /// # Panics
    ///
    /// Panics if the self value equals [`Possible::None`] or [`Possible::Void`].
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some("air");
    /// assert_eq!(x.unwrap(), "air");
    /// ```
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x: Possible<&str> = Possible::None;
    /// // assert_eq!(x.unwrap(), "air"); // panics
    /// ```
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x: Possible<&str> = Possible::Void;
    /// // assert_eq!(x.unwrap(), "air"); // panics
    /// ```
    #[inline]
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Possible::Some(value) => value,
            Possible::None => {
                expect_failed("called `Possible::unwrap()` on a `Possible::None` value")
            }
            Possible::Void => {
                expect_failed("called `Possible::unwrap()` on a `Possible::Void` value")
            }
        }
    }

    /// Returns the contained [`Some`] value or a provided default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`unwrap_or_else`]: Possible::unwrap_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// assert_eq!(Possible::Some("car").unwrap_or("bike"), "car");
    /// assert_eq!(Possible::None.unwrap_or("bike"), "bike");
    /// assert_eq!(Possible::Void.unwrap_or("scooter"), "scooter");
    /// ```
    #[inline]
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Possible::Some(x) => x,
            Possible::None | Possible::Void => default,
        }
    }

    /// Returns the contained [`Some`] value or computes it from a closure.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let k = 10;
    /// assert_eq!(Possible::Some(4).unwrap_or_else(|| 2 * k), 4);
    /// assert_eq!(Possible::None.unwrap_or_else(|| 2 * k), 20);
    /// assert_eq!(Possible::Void.unwrap_or_else(|| 3 * k), 30);
    /// ```
    #[inline]
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Possible::Some(x) => x,
            Possible::None | Possible::Void => f(),
        }
    }
}

impl<T: Default> Possible<T> {
    /// Returns the contained [`Some`] value or a default.
    ///
    /// Consumes the `self` argument then, if [`Some`], returns the contained
    /// value, otherwise if [`Possible::None`] or [`Possible::Void`], returns
    /// the [default value] for that type.
    ///
    /// # Examples
    ///
    /// Converts a string to an integer, turning poorly-formed strings
    /// into 0 (the default value for integers). [`parse`] converts
    /// a string to any other type that implements [`FromStr`], returning
    /// [`Possible::None`] on error.
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// #[derive(Debug, PartialEq)]
    /// struct TheUltimateAnswer(u32);
    ///
    /// impl Default for TheUltimateAnswer {
    ///     fn default() -> TheUltimateAnswer { TheUltimateAnswer(42) }
    /// }
    ///
    /// let some: TheUltimateAnswer = Possible::Some(TheUltimateAnswer(4)).unwrap_or_default();
    /// assert_eq!(some, TheUltimateAnswer(4));
    ///
    /// let none: TheUltimateAnswer = Possible::None.unwrap_or_default();
    /// assert_eq!(none, TheUltimateAnswer(42));
    ///
    /// let void: TheUltimateAnswer = Possible::Void.unwrap_or_default();
    /// assert_eq!(void, TheUltimateAnswer(42));
    /// ```
    ///
    /// [default value]: Default::default
    /// [`parse`]: str::parse
    /// [`FromStr`]: crate::str::FromStr
    #[inline]
    pub fn unwrap_or_default(self) -> T {
        match self {
            Possible::Some(x) => x,
            Possible::None | Possible::Void => Default::default(),
        }
    }
}
