#![feature(drain_filter)]

extern crate termion;

pub mod bars;

use core::cell::RefCell;
use std::{rc::Rc, io::Write};


pub(crate) fn term_width() -> std::io::Result<u16> {
    use termion::terminal_size;
    Ok(terminal_size()?.0)
}

/**
manager for all current progress bars and text output.

when printing text, when text is printed it removes all (unfinished) 
bars, prints the text, and reprints all bars
*/
pub struct BarManager<B: IsBar> {
    bars: Vec<Rc<RefCell<B>>>,
    last_lines: usize,
}

impl<B: IsBar> BarManager<B> {
    pub fn new() -> Self {
        Self {
            bars: vec![],
            last_lines: 0,
        }
    }

    pub fn new_bar(&mut self, name: String) -> BarWrapper<B> {
        let bar = Rc::new(RefCell::new(B::new(name)));
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

impl<B: IsBar> Default for BarManager<B> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct BarWrapper<B: IsBar> (Rc<RefCell<B>>);

impl<B: IsBar> BarWrapper<B> {
    pub fn set_progress(&mut self, progress: B::Progress) {
        self.0.borrow_mut().set_progress(progress);
    }

    pub fn set_name(&mut self, job_name: String) {
        self.0.borrow_mut().set_name(job_name);
    }

    pub fn done(&mut self) {
        self.0.borrow_mut().done();
    }
}

impl<B: IsBar> From<Rc<RefCell<B>>> for BarWrapper<B> {
    fn from(item: Rc<RefCell<B>>) -> Self {
        Self (item)
    }
}

impl<B: IsBar> Drop for BarWrapper<B> {
    fn drop(&mut self) {
        if let Ok(mut b) = self.0.try_borrow_mut() {
            b.done();
        }
    }
}

pub trait IsBar {
    type Progress;

    fn new(job_name: String) -> Self where Self: Sized;

    fn done(&mut self);

    fn is_done(&self) -> bool;

    fn set_progress(&mut self, progress: Self::Progress);

    fn set_name(&mut self, job_name: String);

    fn display(&self) -> String;
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
