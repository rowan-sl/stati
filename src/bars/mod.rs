//! Simple progress bar implementations.
//! use these or create your own!

pub mod common;

pub mod simple;
pub use simple::SimpleBar;

pub mod spinny;
pub use spinny::SpinniBuilder;

pub mod custom;
pub use custom::CustomBar;
