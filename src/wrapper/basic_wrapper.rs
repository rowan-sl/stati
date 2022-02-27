use core::cell::RefCell;
use std::rc::Rc;
use crate::isbar::IsBar;
use super::IsBarWrapper;
use std::cell::BorrowMutError;
#[cfg(not(feature = "nightly"))]
use std::ops::DerefMut;
#[cfg(feature = "nightly")]
use std::cell::RefMut;


/// a wrapper around a [`Bar`], allowing the manager to keep a copy while
/// passing one to the user
///
/// When this is dropped, [`done`] *should* be called,
/// however it does not check if it errors or not to avoid panicking,
/// so it may not have sucseeded. if you want to check this, call [`done`] manually
///
/// [`Bar`]: IsBar
/// [`done`]: IsBar::done
#[derive(Clone, Debug)]
pub struct BarWrapper<B: IsBar>(Rc<RefCell<B>>);

#[cfg(not(feature = "nightly"))]
impl<B: IsBar> IsBarWrapper for BarWrapper<B> {
    type Bar = B;
    type Error = BorrowMutError;
    fn try_bar<'b>(
        &'b mut self,
    ) -> Result<Box<dyn DerefMut<Target = Self::Bar> + 'b>, BorrowMutError> {
        Ok(Box::new(self.0.try_borrow_mut()?))
    }
}

#[cfg(feature = "nightly")]
impl<B: IsBar> IsBarWrapper for BarWrapper<B> {
    type Bar = B;
    type Error = BorrowMutError;
    type BarGuard<'g>  where Self: 'g = RefMut<'g, Self::Bar>;
    fn try_bar<'g>(
        &'g mut self,
    ) -> Result<Self::BarGuard<'g>, BorrowMutError> {
        self.0.try_borrow_mut()
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
