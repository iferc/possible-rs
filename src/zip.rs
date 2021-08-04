use super::Possible;

impl<T> Possible<T> {
    /// Zips `self` with another `Possible`.
    ///
    /// If `self` and `other` are both `Possible::Some`, returns `Possible::Some((s, o))`.
    /// If one of `self` or `other` are `Possible::None`, returns `Possible::None`.
    /// Otherwise, `Possible::Void` or  is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some(1);
    /// let y = Possible::Some("hi");
    /// let z = Possible::None::<u8>;
    /// let w = Possible::Void::<&str>;
    ///
    /// assert_eq!(x.zip(y), Possible::Some((1, "hi")));
    /// assert_eq!(x.zip(z), Possible::None);
    /// assert_eq!(y.zip(w), Possible::Void);
    /// assert_eq!(w.zip(y), Possible::Void);
    /// ```
    pub fn zip<U>(self, other: Possible<U>) -> Possible<(T, U)> {
        match (self, other) {
            (Possible::Some(a), Possible::Some(b)) => Possible::Some((a, b)),

            (Possible::Some(_), Possible::None)
            | (Possible::None, Possible::Some(_))
            | (Possible::None, Possible::None) => Possible::None,

            _ => Possible::Void,
        }
    }
}
