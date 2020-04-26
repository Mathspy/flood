use futures_core::stream::Stream;
use futures_util::stream::{iter, Iter};

/// Conversion into an [`Stream`].
///
/// By implementing IntoStream for a type, you define how it will be converted to a [`Stream`].\
/// This is the [`Stream`] trait parallel to [`IntoIterator`](std::iter::IntoIterator) for [`Iterator`](std::iter::Iterator)s.
///
/// [`Stream`]: futures_core::stream::Stream
pub trait IntoStream {
    /// The type of the elements being streamed.
    type Item;

    /// Which kind of stream are we turning this into?
    type IntoStream: Stream<Item = Self::Item>;

    /// Creates a stream from a value.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use flood::IntoStream;
    /// use futures_util::stream::StreamExt;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let v = vec![1, 2, 3];
    ///     let mut stream = v.into_stream();
    ///
    ///     assert_eq!(Some(1), stream.next().await);
    ///     assert_eq!(Some(2), stream.next().await);
    ///     assert_eq!(Some(3), stream.next().await);
    ///     assert_eq!(None, stream.next().await);
    /// }
    /// ```
    fn into_stream(self) -> Self::IntoStream;
}

impl<T> IntoStream for Vec<T> {
    type Item = T;
    type IntoStream = Iter<<Self as IntoIterator>::IntoIter>;

    fn into_stream(self) -> Self::IntoStream {
        iter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::IntoStream;
    // Thanks karroffel for the protip
    use futures_util::stream::StreamExt as _;

    #[tokio::test]
    async fn into_stream_vec() {
        let v = vec![1, 2, 3];
        let mut stream = v.into_stream();

        assert_eq!(Some(1), stream.next().await);
        assert_eq!(Some(2), stream.next().await);
        assert_eq!(Some(3), stream.next().await);
        assert_eq!(None, stream.next().await);
    }
}