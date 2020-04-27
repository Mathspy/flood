#![deny(clippy::all)]
#![allow(clippy::needless_doctest_main)]

//! The missing pieces of [`Stream`]s
//!
//! This is meant to be to [`Stream`]s what [`itertools`] is to [`Iterator`]s.\
//! Currently clearly a WIP. Feel free to open PRs adding methods/functions/macros or even issues requesting them, I will be adding things as I or others need them.
//!
//! [`Stream`]: futures_core::stream::Stream
//! [`itertools`]: https://docs.rs/itertools/
//! [`Iterator`]: std::iter::Iterator

mod adaptors;
mod flood;
mod into_stream;
mod size_hint;

pub use self::flood::Flood;
pub use adaptors::{interleave, Interleave};
pub use into_stream::IntoStream;
