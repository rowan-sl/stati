use std::fmt::Debug;

use super::{BarCloseMethod, IsBar};

pub trait IsBarManagerInterface: Debug {
    fn display(&mut self) -> String;

    fn is_done(&self) -> bool;

    fn close_method(&self) -> BarCloseMethod;
}

impl<T> IsBarManagerInterface for T
where
    T: IsBar + Debug,
{
    fn display(&mut self) -> String {
        <T as IsBar>::display(self)
    }

    fn is_done(&self) -> bool {
        <T as IsBar>::is_done(self)
    }

    fn close_method(&self) -> BarCloseMethod {
        self.close_method()
    }
}
