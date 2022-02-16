//! A library for easy to use and configurable cli progress bars
//!
//! ## Features
//! fairness: enables using `parking_lot`'s `FairMutex`
//! (adds some overhead but may fix some issues?)

extern crate parking_lot;
extern crate terminal_size;

pub mod bars;
pub(crate) mod isbar;
pub(crate) mod iterator;
pub mod macros;
pub(crate) mod manager;
pub mod prelude;
pub(crate) mod sealant;
pub(crate) mod utils;
pub(crate) mod wrapper;

pub use isbar::subsets as bar_subsets;
pub use isbar::BarCloseMethod;
pub use isbar::IsBar;
pub use manager::BarManager;
pub use wrapper::BarWrapper;
pub use wrapper::IsBarWrapper;
pub use wrapper::ThreadedBarWrapper;
