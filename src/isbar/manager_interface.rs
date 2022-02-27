use std::fmt::Debug;

use super::{BarCloseMethod, IsBar};

/// Internal interface for a progress bar, which is held by [`BarManager`]
///
/// All methods here forward to the respective methods on the underlying [`IsBar`] implementation
///
/// [`BarManager`]: crate::manager::BarManager
/// [`IsBar`]: crate::isbar::IsBar
pub trait IsBarManagerInterface: Debug {
    fn display(&mut self) -> String;

    fn is_done(&self) -> bool;

    fn close_method(&self) -> BarCloseMethod;
}

impl<T> IsBarManagerInterface for T
where
    T: IsBar + Debug,
{
    fn display(&mut self) -> String {
        <T as IsBar>::display(self)
    }

    fn is_done(&self) -> bool {
        <T as IsBar>::is_done(self)
    }

    fn close_method(&self) -> BarCloseMethod {
        self.close_method()
    }
}
