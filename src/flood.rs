use crate::{interleave, Interleave};
use futures_core::stream::Stream;

/// The equivalent of [`itertools::Itertools`]
///
/// Do you love [`itertools::Itertools`] and wish your [`Stream`]s implemented it? Fret no more! Flood to the rescue\
/// This is a bit of WIP (Work In Progress) feel free to [submit PRs](https://github.com/Mathspy/flood/pulls) implementing the rest of the methods. Otherwise they shall be implemented as they are needed
///
/// [`Stream`]: futures_core::stream::Stream
/// [`itertools::Itertools`]: https://docs.rs/itertools/0.9.0/itertools/trait.Itertools.html
pub trait Flood: Stream {
    fn interleave<J>(self, other: J) -> Interleave<Self, J>
    where
        Self: Sized,
        J: Stream<Item = Self::Item>,
    {
        interleave(self, other)
    }
}

impl<St> Flood for St where St: Stream {}

#[cfg(test)]
mod tests {
    // Thanks karroffel for the protip
    use super::Flood as _;
    use futures_util::stream::{iter, StreamExt as _};

    #[tokio::test]
    async fn interleave_method_works() {
        let i = iter(vec![9, 8, 7]);
        let j = iter(vec![1, 2, 3, 4, 5, 6]);
        let mut stream = i.interleave(j);

        assert_eq!(Some(9), stream.next().await);
        assert_eq!(Some(1), stream.next().await);
        assert_eq!(Some(8), stream.next().await);
        assert_eq!(Some(2), stream.next().await);
        assert_eq!(Some(7), stream.next().await);
        assert_eq!(Some(3), stream.next().await);
        assert_eq!(Some(4), stream.next().await);
        assert_eq!(Some(5), stream.next().await);
        assert_eq!(Some(6), stream.next().await);
        assert_eq!(None, stream.next().await);
    }
}
