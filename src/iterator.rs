use std::any::Any;
use std::fmt::Debug;

use crate::{subsets::IteratorProgress, wrapper::IsBarWrapper, IsBar};

#[derive(Debug)]
pub struct ProgressTracker<I, E: Any, B: IsBar, W: IsBarWrapper<Bar = B, Error = E>> {
    iterator: I,
    items_count: usize,
    manual_hint: Option<usize>,
    bar: W,
}

impl<I, E: Any, B: IsBar, W: IsBarWrapper<Bar = B, Error = E>> ProgressTracker<I, E, B, W> {
    pub(crate) fn new(iter: I, bar: W) -> Self {
        Self {
            iterator: iter,
            items_count: 0,
            manual_hint: None,
            bar,
        }
    }

    /// Applies a manual size hint to the progress bar tracker, to fix broken progress bars
    pub fn manual_hint(&mut self, hint: usize) -> &mut Self {
        self.manual_hint = Some(hint);
        self
    }
}

impl<I, E, B, W, Ir> Iterator for ProgressTracker<I, E, B, W>
where
    I: Iterator<Item = Ir>,
    E: Any + Debug,
    B: IsBar + IteratorProgress,
    W: IsBarWrapper<Bar = B, Error = E>,
{
    type Item = Ir;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iterator.next();
        let (lower, upper) = self.iterator.size_hint();

        self.bar.bar().set_progress(self.items_count);
        self.bar
            .bar()
            .set_size_hint(if let Some(hint) = self.manual_hint {
                hint
            } else {
                std::cmp::max(
                    self.items_count,
                    self.items_count + std::cmp::max(lower, upper.unwrap_or(0)),
                )
            });

        if let Some(i) = next {
            self.items_count += 1;
            Some(i)
        } else {
            None
        }
    }
}

pub trait ProgressTrackingAdaptor<T>: Iterator<Item = T> + Sized {
    /// Takes controll of a progress bar (or finishes a builder),
    /// displaying the iterators progress
    ///
    /// currently VERY experimental, and WILL break, mainly with iterators that do not have a good [`size_hint`] function
    /// 
    /// [`size_hint`]: std::iter::Iterator::size_hint
    fn display_bar<
        'bar,
        B: 'bar + IsBar + crate::subsets::IteratorProgress,
        E: Any,
        W: IsBarWrapper<Bar = B, Error = E>,
    >(
        self,
        bar: W,
    ) -> ProgressTracker<Self, E, B, W> {
        ProgressTracker::new(self, bar)
    }
}

impl<T, I: Iterator<Item = T>> ProgressTrackingAdaptor<T> for I {}
