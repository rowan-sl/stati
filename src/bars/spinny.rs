use crate::BarCloseMethod;

/// Spinny spinning spinner
pub struct Spinni {
    tick_strings: Vec<char>,
    current_char: usize,
    args: SpinniArgs,
    job_name: String,
    done: bool,
}

pub struct SpinniArgs {
    current_task_name: String,
    close_method: BarCloseMethod,
}

impl crate::isbar::IsBar for Spinni {
    type Progress = Option<String>;
    type Args = SpinniArgs;

    fn new(job_name: String, args: Self::Args) -> Self
    where
            Self: Sized {
        Self {
            tick_strings: "⠁⠁⠉⠙⠚⠒⠂⠂⠒⠲⠴⠤⠄⠄⠤⠠⠠⠤⠦⠖⠒⠐⠐⠒⠓⠋⠉⠈⠈ "
                .chars()
                .collect(),
            current_char: 0,
            args,
            job_name,
            done: false,
        }
    }

    fn done(&mut self) {
        self.done = true;
    }

    fn is_done(&self) -> bool {
        self.done
    }

    /// In this case, it just updates the current task name and ticks it,
    /// only update name if progress is Some
    fn set_progress(&mut self, progress: Self::Progress) {
        if let Some(p) = progress {
            self.args.current_task_name = p;
        }
    }

    fn set_name(&mut self, job_name: String) {
        self.job_name = job_name;
    }

    fn close_method(&self) -> crate::BarCloseMethod {
        self.args.close_method
    }

    fn display(&mut self) -> String {
        if self.current_char == self.tick_strings.len() {
            self.current_char = 0;
        } else {
            self.current_char += 1;
        }
        let spini_step = self.tick_strings[self.current_char];
        format!("{} {}: {}", spini_step, self.job_name, self.args.current_task_name)
    }
}