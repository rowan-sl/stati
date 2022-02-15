mod wrapper;
pub use wrapper::BarWrapper;
mod threaded_wrapper;
pub use threaded_wrapper::ThreadedBarWrapper;

use crate::IsBar;

pub trait IsBarWrapper: crate::sealant::Sealed {
    type Error;
    type Bar: IsBar;
    fn set_progress(&mut self, progress: <<Self as IsBarWrapper>::Bar as IsBar>::Progress) -> Result<(), Self::Error>;

    /// Sets the job name of the bar. for more info, see [`IsBar::set_name`]
    fn set_name(&mut self, job_name: String) -> Result<(), Self::Error>;

    /// Indicates that the bar has finished, and can be finalized and dropped by the manager.
    /// for more info, see [`IsBar::done`]
    ///
    /// this is also called by the [`Drop`] impl on this type
    fn done(&mut self) -> Result<(), Self::Error>;
}
