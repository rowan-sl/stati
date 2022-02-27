use crate::{BarCloseMethod, IsBar};

/// Spinny spinning spinner
#[derive(Clone, Debug, Hash)]
pub struct Spinni {
    tick_strings: Vec<char>,
    current_char: usize,
    job_name: String,
    subtask: String,
    done: bool,
    close_method: BarCloseMethod,
    tick_on_display: bool,
}

impl Spinni {
    pub(crate) fn new(
        job_name: String,
        subtask: String,
        close_method: BarCloseMethod,
        tick_on_display: bool,
    ) -> Self
    where
        Self: Sized,
    {
        Self {
            tick_strings: "⠁⠁⠉⠙⠚⠒⠂⠂⠒⠲⠴⠤⠄⠄⠤⠠⠠⠤⠦⠖⠒⠐⠐⠒⠓⠋⠉⠈⠈ ".chars().collect(),
            current_char: 0,
            job_name,
            subtask,
            done: false,
            close_method,
            tick_on_display,
        }
    }

    pub fn set_job(&mut self, job_name: String) {
        self.job_name = job_name;
    }

    pub fn set_subtask(&mut self, task_name: String) {
        self.subtask = task_name;
    }

    /// spin the wheel
    pub fn tick(&mut self) {
        if self.current_char == self.tick_strings.len() - 1 {
            self.current_char = 0;
        } else {
            self.current_char += 1;
        }
    }
}

impl IsBar for Spinni {
    fn done(&mut self) {
        self.done = true;
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn close_method(&self) -> crate::BarCloseMethod {
        self.close_method
    }

    fn display(&mut self) -> String {
        if self.tick_on_display {
            self.tick();
        }
        let spini_step = self.tick_strings[self.current_char];
        format!("{} {}: {}", spini_step, self.job_name, self.subtask)
    }
}

#[derive(Clone, Debug, Hash)]
pub struct SpinniBuilder {
    job_name: String,
    task_name: String,
    close_method: BarCloseMethod,
    tick_on_display: bool,
}

impl SpinniBuilder {
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            job_name: name,
            task_name: "".into(),
            close_method: BarCloseMethod::LeaveBehind,
            tick_on_display: true,
        }
    }

    #[must_use]
    #[allow(clippy::missing_const_for_fn)] //you are WRONG
    pub fn task_name(mut self, task_name: String) -> Self {
        self.task_name = task_name;
        self
    }

    #[must_use]
    pub const fn close_method(mut self, close_method: BarCloseMethod) -> Self {
        self.close_method = close_method;
        self
    }

    #[must_use]
    pub const fn tick_on_display(mut self, tick_on_display: bool) -> Self {
        self.tick_on_display = tick_on_display;
        self
    }

    #[must_use]
    pub fn build(self) -> Spinni {
        Spinni::new(
            self.job_name,
            self.task_name,
            self.close_method,
            self.tick_on_display,
        )
    }
}
