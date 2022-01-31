pub trait IsBar {
    type Progress;

    fn new(job_name: String) -> Self where Self: Sized;

    fn done(&mut self);

    fn is_done(&self) -> bool;

    fn set_progress(&mut self, progress: Self::Progress);

    fn set_name(&mut self, job_name: String);

    fn display(&self) -> String;
}
