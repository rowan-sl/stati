use core::cell::RefCell;
use std::rc::Rc;

use crate::isbar::IsBar;

#[derive(Clone)]
pub struct BarWrapper<B: IsBar> (Rc<RefCell<B>>);

impl<B: IsBar> BarWrapper<B> {
    pub fn set_progress(&mut self, progress: B::Progress) {
        self.0.borrow_mut().set_progress(progress);
    }

    pub fn set_name(&mut self, job_name: String) {
        self.0.borrow_mut().set_name(job_name);
    }

    pub fn done(&mut self) {
        self.0.borrow_mut().done();
    }
}

impl<B: IsBar> From<Rc<RefCell<B>>> for BarWrapper<B> {
    fn from(item: Rc<RefCell<B>>) -> Self {
        Self (item)
    }
}

impl<B: IsBar> Drop for BarWrapper<B> {
    fn drop(&mut self) {
        if let Ok(mut b) = self.0.try_borrow_mut() {
            b.done();
        }
    }
}
