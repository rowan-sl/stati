use core::cell::RefCell;
use std::{rc::Rc, io::Write};

use crate::isbar::IsBar;
use crate::wrapper::BarWrapper;

/**
Manager for all current progress bars and text output.

This can be used, with the new_bar method and
the [`println!`] and [`print!`] (crate) macros
to display progress bars and even print while doing so

to display the bar, simply call `.print()`.
in adition, the bar will be automaticaly printed when using
the [`print!`] and [`println!`] macros.

Please note that you currently cannot use more than one bar type
in a manager. this may change in the future as it *is* kinda a problem,
but fixing it would have some tradeoffs

## Examples

simple bar

```rust
use std::thread;

use stati::BarManager;

# fn main() {
let mut manager = BarManager::<stati::bars::SimpleBar>::new();
let mut bar = manager.new_bar("Working...".into());
for i in 0..=100 {
    bar.set_progress(i);
    manager.print();
    # #[allow(deprecated)]
    thread::sleep_ms(40);
}
# }
```

printing while using progress bar

```rust
use std::thread;

use stati::BarManager;

# fn main() {
let mut manager = BarManager::<stati::bars::SimpleBar>::new();
let mut bar = manager.new_bar("Working...".into());
for i in 0..=100 {
    bar.set_progress(i);
    stati::println!(manager, "Progressed to {} in the first section", i);
    manager.print();
    # #[allow(deprecated)]
    thread::sleep_ms(40);
}
# }
```

# A note on ANSI controll charecters
The way that this uses these charecters to re-print bars is rather finicky,
and if you use ANSI text while using this, it probably will work, but could
also break and cause hard to debug errors.

## Thread Saftey
***n o***

[`print!`]: crate::print
[`println!`]: crate::print
*/
pub struct BarManager<P> {
    bars: Vec<Rc<RefCell<dyn IsBar<Progress = P>>>>,
    last_lines: usize,
}

impl<P> BarManager<P> {
    /// Creates a new [`BarManager`]
    pub fn new() -> Self {
        Self {
            bars: vec![],
            last_lines: 0,
        }
    }

    /// Creates a new progeress bar, returning what is effectivley
    /// reference to it. when the reference is dropped or `.done()` is called,
    /// the bar is finished, and is no longer tracked or re-printed.
    pub fn new_bar<B: IsBar<Progress = P>>(&mut self, name: String) -> BarWrapper<P> {
        let bar = Rc::new(RefCell::new(B::new(name)));
        self.bars.push(bar.clone());
    }

    /// Formats the current progress bars, along with the text as messages
    /// that have been printed in this time, to a string.
    /// 
    /// this assumes that nothing has been written to stdout in the time since it was last called, and as such
    /// you should not use `std::println!` or `std::print!` with this, and instead `stati::println!` or `stati::print!`
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

    /// Queues text to be printed before the bars. this should NOT be use
    /// directly, but should be used with the println! and print! macros
    pub fn queue_text(&mut self, text: &str) {
        std::print!("{}", self.display(text));
        std::io::stdout().flush().unwrap();
    }

    /// Print the current bars to stdout
    /// 
    /// This should be called after updating progress bars,
    /// but does not need to be called after using println! or print!
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
