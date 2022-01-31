#![feature(drain_filter)]

extern crate termion;

pub mod bars;
pub mod macros;
pub mod prelude;
pub(crate) mod wrapper;
pub(crate) mod manager;
pub(crate) mod isbar;
pub(crate) mod utils;

pub use manager::BarManager;
pub use wrapper::BarWrapper;
pub use isbar::IsBar;