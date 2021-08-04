use super::Possible;

impl<T> Default for Possible<T> {
    fn default() -> Possible<T> {
        Possible::Void
    }
}
