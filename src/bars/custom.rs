use std::time::Instant;

mod default {
    pub const FILLED: &str = "=";
    pub const EMPTY: &str = "-";
    pub const START: &str = "[";
    pub const END: &str = "]";
    pub const UNIT: &str = "its";
}

#[derive(Clone, Debug, Hash)]
pub struct BarElements {
    filled: String,
    empty: String,
    start: String,
    end: String,
    unit: String,
}

impl Default for BarElements {
    fn default() -> Self {
        use default::*;
        Self {
            filled: FILLED.to_string(),
            empty: EMPTY.to_string(),
            start: START.to_string(),
            end: END.to_string(),
            unit: UNIT.to_string(),
        }
    }
}

/// A Much more customiseable and advanced version of [`SimpleBar`].
/// 
/// To construct, use [`Builder`]
/// 
/// [`Builder`]: Builder
/// [`SimpleBar`]: super::simple/*lmao*/::SimpleBar
#[derive(Clone, Debug, Hash)]
pub struct CustomBar {
    job_name: String,
    progress: usize,
    max_hint: usize,
    finished: bool,
    last_iter: Instant,
    elems: BarElements,
}

impl CustomBar {
    pub fn set_name(&mut self, job_name: String) {
        self.job_name = job_name;
    }
}

impl crate::IsBar for CustomBar {
    fn done(&mut self) {
        self.finished = true;
    }

    #[must_use]
    fn is_done(&self) -> bool {
        self.finished
    }

    #[must_use]
    fn display(&mut self) -> String {
        let now = Instant::now();
        let time_fmt = format!("{:.3}", 1f32 / (now - self.last_iter).as_secs_f32());
        self.last_iter = now;

        //TODO make this not use default
        let width = crate::utils::term_width().unwrap_or(81) as usize;

        let mut res = String::with_capacity(width);

        let percentage = self.progress * 100 / self.max_hint;
        let bar_len = width.checked_sub(
            self.job_name.len() +
            1 /* gap */ +
            1 /* bar start */ +
            /* bar would go here */
            1 /* bar end */ +
            1 /* gap */ +
            5 /* precent len */ +
            1 /*gap*/ +
            time_fmt.len() /* time amnt*/+
            1 /*gap*/ +
            self.elems.unit.len() + 2 /* unit len (___/s) */
        ).unwrap();
        let bar_finished_len = (bar_len as f32 * percentage as f32 / 100.0) as isize;

        res += "\r";
        res += &self.job_name;
        res += " ";
        res += &self.elems.start;
        for _ in 0..bar_finished_len {
            res += &self.elems.filled;
        }
        for _ in bar_finished_len as usize..bar_len {
            res += &self.elems.empty;
        }
        res += &self.elems.end;
        //pad to 4 chars on left
        res += &format!("{:>4}%", percentage);
        res += " ";
        res += &time_fmt;
        res += " ";
        res += &self.elems.unit;
        res += "/s";

        res
    }

    #[must_use]
    fn close_method(&self) -> crate::isbar::BarCloseMethod {
        crate::isbar::BarCloseMethod::LeaveBehind
    }
}

impl crate::subsets::IteratorProgress for CustomBar {
    fn set_progress(&mut self, progress: usize) {
        self.progress = progress;
    }

    fn set_size_hint(&mut self, hint: usize) {
        self.max_hint = hint;
    }
}

/// Builder pattern builder for [`CustomBar`]
/// 
/// to create the builder, use [`new`] and to construct the bar use [`build`]
/// 
/// [`CustomBar`]: CustomBar
/// [`new`]: Builder::new
/// [`build`]: Builder::build
pub struct Builder {
    job_name: String,
    hint: usize,
    elems: BarElements,
}

impl Builder {
    /// Create a new [`Builder`]
    #[must_use]
    pub fn new(name: impl ToString) -> Self {
        Self {
            job_name: name.to_string(),
            hint: 100,
            elems: BarElements {
                ..Default::default()
            },
        }
    }

    /// Sets the size hint
    #[must_use]
    pub fn hint(mut self, hint: usize) -> Self {
        self.hint = hint;
        self
    }

    /// Set the bar element customization
    #[must_use]
    pub fn elems(mut self, elems: BarElements) -> Self {
        self.elems = elems;
        self
    }

    /// Set the string for the filled bar section (should be one charecter in length, is repeated)
    #[must_use]
    pub fn filled(mut self, v: impl ToString) -> Self {
        self.elems.filled = v.to_string();
        self
    }

    /// Set the string for the empty bar section (should be one charecter in length, is repeated)
    #[must_use]
    pub fn empty(mut self, v: impl ToString) -> Self {
        self.elems.empty = v.to_string();
        self
    }

    /// Set the start string for the bar section
    #[must_use]
    pub fn start(mut self, v: impl ToString) -> Self {
        self.elems.start = v.to_string();
        self
    }

    /// Set the end string for the bar section
    #[must_use]
    pub fn end(mut self, v: impl ToString) -> Self {
        self.elems.end = v.to_string();
        self
    }

    /// Sets the unit in ___/sec for the progress bar
    #[must_use]
    pub fn unit(mut self, v: impl ToString) -> Self {
        self.elems.unit = v.to_string();
        self
    }

    /// Build the [`CustomBar`]
    #[must_use]
    pub fn build(self) -> CustomBar {
        CustomBar {
            job_name: self.job_name,
            max_hint: self.hint,
            elems: self.elems,
            progress: 0,
            last_iter: Instant::now(),
            finished: false,
        }
    }
}
