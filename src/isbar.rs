pub enum BarCloseMethod {
    LeaveBehind,
    Clear,
}

/// Functions that a progress bar should implement,
/// like [`SimpleBar`]
///
/// [`SimpleBar`]: crate::bars::simple::SimpleBar
pub trait IsBar {
    /// The unit that is passed to [`IsBar::set_progress`]
    ///
    /// for example, if the bar was a precentage done, then this could be a `usize`
    type Progress;

    /// Arguments that are passed to [`IsBar::new`]
    type Args;

    /// Crate a new [`Bar`] with the provided name,
    /// and a progress level of zero
    ///
    /// [`Bar`]: IsBar
    fn new(job_name: String, args: Self::Args) -> Self
    where
        Self: Sized;

    /// Finishes the [`Bar`], allowing it to be finalized and dropped by the manager.
    /// the bar should not be used after this is called
    ///
    /// [`Bar`]: IsBar
    fn done(&mut self);

    /// Checks if the [`Bar`] is done ([`IsBar::done`] as been called)
    ///
    /// [`Bar`]: IsBar
    fn is_done(&self) -> bool;

    /// Sets the progress level of the [`Bar`]
    ///
    /// [`Bar`]: IsBar
    fn set_progress(&mut self, progress: Self::Progress);

    /// Sets the name of the [`Bar`]
    ///
    /// [`Bar`]: IsBar
    fn set_name(&mut self, job_name: String);

    /// Formats the [`Bar`] into a string. this is generaly only used by the [`BarManager`]
    ///
    /// [`Bar`]: IsBar
    /// [`BarManager`]: crate::manager::BarManager
    fn display(&self) -> String;

    fn close_method(&self) -> BarCloseMethod;
}

/// Subsets of progress bars that can be used for various things
pub mod subsets {
    use super::IsBar;

    /// Internal trait auto impl for types that are PrecentageBarFlag and meet the requirements
    /// do not implement this
    pub trait PrecentageBar {
        type Args;
    }

    /// Bars that accept a precentage for the Progress type (represented as usize)
    /// bars like this should generaly use 100% (`100usize`) for completed,
    ///
    /// This should be implemented for types that fit these conditions and
    /// are ok to use as such
    pub trait PrecentageBarFlag {}

    impl<T, A> PrecentageBar for T
    where
        T: IsBar<Args = A, Progress = usize> + PrecentageBarFlag,
    {
        type Args = A;
    }
}

pub(crate) trait IsBarManagerInterface {
    fn display(&self) -> String;

    fn is_done(&self) -> bool;
}

impl<T> IsBarManagerInterface for T
where
    T: IsBar,
{
    fn display(&self) -> String {
        <T as IsBar>::display(&self)
    }

    fn is_done(&self) -> bool {
        <T as IsBar>::is_done(&self)
    }
}
