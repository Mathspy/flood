use futures_core::stream::Stream;
use futures_util::stream::{iter, Iter};

/// Conversion into an [`Stream`].
///
/// By implementing IntoStream for a type, you define how it will be converted to a [`Stream`].\
/// This is the [`Stream`] trait parallel to [`IntoIterator`] for [`Iterator`](std::iter::Iterator)s.
///
/// A note of caution is to not implement generic methods for this like you'd for [`IntoIterator`] because without [specialization](https://github.com/rust-lang/rust/issues/31844) it's not currently possible to blanket implement this for all [`Stream`]s
///
/// [`Stream`]: futures_core::stream::Stream
/// [`IntoIterator`]: std::iter::IntoIterator
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

impl<T> IntoStream for T where T: IntoIterator {
    type Item = T::Item;
    type IntoStream = Iter<T::IntoIter>;

    fn into_stream(self) -> Self::IntoStream {
        iter(self)
    }
}

// TODO: Specialization
// impl<St: Stream> IntoStream for St {
//     type Item = St::Item;
//     type IntoStream = St;

//     fn into_stream(self) -> Self::IntoStream {
//         self
//     }
// }

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

    // I am not putting this test here to avoid breaking crater runs.
    // #[tokio::test]
    // async fn into_stream_array() {
    //     let arr = [1, 2, 3];
    //     let mut stream = arr.into_stream();

    //     assert_eq!(Some(1), stream.next().await);
    //     assert_eq!(Some(2), stream.next().await);
    //     assert_eq!(Some(3), stream.next().await);
    //     assert_eq!(None, stream.next().await);
    // }

    #[tokio::test]
    async fn into_stream_slice() {
        let v = vec![1, 2, 3];
        let slice = &v[..];
        let mut stream = slice.into_stream();

        assert_eq!(Some(&1), stream.next().await);
        assert_eq!(Some(&2), stream.next().await);
        assert_eq!(Some(&3), stream.next().await);
        assert_eq!(None, stream.next().await);
    }
}
