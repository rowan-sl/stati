use crate::{BarWrapper, IsBar};

pub struct ProgressTracker<I, B: IsBar> {
    iterator: I,
    items_count: usize,
    manual_hint: Option<usize>,
    bar: BarWrapper<B>,
}

impl<I, B: IsBar> ProgressTracker<I, B> {
    pub fn new(iter: I, bar: BarWrapper<B>) -> Self {
        Self {
            iterator: iter,
            items_count: 0,
            manual_hint: None,
            bar,
        }
    }

    /// Applies a manual size_hint to the progress bar tracker, to fix broken progress bars
    pub fn manual_hint(&mut self, hint: usize) -> &mut Self {
        self.manual_hint = Some(hint);
        self
    }
}

impl<I, Ir, B> Iterator for ProgressTracker<I, B>
where
    I: Iterator<Item = Ir>,
    B: IsBar<Progress = usize> + crate::bar_subsets::PrecentageBar,
{
    type Item = Ir;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.iterator.next();
        let (lower, upper) = self.iterator.size_hint();
        self.bar.set_progress(
            if next.is_some() {
                if let Some(hint) = self.manual_hint {
                    (100 * self.items_count) / hint
                } else {
                    (100 * self.items_count)
                    / std::cmp::max(
                        self.items_count,
                        self.items_count + std::cmp::max(lower, upper.unwrap_or(0)),
                    )
                }
            } else {
                100
            }
        );
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
    /// currently VERY experimental, and WILL break, mainly with iterators that do not have a good size_hint function
    fn display_bar<'bar, B: 'bar + IsBar + crate::bar_subsets::PrecentageBar>(
        self,
        bar: BarWrapper<B>
    ) -> ProgressTracker<Self, B> {
        ProgressTracker::new(self, bar)
    }
}

impl<T, I: Iterator<Item = T>> ProgressTrackingAdaptor<T> for I {}
