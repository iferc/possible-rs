use super::Possible;
use core::iter::FusedIterator;

impl<T> Possible<T> {
    /// Returns an iterator over the possibly contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some(4);
    /// assert_eq!(x.iter().next(), Some(&4));
    ///
    /// let x: Possible<u32> = Possible::None;
    /// assert_eq!(x.iter().next(), None);
    /// ```
    #[inline]
    pub const fn iter(&self) -> Iter<'_, T> {
        Iter {
            inner: Item { opt: self.as_ref() },
        }
    }

    /// Returns a mutable iterator over the possibly contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::Some(4);
    /// match x.iter_mut().next() {
    ///     Some(v) => *v = 42,
    ///     None => {},
    /// }
    /// assert_eq!(x, Possible::Some(42));
    ///
    /// let mut x: Possible<u32> = Possible::None;
    /// assert_eq!(x.iter_mut().next(), None);
    ///
    /// let mut x: Possible<u32> = Possible::Void;
    /// assert_eq!(x.iter_mut().next(), None);
    /// ```
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            inner: Item { opt: self.as_mut() },
        }
    }
}

/// An iterator over a reference to the [`Some`] variant of an [`Possible`].
///
/// The iterator yields one value if the [`Possible`] is a [`Some`], otherwise none.
///
/// This `struct` is created by the [`Possible::iter`] function.
#[derive(Debug)]
pub struct Iter<'a, A: 'a> {
    inner: Item<&'a A>,
}

impl<'a, A> Iterator for Iter<'a, A> {
    type Item = &'a A;

    #[inline]
    fn next(&mut self) -> Option<&'a A> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, A> DoubleEndedIterator for Iter<'a, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a A> {
        self.inner.next_back()
    }
}

impl<A> ExactSizeIterator for Iter<'_, A> {}

impl<A> FusedIterator for Iter<'_, A> {}

impl<A> Clone for Iter<'_, A> {
    #[inline]
    fn clone(&self) -> Self {
        Iter {
            inner: self.inner.clone(),
        }
    }
}

/// An iterator over a mutable reference to the [`Some`] variant of an [`Possible`].
///
/// The iterator yields one value if the [`Possible`] is a [`Some`], otherwise none.
///
/// This `struct` is created by the [`Possible::iter_mut`] function.
#[derive(Debug)]
pub struct IterMut<'a, A: 'a> {
    inner: Item<&'a mut A>,
}

impl<'a, A> Iterator for IterMut<'a, A> {
    type Item = &'a mut A;

    #[inline]
    fn next(&mut self) -> Option<&'a mut A> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<'a, A> DoubleEndedIterator for IterMut<'a, A> {
    #[inline]
    fn next_back(&mut self) -> Option<&'a mut A> {
        self.inner.next_back()
    }
}

impl<A> ExactSizeIterator for IterMut<'_, A> {}

impl<A> FusedIterator for IterMut<'_, A> {}

/// An iterator over the value in [`Some`] variant of an [`Possible`].
///
/// The iterator yields one value if the [`Possible`] is a [`Some`], otherwise none.
///
/// This `struct` is created by the [`Possible::into_iter`] function.
#[derive(Clone, Debug)]
pub struct IntoIter<A> {
    inner: Item<A>,
}

impl<A> Iterator for IntoIter<A> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<A> DoubleEndedIterator for IntoIter<A> {
    #[inline]
    fn next_back(&mut self) -> Option<A> {
        self.inner.next_back()
    }
}

impl<A> ExactSizeIterator for IntoIter<A> {}

impl<A> FusedIterator for IntoIter<A> {}

#[derive(Clone, Debug)]
struct Item<A> {
    opt: Possible<A>,
}

impl<A> Iterator for Item<A> {
    type Item = A;

    #[inline]
    fn next(&mut self) -> Option<A> {
        match self.opt.take() {
            Possible::Some(v) => Some(v),
            Possible::None | Possible::Void => None,
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        match self.opt {
            Possible::Some(_) => (1, Option::Some(1)),
            Possible::None | Possible::Void => (0, Option::Some(0)),
        }
    }
}

impl<A> DoubleEndedIterator for Item<A> {
    #[inline]
    fn next_back(&mut self) -> Option<A> {
        match self.opt.take() {
            Possible::Some(v) => Some(v),
            Possible::None | Possible::Void => None,
        }
    }
}

impl<A> ExactSizeIterator for Item<A> {}

impl<A> FusedIterator for Item<A> {}

impl<T> IntoIterator for Possible<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    /// Returns a consuming iterator over the possibly contained value.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let x = Possible::Some("string");
    /// let v: Vec<&str> = x.into_iter().collect();
    /// assert_eq!(v, ["string"]);
    ///
    /// let x = Possible::None;
    /// let v: Vec<&str> = x.into_iter().collect();
    /// assert!(v.is_empty());
    /// ```
    #[inline]
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            inner: Item { opt: self },
        }
    }
}

impl<'a, T> IntoIterator for &'a Possible<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Possible<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> IterMut<'a, T> {
        self.iter_mut()
    }
}
