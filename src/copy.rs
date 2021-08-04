use super::Possible;

impl<T: Copy> Possible<&T> {
    /// Maps an `Possible<&T>` to an `Possible<T>` by copying the contents of the possiblity.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = 12;
    /// let y = Possible::Some(&x);
    /// assert_eq!(y, Possible::Some(&12));
    ///
    /// let copied = y.copied();
    /// assert_eq!(copied, Possible::Some(12));
    /// ```
    pub fn copied(self) -> Possible<T> {
        self.map(|&t| t)
    }
}

impl<T: Copy> Possible<&mut T> {
    /// Maps an `Possible<&mut T>` to an `Possible<T>` by copying the contents of the possiblity.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = 12;
    /// let y = Possible::Some(&mut x);
    /// assert_eq!(y, Possible::Some(&mut 12));
    ///
    /// let copied = y.copied();
    /// assert_eq!(copied, Possible::Some(12));
    /// ```
    pub fn copied(self) -> Possible<T> {
        self.map(|&mut t| t)
    }
}

impl<T: Clone> Possible<&T> {
    /// Maps an `Possible<&T>` to an `Possible<T>` by cloning the contents of the possiblity.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = 12;
    /// let y = Possible::Some(&x);
    /// assert_eq!(y, Possible::Some(&12));
    ///
    /// let cloned = y.cloned();
    /// assert_eq!(cloned, Possible::Some(12));
    /// ```
    pub fn cloned(self) -> Possible<T> {
        self.map(|t| t.clone())
    }
}

impl<T: Clone> Possible<&mut T> {
    /// Maps an `Possible<&mut T>` to an `Possible<T>` by cloning the contents of the possiblity.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = 12;
    /// let y = Possible::Some(&mut x);
    /// assert_eq!(y, Possible::Some(&mut 12));
    ///
    /// let cloned = y.cloned();
    /// assert_eq!(cloned, Possible::Some(12));
    /// ```
    pub fn cloned(self) -> Possible<T> {
        self.map(|t| t.clone())
    }
}

impl<T: Clone> Clone for Possible<T> {
    #[inline]
    fn clone(&self) -> Self {
        match self {
            Possible::Some(x) => Possible::Some(x.clone()),
            Possible::None => Possible::None,
            Possible::Void => Possible::Void,
        }
    }

    #[inline]
    fn clone_from(&mut self, source: &Self) {
        match (self, source) {
            (Possible::Some(to), Possible::Some(from)) => to.clone_from(from),
            (to, from) => *to = from.clone(),
        }
    }
}
