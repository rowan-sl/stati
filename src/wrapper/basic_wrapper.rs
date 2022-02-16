use core::cell::RefCell;
use std::{cell::BorrowMutError, ops::DerefMut, rc::Rc};

use super::IsBarWrapper;
use crate::isbar::IsBar;

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
    fn try_bar<'b>(
        &'b mut self,
    ) -> Result<Box<dyn DerefMut<Target = Self::Bar> + 'b>, BorrowMutError> {
        Ok(Box::new(self.0.try_borrow_mut()?))
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

impl<B: IsBar> crate::sealant::Sealed for BarWrapper<B> {}
