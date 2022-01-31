
/// Functions that a progress bar should implement,
/// like [`SimpleBar`]
/// 
/// [`SimpleBar`]: crate::bars::simple::SimpleBar
pub trait IsBar {
    /// The unit that is passed to [`IsBar::set_progress`]
    /// 
    /// for example, if the bar was a precentage done, then this could be a `usize`
    type Progress;

    /// Crate a new [`Bar`] with the provided name,
    /// and a progress level of zero
    /// 
    /// [`Bar`]: IsBar
    fn new(job_name: String) -> Self where Self: Sized;

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
}
