use std::usize;

/// **SizeHint** is the return type of **Stream::size_hint()**.
pub type SizeHint = (usize, Option<usize>);

/// Add **SizeHint** correctly.
#[inline]
pub fn add(a: SizeHint, b: SizeHint) -> SizeHint {
    let min = a.0.saturating_add(b.0);
    let max = match (a.1, b.1) {
        (Some(x), Some(y)) => x.checked_add(y),
        _ => None,
    };

    (min, max)
}
