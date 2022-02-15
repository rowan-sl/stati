use core::cell::RefCell;
use std::{io::Write, rc::Rc};

use std::sync::Arc;
#[cfg(feature = "fairness")]
use parking_lot::FairMutex as Mutex;
#[cfg(not(feature = "fairness"))]
use parking_lot::Mutex;

use crate::isbar::{IsBar, IsBarManagerInterface};
use crate::wrapper::{BarWrapper, ThreadedBarWrapper};

/**
Manager for all current progress bars and text output.

This can be used, with the new_bar method and
the [`println!`] and [`print!`] (crate) macros
to display progress bars and even print while doing so

to display the bar, simply call `.print()`.
in adition, the bar will be automaticaly printed when using
the [`print!`] and [`println!`] macros.

## Examples

simple bar

```rust
use std::thread;

use stati::BarManager;
use stati::prelude::*;

# fn main() {
let mut manager = BarManager::new();
let mut bar = manager.register_bar(stati::bars::SimpleBar::new("Working...", ()));
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
use stati::prelude::*;

# fn main() {
let mut manager = BarManager::new();
let mut bar = manager.register_bar(stati::bars::SimpleBar::new("Working...", ()));
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

for a thread-safe version, see [`ThreadedBarWrapper`]

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

    /// Registers a progress bar with the bar manager, to be drawn with the manager.
    /// Returns what is effectively a reference to it, and when that refference is dropped or `.done()` is called,
    /// the bar is finished, and is completed according to `bar.close_method()`
    pub fn register_bar<B: 'bar + IsBar>(&mut self, bar: B) -> BarWrapper<B> {
        let wrapped = Rc::new(RefCell::new(bar));
        self.bars.push(wrapped.clone());
        wrapped.into()
    }

    /// Formats the current progress bars, along with the text as messages
    /// that have been printed in this time, to a string.
    ///
    /// this assumes that nothing has been written to stdout in the time since it was last called, and as such
    /// you should not use `std::println!` or `std::print!` with this, and instead `stati::println!` or `stati::print!`
    /// 
    /// # Panics
    /// if it cannot borrow any of the contained bars
    #[must_use]
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
        let mut bar_filterer = |b: &mut Rc<RefCell<dyn IsBarManagerInterface>>| {
            let mut bref = b.borrow_mut();
            if bref.is_done() {
                match bref.close_method() {
                    crate::BarCloseMethod::Clear => {}
                    crate::BarCloseMethod::LeaveBehind => {
                        res += &bref.display();
                        res += "\n";
                    }
                }
                true
            } else {
                res += &bref.display();
                res += "\n";
                false
            }
        };
        let mut i = 0;
        while i < self.bars.len() {
            if bar_filterer(&mut self.bars[i]) {
                let _ = self.bars.remove(i);
            } else {
                i += 1;
            }
        }
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

/**
A version of [`BarManager`] that can controll bars used across multiple threads
through the [`ThreadedBarWrapper`]. the manager itself **cannot** be used across multiple threads,
but the bars returned by it can be used in a different thread than it

for more information, see [`BarManager`]
*/
pub struct ThreadedBarManager<'bar> {
    bars: Vec<Arc<Mutex<dyn IsBarManagerInterface + 'bar>>>,
    print_queue: Vec<String>,
    last_lines: usize,
}

impl<'bar> ThreadedBarManager<'bar> {
    /// Creates a new [`ThreadedBarManager`]
    pub fn new() -> Self {
        Self {
            bars: vec![],
            print_queue: vec![],
            last_lines: 0,
        }
    }

    /// Registers a progress bar with the bar manager, to be drawn with the manager.
    /// Returns what is effectively a reference to it, and when that refference is dropped or `.done()` is called,
    /// the bar is finished, and is completed according to `bar.close_method()`
    pub fn register_bar<B: 'bar + IsBar>(&mut self, bar: B) -> ThreadedBarWrapper<B> {
        let wrapped = Arc::new(Mutex::new(bar));
        self.bars.push(wrapped.clone());
        wrapped.into()
    }

    /// Formats the current progress bars, along with the text as messages
    /// that have been printed in this time, to a string.
    ///
    /// this assumes that nothing has been written to stdout in the time since it was last called, and as such
    /// you should not use `std::println!` or `std::print!` with this, and instead `stati::println!` or `stati::print!`
    /// 
    /// # Panics
    /// if it cannot borrow any of the contained bars
    #[must_use]
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
        let mut bar_filterer = |b: &mut Arc<Mutex<dyn IsBarManagerInterface>>| {
            let mut bref = b.lock();
            if bref.is_done() {
                match bref.close_method() {
                    crate::BarCloseMethod::Clear => {}
                    crate::BarCloseMethod::LeaveBehind => {
                        res += &bref.display();
                        res += "\n";
                    }
                }
                true
            } else {
                res += &bref.display();
                res += "\n";
                false
            }
        };
        let mut i = 0;
        while i < self.bars.len() {
            if bar_filterer(&mut self.bars[i]) {
                let _ = self.bars.remove(i);
            } else {
                i += 1;
            }
        }
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
    /// for a non-panicing alternative, see [`ThreadedBarManager::try_print`]
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

impl<'bar> Default for ThreadedBarManager<'bar> {
    fn default() -> Self {
        Self::new()
    }
}
