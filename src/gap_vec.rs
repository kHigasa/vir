//! A vector which has gap inside the buf implemented with a growable ring buffer.

use crate::raw_vec::RawVec;

pub struct GapVec<T> {
    buf: RawVec<T>,
    head: usize,
    tail: usize,
    gap_begin: usize,
    gap_end: usize,
}

