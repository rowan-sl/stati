/// simple API for use by the progress bar iterator [`ProgressTracker`]
/// 
/// [`ProgressTracker`]: crate::iterator::ProgressTracker
pub trait IteratorProgress {
    /// Sets the current progress (number of items iterated through)
    fn set_progress(&mut self, progress: usize);

    /// sets a hint for the maximum value [`set_progress`] will ever reach
    /// 
    /// implementations *should* be able to assume that any value passed to [`set_progress`]
    /// is smaller or equal to the latest value passed to this
    /// 
    /// [`set_progress`]: IteratorProgress::set_progress
    fn set_size_hint(&mut self, max: usize);
}
