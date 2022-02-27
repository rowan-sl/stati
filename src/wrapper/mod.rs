mod basic_wrapper;
mod threaded_wrapper;

use std::fmt::Debug;
use std::ops::DerefMut;

use crate::IsBar;

pub use basic_wrapper::BarWrapper;
pub use threaded_wrapper::ThreadedBarWrapper;

/// General API for a bar wrappers used by [`ThreadedBarWrapper`] and [`BarWrapper`]
#[allow(clippy::module_name_repetitions)]
#[cfg(not(feature = "nightly"))]
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

/// General API for a bar wrappers used by [`ThreadedBarWrapper`] and [`BarWrapper`]
#[allow(clippy::module_name_repetitions)]
#[cfg(feature = "nightly")]
pub trait IsBarWrapper: crate::sealant::Sealed {
    type Bar: IsBar;
    type Error: Debug;
    type BarGuard<'g>: DerefMut<Target = Self::Bar>
    where
        Self: 'g;
    // *screams*
    /// Attempts to aqquire the contained bar
    ///
    /// # Errors
    /// if there is some error aqquiring the bar
    fn try_bar<'g>(&'g mut self) -> Result<Self::BarGuard<'g>, Self::Error>;

    /// Gets a reference to the underlying bar for calling functions on it
    ///
    /// # Panics
    /// if aqquiring the bar fails.
    ///
    /// for a non-panicking version, see [`try_bar`]
    ///
    /// [`try_bar`]: IsBarWrapper::try_bar
    fn bar<'g>(&'g mut self) -> Self::BarGuard<'g> {
        self.try_bar().unwrap()
    }
}
