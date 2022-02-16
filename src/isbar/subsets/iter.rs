/// Bars that accept a precentage for the Progress type (represented as usize)
/// bars like this should generaly use 100% (`100usize`) for completed,
///
/// This should be implemented for types that fit these conditions and
/// are ok to use as such
pub trait IteratorProgress {
    /// set items iterated through.
    fn set_progress(&mut self, progress: usize);
    /// set the hint for the maximum size
    fn set_size_hint(&mut self, max: usize);
}
