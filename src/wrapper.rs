use core::cell::RefCell;
use std::rc::Rc;

use crate::isbar::IsBar;

/// a wrapper around a [`Bar`], allowing the manager to keep a copy while
/// passing one to the user
///
/// [`Bar`]: IsBar
#[derive(Clone)]
pub struct BarWrapper<B: IsBar>(Rc<RefCell<B>>);

impl<B: IsBar> BarWrapper<B> {
    /// Sets the progress of the bar. for more info, see [`IsBar::set_progress`]
    pub fn set_progress(&mut self, progress: B::Progress) {
        self.0.borrow_mut().set_progress(progress);
    }

    /// Sets the job name of the bar. for more info, see [`IsBar::set_name`]
    pub fn set_name(&mut self, job_name: String) {
        self.0.borrow_mut().set_name(job_name);
    }

    /// Indicates that the bar has finished, and can be finalized and dropped by the manager.
    /// for more info, see [`IsBar::done`]
    ///
    /// this is also called by the [`Drop`] impl on this type
    pub fn done(&mut self) {
        self.0.borrow_mut().done();
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
            b.done();
        }
    }
}
