mod threaded_wrapper;
mod basic_wrapper;

use std::fmt::Debug;
use std::ops::DerefMut;

use crate::IsBar;

pub use threaded_wrapper::ThreadedBarWrapper;
pub use basic_wrapper::BarWrapper;

#[allow(clippy::module_name_repetitions)]
pub trait IsBarWrapper: crate::sealant::Sealed {
    type Bar: IsBar;
    type Error: Debug;
    // *screams*
    /// Attempts to aqquire the contained bar
    /// 
    /// # Errors
    /// if there is some error aqquiring the bar
    fn try_bar<'b>(&'b mut self)
        -> Result<Box<dyn DerefMut<Target = Self::Bar> + 'b>, Self::Error>;

    /// Gets a reference to the underlying bar for calling functions on it
    ///
    /// # Panics
    /// if aqquiring the bar fails.
    /// 
    /// for a non-panicking version, see [`try_bar`]
    ///
    /// [`try_bar`]: IsBarWrapper::try_bar
    fn bar<'b>(&'b mut self) -> Box<dyn DerefMut<Target = Self::Bar> + 'b> {
        self.try_bar().unwrap()
    }
}
