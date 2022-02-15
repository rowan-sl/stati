use core::cell::RefCell;
use std::{rc::Rc, cell::BorrowMutError};

use crate::isbar::IsBar;
use super::IsBarWrapper;

/// a wrapper around a [`Bar`], allowing the manager to keep a copy while
/// passing one to the user
///
/// when this is dropped, `done()` *should* be called,
/// however it does not check if it succedded or not to avoid panicking,
/// so it may not have been called. if you want to check this, call `done()` manually
/// 
/// [`Bar`]: IsBar
#[derive(Clone)]
pub struct BarWrapper<B: IsBar>(Rc<RefCell<B>>);

impl<B: IsBar> IsBarWrapper for BarWrapper<B> {
    type Bar = B;
    type Error = BorrowMutError;
    /// Sets the progress of the bar. for more info, see [`IsBar::set_progress`]
    fn set_progress(&mut self, progress: B::Progress) -> Result<(), BorrowMutError> {
        self.0.try_borrow_mut()?.set_progress(progress);
        Ok(())
    }

    /// Sets the job name of the bar. for more info, see [`IsBar::set_name`]
    fn set_name(&mut self, job_name: String) -> Result<(), BorrowMutError> {
        self.0.try_borrow_mut()?.set_name(job_name);
        Ok(())
    }

    /// Indicates that the bar has finished, and can be finalized and dropped by the manager.
    /// for more info, see [`IsBar::done`]
    ///
    /// this is also called by the [`Drop`] impl on this type
    fn done(&mut self) -> Result<(), BorrowMutError> {
        self.0.try_borrow_mut()?.done();
        Ok(())
    }
}

impl<B: IsBar> From<Rc<RefCell<B>>> for BarWrapper<B> {
    fn from(item: Rc<RefCell<B>>) -> Self {
        Self(item)
    }
}

impl<B: IsBar> Drop for BarWrapper<B> {
    fn drop(&mut self) {
        if let Ok(mut b) = self.0.try_borrow_mut() {
            let _ = b.done();
        }
    }
}

impl<B: IsBar> crate::sealant::Sealed for BarWrapper<B> {}
