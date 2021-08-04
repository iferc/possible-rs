use super::Possible;
use core::iter::FromIterator;

impl<T> From<Possible<T>> for Option<T> {
    /// Copies `value` into an `Option::Some`.
    ///
    /// Note that this functionpotentially loses information since `Void` is merged into `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let o: Possible<u8> = Possible::from(67);
    ///
    /// assert_eq!(Possible::Some(67), o);
    /// ```
    fn from(value: Possible<T>) -> Option<T> {
        match value {
            Possible::Some(value) => Some(value),
            Possible::None | Possible::Void => None,
        }
    }
}

impl<T> From<Option<T>> for Possible<T> {
    /// Copies `value` into a `Possible::Some`.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let o: Possible<u8> = Possible::from(67);
    ///
    /// assert_eq!(Possible::Some(67), o);
    /// ```
    fn from(value: Option<T>) -> Possible<T> {
        match value {
            Some(value) => Possible::Some(value),
            None => Possible::None,
        }
    }
}

impl<T> From<T> for Possible<T> {
    /// Copies `value` into a new `Possible::Some`.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let o: Possible<u8> = Possible::from(67);
    ///
    /// assert_eq!(Possible::Some(67), o);
    /// ```
    fn from(value: T) -> Possible<T> {
        Possible::Some(value)
    }
}

impl<'a, T> From<&'a Possible<T>> for Possible<&'a T> {
    /// Converts from `&Possible<T>` to `Possible<&T>`.
    ///
    /// # Examples
    ///
    /// Converts a `Possible<`[`String`]`>` into a `Possible<`[`usize`]`>`, preserving the original.
    /// The [`map`] method takes the `self` argument by value, consuming the original,
    /// so this technique uses `from` to first take an `Possible` to a reference
    /// to the value inside the original.
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let s: Possible<String> = Possible::Some(String::from("Hello, Rustaceans!"));
    /// let o: Possible<usize> = Possible::from(&s).map(|ss: &String| ss.len());
    ///
    /// println!("Can still print s: {:?}", s);
    ///
    /// assert_eq!(o, Possible::Some(18));
    /// ```
    fn from(o: &'a Possible<T>) -> Possible<&'a T> {
        o.as_ref()
    }
}

impl<'a, T> From<&'a mut Possible<T>> for Possible<&'a mut T> {
    /// Converts from `&mut Possible<T>` to `Possible<&mut T>`
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut s = Possible::Some(String::from("Hello"));
    /// let o: Possible<&mut String> = Possible::from(&mut s);
    ///
    /// match o {
    ///     Possible::Some(t) => *t = String::from("Hello, Rustaceans!"),
    ///     Possible::None | Possible::Void => (),
    /// }
    ///
    /// assert_eq!(s, Possible::Some(String::from("Hello, Rustaceans!")));
    /// ```
    fn from(o: &'a mut Possible<T>) -> Possible<&'a mut T> {
        o.as_mut()
    }
}

impl<A, V: FromIterator<A>> FromIterator<Possible<A>> for Possible<V> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Possible<A>>>(iter: I) -> Possible<V> {
        match iter
            .into_iter()
            .map(|x| x.ok_or(()))
            .collect::<Result<_, _>>()
            .ok()
        {
            Some(v) => Possible::Some(v),
            None => Possible::None,
        }
    }
}

impl<A, V: FromIterator<A>> FromIterator<Option<A>> for Possible<V> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Option<A>>>(iter: I) -> Possible<V> {
        match iter
            .into_iter()
            .map(|x| x.ok_or(()))
            .collect::<Result<_, _>>()
            .ok()
        {
            Some(v) => Possible::Some(v),
            None => Possible::None,
        }
    }
}
