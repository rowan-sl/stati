use core::cell::RefCell;
use std::rc::Rc;

use crate::isbar::IsBar;

/// a wrapper around a [`Bar`], allowing the manager to keep a copy while
/// passing one to the user
/// 
/// [`Bar`]: IsBar
#[derive(Clone)]
pub struct BarWrapper<P> (Rc<RefCell<dyn IsBar<Progress = P>>>);

impl<P> BarWrapper<P> {
    pub(crate) fn new(b: Rc<RefCell<dyn IsBar<Progress = P>>>) -> Self {Self (b)}
    /// Sets the progress of the bar. for more info, see [`IsBar::set_progress`]
    pub fn set_progress(&mut self, progress: P) {
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

impl<P> From<Rc<RefCell<dyn IsBar<Progress = P>>>> for BarWrapper<P> {
    fn from(item: Rc<RefCell<dyn IsBar<Progress = P>>>) -> Self {
        Self (item)
    }
}

impl<P> Drop for BarWrapper<P> {
    fn drop(&mut self) {
        if let Ok(mut b) = self.0.try_borrow_mut() {
            b.done();
        }
    }
}
