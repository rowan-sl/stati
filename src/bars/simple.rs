use super::common::TermWidthError;

const FILLED: &str = "=";
const EMPTY: &str = "-";
const START: &str = "[";
const END: &str = "]";
const UNIT: &str = "%";

/// A simple progress bar implementation, based off that of
/// the progress crates progresbar
#[derive(Clone, Debug, Hash)]
pub struct SimpleBar {
    job_name: String,
    progress: usize,
    max_hint: usize,
    finished: bool,
}

impl SimpleBar {
    /// name: the name of the job
    ///
    /// hint: hint for the maximum value this will reach
    pub fn new(name: impl ToString, hint: usize) -> Self {
        Self {
            job_name: name
                .to_string()
                .chars()
                .filter(|ch| ch != &'\n' || ch != &'\r')
                .collect(),
            progress: 0,
            max_hint: hint,
            finished: false,
        }
    }

    pub fn set_name(&mut self, job_name: String) {
        self.job_name = job_name;
    }
}

impl crate::IsBar for SimpleBar {
    fn done(&mut self) {
        self.finished = true;
    }

    fn is_done(&self) -> bool {
        self.finished
    }

    /// Some implementation details:
    ///
    /// starts with "\r" and has no end char
    ///
    ///  if it cannot get the real term size, uses 81 as the size
    fn display(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let width = match crate::utils::term_width() {
            Some(v) => {v},
            None => {return Err(Box::new(TermWidthError))},
        } as usize;

        let mut res =
            String::with_capacity(width as usize /* starts out as a u16, so its fine */);

        let percentage = self.progress * 100 / self.max_hint;
        let bar_len = width - (50 + 5) - 2;
        let bar_finished_len = (bar_len as f32 * percentage as f32 / 100.0) as i32;

        res += "\r";

        // pad to 50 chars on right
        res += &format!("{:<50}", self.job_name);
        res += START;
        for _ in 0..bar_finished_len {
            res += FILLED;
        }
        for _ in bar_finished_len as usize..bar_len {
            res += EMPTY;
        }
        res += END;

        //pad to 4 chars on left
        res += &format!("{:>4}", percentage);
        res += UNIT;

        Ok(res)
    }

    #[must_use]
    fn close_method(&self) -> Option<crate::isbar::BarCloseMethod> {
        None
    }
}

impl crate::subsets::IteratorProgress for SimpleBar {
    fn set_progress(&mut self, progress: usize) {
        self.progress = progress;
    }

    fn set_size_hint(&mut self, hint: usize) {
        self.max_hint = hint;
    }
}
