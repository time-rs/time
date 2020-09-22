// Modified from the `peeking_take_while` crate. Copyright 2017 Nick Fitzgerald.
// License MIT OR Apache-2.

use std::iter::Peekable;

pub(crate) struct PeekingTakeWhile<'a, I: Iterator, P> {
    iter: &'a mut Peekable<I>,
    pred: P,
}

impl<I: Iterator, P: Fn(&I::Item) -> bool> Iterator for PeekingTakeWhile<'_, I, P> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let pred = &self.pred;
        if self.iter.peek().map_or(false, |v| !(pred)(v)) {
            None
        } else {
            self.iter.next()
        }
    }
}

pub(crate) trait PeekableExt<'a, I: Iterator>: Iterator {
    fn peeking_take_while<P: Fn(&Self::Item) -> bool>(
        &'a mut self,
        pred: P,
    ) -> PeekingTakeWhile<'a, I, P>;
}

impl<'a, I: Iterator> PeekableExt<'a, I> for Peekable<I> {
    fn peeking_take_while<P: Fn(&Self::Item) -> bool>(
        &'a mut self,
        pred: P,
    ) -> PeekingTakeWhile<'_, I, P> {
        PeekingTakeWhile { iter: self, pred }
    }
}
