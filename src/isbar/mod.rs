mod manager_interface;
pub mod subsets;

pub use manager_interface::IsBarManagerInterface;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum BarCloseMethod {
    LeaveBehind,
    Clear,
}

/// Functions that a progress bar should implement,
/// like [`SimpleBar`]
///
/// this does NOT encompass all things a bar needs, and you **should** add more methods
/// for setting progress, initiating the bar, etc.
///
/// [`SimpleBar`]: crate::bars::simple::SimpleBar
pub trait IsBar {
    /// Finishes the [`Bar`], allowing it to be finalized and dropped by the manager.
    /// the bar should not be used after this is called
    ///
    /// [`Bar`]: IsBar
    fn done(&mut self);

    /// Checks if the [`Bar`] is done ([`IsBar::done`] as been called)
    ///
    /// [`Bar`]: IsBar
    fn is_done(&self) -> bool;

    /// Formats the [`Bar`] into a string. this is generaly only used by the [`BarManager`]
    ///
    /// [`Bar`]: IsBar
    /// [`BarManager`]: crate::manager::BarManager
    fn display(&mut self) -> String;

    fn close_method(&self) -> BarCloseMethod;
}
