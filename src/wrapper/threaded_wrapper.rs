#[cfg(feature = "fairness")]
use parking_lot::FairMutex as Mutex;
#[cfg(not(feature = "fairness"))]
use parking_lot::Mutex;
use std::sync::Arc;

use super::IsBarWrapper;
use crate::isbar::IsBar;

/// a wrapper around a [`Bar`], allowing the manager to keep a copy while
/// passing one to the user
///
/// this one is thread-safe!
///
/// when this is dropped, `done()` *should* be called,
/// however it does not check if it succedded or not to avoid panicking,
/// so it may not have been called. if you want to check this, call `done()` manually
///
///
/// [`Bar`]: IsBar
#[derive(Clone, Debug)]
pub struct ThreadedBarWrapper<B: IsBar>(Arc<Mutex<B>>);

#[cfg(not(feature = "nightly"))]
use std::ops::DerefMut;
#[cfg(not(feature = "nightly"))]
impl<B: IsBar> IsBarWrapper for ThreadedBarWrapper<B> {
    type Bar = B;
    type Error = ();

    fn try_bar<'b>(&'b mut self) -> Result<Box<dyn DerefMut<Target = Self::Bar> + 'b>, ()> {
        Ok(Box::new(self.0.lock()))
    }

    /// Get a reference to the underlying bar.
    ///
    /// warning! **DO NOT** call this twice without dropping
    /// the first reference returned, this will cause a deadlock!
    ///
    /// for a non-panicking alternative, see [`try_bar`]
    /// 
    /// [`try_bar`]: ThreadedBarWrapper::try_bar
    fn bar<'b>(&'b mut self) -> Box<dyn DerefMut<Target = Self::Bar> + 'b> {
        self.try_bar().unwrap()
    }
}

#[cfg(feature = "nightly")]
use parking_lot::{RawFairMutex, lock_api::MutexGuard};
#[cfg(feature = "nightly")]
impl<B: IsBar> IsBarWrapper for ThreadedBarWrapper<B> {
    type Bar = B;
    type Error = ();
    type BarGuard<'g> where Self: 'g = MutexGuard<'g, RawFairMutex, Self::Bar>;

    fn try_bar<'g>(&'g mut self) -> Result<Self::BarGuard<'g>, ()> {
        Ok(self.0.lock())
    }

    /// Get a reference to the underlying bar.
    ///
    /// warning! **DO NOT** call this twice without dropping
    /// the first reference returned, this will cause a deadlock!
    ///
    /// for a non-panicking alternative, see [`try_bar`]
    /// 
    /// [`try_bar`]: ThreadedBarWrapper::try_bar
    fn bar<'g>(&'g mut self) -> Self::BarGuard<'g> {
        self.try_bar().unwrap()
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
