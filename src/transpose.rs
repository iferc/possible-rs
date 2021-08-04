use super::Possible;

impl<T, E> Possible<Result<T, E>> {
    /// Transposes an `Possible` of a [`Result`] into a [`Result`] of an `Possible`.
    ///
    /// [`Possible::Void`] will be mapped to [`Ok`]`(`[`Possible::Void`]`)`.
    /// [`Possible::None`] will be mapped to [`Ok`]`(`[`Possible::None`]`)`.
    /// [`Some`]`(`[`Ok`]`(_))` and [`Some`]`(`[`Err`]`(_))` will be mapped to
    /// [`Ok`]`(`[`Some`]`(_))` and [`Err`]`(_)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use possible::Possible;
    ///
    /// #[derive(Debug, Eq, PartialEq)]
    /// struct SomeErr;
    ///
    /// let x: Result<Possible<i32>, SomeErr> = Ok(Possible::Some(5));
    /// let y: Possible<Result<i32, SomeErr>> = Possible::Some(Ok(5));
    /// assert_eq!(x, y.transpose());
    /// ```
    #[inline]
    pub fn transpose(self) -> Result<Possible<T>, E> {
        match self {
            Possible::Some(Err(e)) => Err(e),
            Possible::Some(Ok(x)) => Ok(Possible::Some(x)),
            Possible::None => Ok(Possible::None),
            Possible::Void => Ok(Possible::Void),
        }
    }
}
