use crate::size_hint;
use futures_core::stream::Stream;
use futures_util::stream::{Fuse, StreamExt};
use pin_project::{pin_project, project};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[pin_project]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Interleave<I, J> {
    #[pin]
    a: Fuse<I>,
    #[pin]
    b: Fuse<J>,
    flag: bool,
}

/// Create a stream that interleaves elements in `i` and `j`.
///
/// ```
/// use flood::interleave;
/// use flood::IntoStream;
/// use futures_util::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let i = vec![1, 2];
///     let j = vec![9, 8, 7];
///     let mut stream = interleave(i.into_stream(), j.into_stream());
///
///     assert_eq!(Some(1), stream.next().await);
///     assert_eq!(Some(9), stream.next().await);
///     assert_eq!(Some(2), stream.next().await);
///     assert_eq!(Some(8), stream.next().await);
///     assert_eq!(Some(7), stream.next().await);
///     assert_eq!(None, stream.next().await);
/// }
/// ```
pub fn interleave<I, J>(i: I, j: J) -> Interleave<I, J>
where
    I: Stream,
    J: Stream<Item = I::Item>,
{
    Interleave {
        a: i.fuse(),
        b: j.fuse(),
        flag: false,
    }
}

impl<I, J> Stream for Interleave<I, J>
where
    I: Stream,
    J: Stream<Item = I::Item>,
{
    type Item = I::Item;
    #[inline]
    #[project]
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let projection = self.project();
        *projection.flag = !*projection.flag;
        if *projection.flag {
            match projection.a.poll_next(cx) {
                Poll::Ready(None) => projection.b.poll_next(cx),
                r => r,
            }
        } else {
            match projection.b.poll_next(cx) {
                Poll::Ready(None) => projection.a.poll_next(cx),
                r => r,
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        size_hint::add(self.a.size_hint(), self.b.size_hint())
    }
}

#[cfg(test)]
mod tests {
    use super::interleave;
    use futures_core::stream::Stream;
    // Thanks karroffel for the protip
    use futures_util::stream::{iter, StreamExt as _};

    #[tokio::test]
    async fn interleave_adaptor_works() {
        let i = iter(vec![1, 2, 3, 4, 5, 6]);
        let j = iter(vec![9, 8, 7]);
        let mut stream = interleave(i, j);

        assert_eq!(Some(1), stream.next().await);
        assert_eq!(Some(9), stream.next().await);
        assert_eq!(Some(2), stream.next().await);
        assert_eq!(Some(8), stream.next().await);
        assert_eq!(Some(3), stream.next().await);
        assert_eq!(Some(7), stream.next().await);
        assert_eq!(Some(4), stream.next().await);
        assert_eq!(Some(5), stream.next().await);
        assert_eq!(Some(6), stream.next().await);
        assert_eq!(None, stream.next().await);
    }

    #[tokio::test]
    async fn interleave_adaptor_hint() {
        let i = iter(vec![1, 2, 3, 4, 5, 6]);
        let j = iter(vec![9, 8, 7]);
        let stream = interleave(i, j);

        assert_eq!((9, Some(9)), stream.size_hint());
    }
}
