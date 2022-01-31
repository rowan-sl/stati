use core::cell::RefCell;
use std::{rc::Rc, io::Write};

use crate::isbar::IsBar;
use crate::wrapper::BarWrapper;

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
