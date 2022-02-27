/**
Traits for subsets of progress bars, that extend the API to support more features

Current subsets:
 - IteratorProgress: general api for bars that can be used with the iterator wrapper
*/
// ! note: ADD ALL TRAITS HERE TO THE PRELUDE
mod iter;

pub use iter::IteratorProgress;
