// -*- coding: utf-8 -*-
//
// Iterator Peekable with multi-forward-peek and multi-backward-peek
//
// Copyright 2024 Michael Büsch <m@bues.ch>
//
// Licensed under the Apache License version 2.0
// or the MIT license, at your option.
// SPDX-License-Identifier: Apache-2.0 OR MIT
//

//! Iterator Peekable with multi-forward-peek and multi-backward-peek
//!
//! This crate is `#![no_std]`, does not heap-allocate and does not contain `unsafe` code.
//!
//! The [Iterator::Item] must implement [Clone].
//!
//! ```
//! use peekable_fwd_bwd::Peekable;
//! use core::slice::Iter;
//!
//! let array = [10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25];
//!
//! const BWD_SIZE: usize = 2; // size of backward peek buffer.
//! const FWD_SIZE: usize = 8; // size of forward peek buffer.
//!
//! let mut iter = Peekable::<Iter<i32>, BWD_SIZE, FWD_SIZE>::new(&array);
//!
//! assert_eq!(iter.next(), Some(&10));
//! assert_eq!(iter.next(), Some(&11));
//!
//! // forward peek into the future.
//! assert_eq!(iter.peek(), Some(&&12));
//! assert_eq!(iter.peek_nth(0), Some(&&12));
//! assert_eq!(iter.peek_nth(1), Some(&&13));
//! assert_eq!(iter.peek_nth(8), None); // FWD_SIZE too small.
//!
//! assert_eq!(iter.next(), Some(&12));
//!
//! // backward peek into the past.
//! assert_eq!(iter.peek_bwd(), Some(&&12));
//! assert_eq!(iter.peek_bwd_nth(0), Some(&&12));
//! assert_eq!(iter.peek_bwd_nth(1), Some(&&11));
//! assert_eq!(iter.peek_bwd_nth(2), None); // BWD_SIZE too small.
//! ```

#![no_std]
#![forbid(unsafe_code)]

use arraydeque::{ArrayDeque, Wrapping};
use core::iter::Fuse;

/// Iterator Peekable with multi-forward-peek and multi-backward-peek
///
/// Generic parameters:
/// - `I`: An [Iterator]. The [Iterator::Item] must implement [Clone].
/// - `BWD_SIZE`: The size of the backward-peek buffer. In number of elements.
/// - `FWD_SIZE`: The size of the forward-peek buffer. In number of elements.
pub struct Peekable<I, const BWD_SIZE: usize, const FWD_SIZE: usize>
where
    I: Iterator,
    I::Item: Clone,
{
    iter: Fuse<I>,
    bwd_buf: ArrayDeque<I::Item, BWD_SIZE, Wrapping>,
    fwd_buf: ArrayDeque<I::Item, FWD_SIZE, Wrapping>,
}

impl<I, const BWD_SIZE: usize, const FWD_SIZE: usize> Peekable<I, BWD_SIZE, FWD_SIZE>
where
    I: Iterator,
    I::Item: Clone,
{
    /// Wrap an iterator into a new [Peekable].
    ///
    /// The [Iterator::Item]s must implement [Clone].
    #[inline]
    pub fn new<II>(iter: II) -> Peekable<II::IntoIter, BWD_SIZE, FWD_SIZE>
    where
        II: IntoIterator,
        II::Item: Clone,
    {
        Peekable {
            iter: iter.into_iter().fuse(),
            bwd_buf: ArrayDeque::new(),
            fwd_buf: ArrayDeque::new(),
        }
    }

    /// Peek the previous element that has last been returned from [Self::next].
    ///
    /// This does neiter advance this iterator nor increment any other internal cursor.
    ///
    /// Successive peeks will return the same element.
    /// See [Self::peek_bwd_nth] for peeking more than one element into the past.
    ///
    /// Returns None, if:
    /// - there was no call to [Self::next], yet, or
    /// - the backward peek buffer is too small to hold one element.
    #[inline]
    pub fn peek_bwd(&mut self) -> Option<&I::Item> {
        self.peek_bwd_nth(0)
    }

    /// Peek the next element.
    ///
    /// This does neiter advance this iterator nor increment any other internal cursor.
    ///
    /// Successive peeks will return the same element.
    /// See [Self::peek_fwd_nth] for peeking more than one element into the future.
    ///
    /// Returns None, if the inner iterator is exhausted and there is no next element.
    #[inline]
    pub fn peek_fwd(&mut self) -> Option<&I::Item> {
        self.peek_fwd_nth(0)
    }

    /// Alias for [Self::peek_fwd].
    ///
    /// This function is just here to make this library more compatible with
    /// other Peekable implementations.
    #[inline]
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.peek_fwd()
    }

    /// Peek the previous n-th element that has been returned from [Self::next].
    ///
    /// - 0 -> Returns the element from the last [Self::next] call.
    /// - 1 -> Returns the element from the previous to last [Self::next] call.
    /// - 2 -> etc ...
    /// - etc ...
    ///
    /// This does neiter advance this iterator nor increment any other internal cursor.
    ///
    /// Successive peeks at the same position will return the same element.
    ///
    /// Returns None, if:
    /// - there were not enough calls to [Self::next], yet, or
    /// - the backward peek buffer is too small to hold `i + 1` elements.
    #[inline]
    pub fn peek_bwd_nth(&mut self, i: usize) -> Option<&I::Item> {
        self.bwd_buf.get(i)
    }

    /// Peek the next n-th element.
    ///
    /// This does neiter advance this iterator nor increment any other internal cursor.
    ///
    /// Successive peeks at the same position will return the same element.
    ///
    /// Returns None, if the inner iterator is exhausted and there is no n-th element.
    pub fn peek_fwd_nth(&mut self, i: usize) -> Option<&I::Item> {
        if i < self.fwd_buf.capacity() {
            while self.fwd_buf.len() <= i {
                self.fwd_buf.push_back(self.iter.next()?);
            }
            Some(&self.fwd_buf[i])
        } else {
            None
        }
    }

    /// Alias for [Self::peek_fwd_nth].
    ///
    /// This is function just here to make this library more compatible with
    /// other Peekable implementations.
    #[inline]
    pub fn peek_nth(&mut self, i: usize) -> Option<&I::Item> {
        self.peek_fwd_nth(i)
    }
}

impl<I, const BWD_SIZE: usize, const FWD_SIZE: usize> Iterator for Peekable<I, BWD_SIZE, FWD_SIZE>
where
    I: Iterator,
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.fwd_buf.pop_front().or_else(|| self.iter.next());
        if let Some(item) = &item {
            self.bwd_buf.push_front(item.clone());
        }
        item
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::slice::Iter;

    #[test]
    fn test_next() {
        let a = [1, 2, 3];
        let mut it = Peekable::<Iter<i32>, 4, 4>::new(&a);

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_peek_fwd() {
        let a = [1, 2, 3];
        let mut it = Peekable::<Iter<i32>, 4, 4>::new(&a);

        assert_eq!(it.peek(), Some(&&1));
        assert_eq!(it.peek(), Some(&&1));
        assert_eq!(it.peek_fwd(), Some(&&1));
        assert_eq!(it.peek_fwd_nth(0), Some(&&1));
        assert_eq!(it.peek_fwd_nth(1), Some(&&2));
        assert_eq!(it.peek_fwd_nth(2), Some(&&3));
        assert_eq!(it.peek_fwd_nth(3), None);
        assert_eq!(it.peek_fwd_nth(4), None);

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.peek(), Some(&&2));
        assert_eq!(it.peek_fwd_nth(1), Some(&&3));
    }

    #[test]
    fn test_peek_fwd_lim() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut it = Peekable::<Iter<i32>, 2, 4>::new(&a);

        assert_eq!(it.peek_fwd_nth(0), Some(&&1));
        assert_eq!(it.peek_fwd_nth(1), Some(&&2));
        assert_eq!(it.peek_fwd_nth(2), Some(&&3));
        assert_eq!(it.peek_fwd_nth(3), Some(&&4));
        assert_eq!(it.peek_fwd_nth(4), None);
        assert_eq!(it.peek_fwd_nth(5), None);

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.peek_fwd_nth(0), Some(&&2));
        assert_eq!(it.peek_fwd_nth(1), Some(&&3));
        assert_eq!(it.peek_fwd_nth(2), Some(&&4));
        assert_eq!(it.peek_fwd_nth(3), Some(&&5));
        assert_eq!(it.peek_fwd_nth(4), None);
        assert_eq!(it.peek_fwd_nth(5), None);
    }

    #[test]
    fn test_peek_bwd() {
        let a = [1, 2, 3];
        let mut it = Peekable::<Iter<i32>, 4, 4>::new(&a);

        assert_eq!(it.peek_bwd(), None);
        assert_eq!(it.peek_bwd_nth(0), None);
        assert_eq!(it.peek_bwd_nth(1), None);

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.peek_bwd(), Some(&&1));
        assert_eq!(it.peek_bwd_nth(1), None);
        assert_eq!(it.peek_bwd_nth(2), None);

        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.peek_bwd(), Some(&&2));
        assert_eq!(it.peek_bwd_nth(1), Some(&&1));
        assert_eq!(it.peek_bwd_nth(2), None);
    }

    #[test]
    fn test_peek_bwd_lim() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut it = Peekable::<Iter<i32>, 2, 4>::new(&a);

        assert_eq!(it.next(), Some(&1));
        assert_eq!(it.next(), Some(&2));
        assert_eq!(it.next(), Some(&3));
        assert_eq!(it.peek_bwd_nth(0), Some(&&3));
        assert_eq!(it.peek_bwd_nth(1), Some(&&2));
        assert_eq!(it.peek_bwd_nth(2), None);
        assert_eq!(it.peek_bwd_nth(3), None);
    }
}

// vim: ts=4 sw=4 expandtab
