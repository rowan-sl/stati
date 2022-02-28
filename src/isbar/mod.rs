mod manager_interface;
pub mod subsets;

use std::error;

pub use manager_interface::IsBarManagerInterface;

/// How the bar is handled when it is completed ([`done`] is called)
///
/// [`done`]: IsBar::done
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BarCloseMethod {
    /// Print the bar one last time, then cease tracking it.
    /// The bar will be moved after all currently tracked bars.
    LeaveBehind,
    /// Delete the bar, clearing it from the screen.
    Clear,
}

/// Basic API of a progress bar,
/// providing methods required for all progress bars
///
/// this does NOT encompass all things a bar needs, and you **should** add more methods
/// for setting progress, initiating the bar, etc.
pub trait IsBar {
    /// Finishes the [`Bar`], allowing it to be finalized and dropped by the manager according to
    /// the close method returned by [`close_method`].
    /// The bar should not be used after this is called.
    ///
    /// [`Bar`]: IsBar
    /// [`close_method`]: IsBar::close_method
    fn done(&mut self);

    /// Checks if the [`Bar`] is done ([`IsBar::done`] as been called)
    ///
    /// [`Bar`]: IsBar
    fn is_done(&self) -> bool;

    /// Formats the [`Bar`] into a string. this is generaly only used by the [`BarManager`]
    ///
    /// [`Bar`]: IsBar
    /// [`BarManager`]: crate::manager::BarManager
    fn display(&mut self) -> Result<String, Box<dyn error::Error + Sync>>;

    /// Returns how the bar should be handled by the [`BarManager`] after [`done`] is called
    ///
    /// Returing [`None`] indicates that the [`BarManager`] should chose the default close method
    /// 
    /// this is for internal use
    ///
    /// [`done`]: IsBar::done
    /// [`BarManager`]: crate::manager::BarManager
    fn close_method(&self) -> Option<BarCloseMethod>;
}
