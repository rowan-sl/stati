use core::cell::RefCell;
use std::{io::Write, rc::Rc};

use crate::isbar::{IsBar, IsBarManagerInterface};
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
let mut manager = BarManager::new();
let mut bar = manager.new_bar::<stati::bars::SimpleBar>("Working...".into(), ());
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
let mut manager = BarManager::new();
let mut bar = manager.new_bar::<stati::bars::SimpleBar>("Working...".into(), ());
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
pub struct BarManager<'bar> {
    bars: Vec<Rc<RefCell<dyn IsBarManagerInterface + 'bar>>>,
    print_queue: Vec<String>,
    last_lines: usize,
}

impl<'bar> BarManager<'bar> {
    /// Creates a new [`BarManager`]
    pub fn new() -> Self {
        Self {
            bars: vec![],
            print_queue: vec![],
            last_lines: 0,
        }
    }

    /// Creates a new progeress bar, returning what is effectivley
    /// reference to it. when the reference is dropped or `.done()` is called,
    /// the bar is finished, and is no longer tracked or re-printed.
    pub fn new_bar<B: 'bar + IsBar>(&mut self, name: String, args: B::Args) -> BarWrapper<B> {
        let bar = Rc::new(RefCell::new(B::new(name, args)));
        self.bars.push(bar.clone());
        bar.into()
    }

    /// Formats the current progress bars, along with the text as messages
    /// that have been printed in this time, to a string.
    ///
    /// this assumes that nothing has been written to stdout in the time since it was last called, and as such
    /// you should not use `std::println!` or `std::print!` with this, and instead `stati::println!` or `stati::print!`
    pub(crate) fn display(&mut self) -> String {
        let mut res = String::new();
        // ESC CSI n F (move to the start of the line n lines up)
        // (this is to overwrite previous bars)
        if self.last_lines != 0 {
            res += &format!("\x1b[{}F", self.last_lines);
        }
        // ESC CSI 0 J (clears from cursor to end of screen)
        res += "\x1b[0J";
        // print stuff
        for item in self.print_queue.drain(..) {
            res += &item;
        }
        // res += text;
        // go through all bars, removing ones that are done
        let _ = self.bars.drain_filter(|b| {
            res += &b.borrow_mut().display();
            res += "\n";
            b.borrow().is_done()
        });
        self.last_lines = self.bars.len();
        res
    }

    pub fn try_flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()?;
        Ok(())
    }

    /// Flushes updates to stdout.
    ///
    /// Currently this only flushes stdout, but will hopefully do more in the future
    ///
    /// # Panics
    /// if stdout cannot be flushed
    ///
    /// for a non-panicing alternative, see [`BarManager::try_flush`]
    pub fn flush(&mut self) {
        self.try_flush().unwrap();
    }

    /// Queues text to be printed before the bars. this should NOT be use
    /// directly, but should be used with the println! and print! macros
    ///
    /// this does NOT immediataly print the text
    pub fn queue_text(&mut self, text: &str) {
        self.print_queue.push(text.into());
    }

    /// Prints the bar status and any queued text to stdout, and flushes it.
    ///
    /// # Panics
    /// if stdout cannot be flushed
    ///
    /// for a non-panicing alternative, see [`BarManager::try_print`]
    pub fn print(&mut self) {
        self.try_print().unwrap();
    }

    pub fn try_print(&mut self) -> std::io::Result<()> {
        self.print_no_flush();
        self.try_flush()?;
        Ok(())
    }

    /// Prints the bar status and any queued text to stdout, without flushing it
    pub fn print_no_flush(&mut self) {
        std::print!("{}", self.display());
    }
}

impl<'bar> Default for BarManager<'bar> {
    fn default() -> Self {
        Self::new()
    }
}
