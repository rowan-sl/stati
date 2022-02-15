use std::sync::Arc;
#[cfg(feature = "fairness")]
use parking_lot::FairMutex as Mutex;
#[cfg(not(feature = "fairness"))]
use parking_lot::Mutex;

use crate::isbar::IsBar;
use super::IsBarWrapper;


/// a wrapper around a [`Bar`], allowing the manager to keep a copy while
/// passing one to the user
/// 
/// this one is thread-safe!
///
/// when this is dropped, `done()` *should* be called,
/// however it does not check if it succedded or not to avoid panicking,
/// so it may not have been called. if you want to check this, call `done()` manually
/// 
/// [`Bar`]: IsBar
#[derive(Clone)]
pub struct ThreadedBarWrapper<B: IsBar>(Arc<Mutex<B>>);

impl<B: IsBar> IsBarWrapper for ThreadedBarWrapper<B> {
    type Bar = B;
    type Error = ();
    /// Sets the progress of the bar. for more info, see [`IsBar::set_progress`]
    fn set_progress(&mut self, progress: B::Progress) -> Result<(), ()>{
        self.0.lock().set_progress(progress);
        Ok(())
    }

    /// Sets the job name of the bar. for more info, see [`IsBar::set_name`]
    fn set_name(&mut self, job_name: String) -> Result<(), ()> {
        self.0.lock().set_name(job_name);
        Ok(())
    }

    /// Indicates that the bar has finished, and can be finalized and dropped by the manager.
    /// for more info, see [`IsBar::done`]
    ///
    /// this is also called by the [`Drop`] impl on this type
    fn done(&mut self) -> Result<(), ()> {
        self.0.lock().done();
        Ok(())
    }
}

impl<B: IsBar> From<Arc<Mutex<B>>> for ThreadedBarWrapper<B> {
    fn from(item: Arc<Mutex<B>>) -> Self {
        Self(item)
    }
}

impl<B: IsBar> Drop for ThreadedBarWrapper<B> {
    fn drop(&mut self) {
        self.0.lock().done();
    }
}

impl<B: IsBar> crate::sealant::Sealed for ThreadedBarWrapper<B> {}
