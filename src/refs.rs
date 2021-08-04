use super::Possible;
use core::pin::Pin;

impl<T> Possible<T> {
    /// Converts from `&Possible<T>` to `Possible<&T>`.
    ///
    /// # Examples
    ///
    /// Converts an `Possible<`[`String`]`>` into an `Possible<`[`usize`]`>`, preserving the original.
    /// The [`map`] method takes the `self` argument by value, consuming the original,
    /// so this technique uses `as_ref` to first take an `Possible` to a reference
    /// to the value inside the original.
    ///
    /// [`map`]: Possible::map
    /// [`String`]: ../../std/string/struct.String.html
    ///
    /// ```
    /// use possible::Possible;
    /// let text: Possible<String> = Possible::Some("Hello, world!".to_string());
    /// // First, cast `Possible<String>` to `Possible<&String>` with `as_ref`,
    /// // then consume *that* with `map`, leaving `text` on the stack.
    /// let text_length: Possible<usize> = text.as_ref().map(|s| s.len());
    /// println!("still can print text: {:?}", text);
    /// ```
    #[inline]
    pub const fn as_ref(&self) -> Possible<&T> {
        match *self {
            Possible::Some(ref x) => Possible::Some(x),
            Possible::None => Possible::None,
            Possible::Void => Possible::Void,
        }
    }

    /// Converts from `&mut Possible<T>` to `Possible<&mut T>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// let mut x = Possible::Some(2);
    /// match x.as_mut() {
    ///     Possible::Some(v) => *v = 42,
    ///     Possible::None | Possible::Void => {},
    /// }
    /// assert_eq!(x, Possible::Some(42));
    /// ```
    #[inline]
    pub fn as_mut(&mut self) -> Possible<&mut T> {
        match *self {
            Possible::Some(ref mut x) => Possible::Some(x),
            Possible::None => Possible::None,
            Possible::Void => Possible::Void,
        }
    }

    /// Converts from [`Pin`]`<&Possible<T>>` to `Possible<`[`Pin`]`<&T>>`.
    #[inline]
    pub fn as_pin_ref(self: Pin<&Self>) -> Possible<Pin<&T>> {
        // SAFETY: `x` is guaranteed to be pinned because it comes from `self`
        // which is pinned.
        unsafe { Pin::get_ref(self).as_ref().map(|x| Pin::new_unchecked(x)) }
    }

    /// Converts from [`Pin`]`<&mut Possible<T>>` to `Possible<`[`Pin`]`<&mut T>>`.
    #[inline]
    pub fn as_pin_mut(self: Pin<&mut Self>) -> Possible<Pin<&mut T>> {
        // SAFETY: `get_unchecked_mut` is never used to move the `Possible` inside `self`.
        // `x` is guaranteed to be pinned because it comes from `self` which is pinned.
        unsafe {
            Pin::get_unchecked_mut(self)
                .as_mut()
                .map(|x| Pin::new_unchecked(x))
        }
    }
}
