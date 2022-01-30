#![feature(drain_filter)]

extern crate termion;

use core::cell::RefCell;
use std::{rc::Rc, io::Write};


pub(crate) fn term_width() -> std::io::Result<u16> {
    use termion::terminal_size;
    Ok(terminal_size()?.0)
}

const FILLED: &str = "=";
const EMPTY: &str = "-";
const START: &str = "[";
const END: &str = "]";
const UNIT: &str = "%";

/**
manager for all current progress bars and text output.

when printing text, when text is printed it removes all (unfinished) 
bars, prints the text, and reprints all bars
*/
pub struct BarManager {
    bars: Vec<Rc<RefCell<Bar>>>,
    last_lines: usize,
}

impl BarManager {
    pub fn new() -> Self {
        Self {
            bars: vec![],
            last_lines: 0,
        }
    }

    pub fn new_bar(&mut self, name: String) -> BarWrapper {
        let bar = Rc::new(RefCell::new(Bar::new(name)));
        self.bars.push(bar.clone());
        bar.into()
    }

    pub fn display(&mut self, text: &str) -> String {
        let mut res = String::new();
        // ESC CSI n F (move to the start of the line n lines up)
        // (this is to overwrite previous bars)
        if self.last_lines != 0 {
            res += &format!("\x1b[{}F", self.last_lines);
        }
        // ESC CSI 0 J (clears from cursor to end of screen)
        res += "\x1b[0J";
        // print stuff
        res += text;
        // go through all bars, removing ones that are done
        let _ = self.bars.drain_filter(|b| {
            res += &b.borrow_mut().display();
            res += "\n";
            b.borrow().is_done()
        });
        self.last_lines = self.bars.len();
        res
    }

    pub fn queue_text(&mut self, text: &str) {
        std::print!("{}", self.display(text));
        std::io::stdout().flush().unwrap();
    }

    pub fn print(&mut self) {
        std::print!("{}", self.display(""));
        std::io::stdout().flush().unwrap();
    }
}

impl Default for BarManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct BarWrapper (Rc<RefCell<Bar>>);

impl BarWrapper {
    pub fn set_precent(&mut self, precent: usize) {
        self.0.borrow_mut().set_precent(precent);
    }

    pub fn done(&mut self) {
        self.0.borrow_mut().done();
    }
}

impl From<Rc<RefCell<Bar>>> for BarWrapper {
    fn from(item: Rc<RefCell<Bar>>) -> Self {
        Self (item)
    }
}

impl Drop for BarWrapper {
    fn drop(&mut self) {
        if let Ok(mut b) = self.0.try_borrow_mut() {
            b.done();
        }
    }
}

pub struct Bar {
    job_name: String,
    precentage: usize,
    finished: bool,
}

impl Bar {
    pub fn new(name: String) -> Self {
        Self {
            job_name: name.chars().filter(|ch| {ch != &'\n' || ch != &'\r'}).collect(),
            precentage: 0,
            finished: false,
        }
    }

    pub fn done(&mut self) {
        self.finished = true;
    }

    pub fn is_done(&self) -> bool {
        self.finished
    }

    pub fn set_precent(&mut self, precent: usize) {
        self.precentage = precent;
    }

    /// Some implementation details:
    /// 
    /// starts with "\r" and has no end char
    /// 
    ///  if it cannot get the real term size, uses 81 as the size
    pub fn display(&self) -> String {
        //TODO make this not use default
        let width = term_width().unwrap_or(81) as i32;

        let mut res = String::with_capacity(width as usize /* starts out as a u16, so its fine */);

        let overhead = self.precentage / 100;
        let left_percentage = self.precentage - overhead * 100;
        let bar_len = width - (50 + 5) - 2;
        let bar_finished_len = ((bar_len as f32) *
                                (left_percentage as f32 / 100.0)) as i32;
        let filled_symbol = if overhead & 0b1 == 0 {
            FILLED
        } else {
            EMPTY
        };
        let empty_symbol = if overhead & 0b1 == 0 {
            EMPTY
        } else {
            FILLED
        };

        res += "\r";

        // pad to 50 chars on right
        res += &format!("{:<50}", self.job_name);
        res += START;
        for _ in 0..bar_finished_len {
            res += filled_symbol;
        }
        for _ in bar_finished_len..bar_len {
            res += empty_symbol;
        }
        res += END;

        //pad to 4 chars on left
        res += &format!("{:>4}", self.precentage);
        res += UNIT;

        res
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! print {
        ($bm:ident, $($arg:tt)*) => ({
            $bm.queue_text(&format!($($arg)*));
        })
    }

    #[macro_export]
    macro_rules! println {
        ($bm: ident) => ($bm.queue_text("\n"));
        ($bm:ident, $($arg:tt)*) => ({
            $bm.queue_text(&format!("{}\n", format!($($arg)*)));
        })
    }
}
