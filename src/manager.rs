use core::cell::RefCell;
use std::fmt::Debug;
use std::{io::Write, rc::Rc};

#[cfg(feature = "fairness")]
use parking_lot::FairMutex as Mutex;
#[cfg(not(feature = "fairness"))]
use parking_lot::Mutex;
use std::sync::Arc;

use crate::isbar::{IsBar, IsBarManagerInterface};
use crate::wrapper::{BarWrapper, ThreadedBarWrapper};

/**
Manager for all current progress bars and text output.

the bars produced by this can be used in other threads from the manager, if they are created with [`register_threadsafe`]

This can be used, with the [`register`] and [`register_threadsafe`] methods and
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
let mut bar = manager.register(stati::bars::SimpleBar::new(&"Working...", 100));
for i in 0..=100 {
    bar.bar().set_progress(i);
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
let mut bar = manager.register(stati::bars::SimpleBar::new(&"Working...", 100));
for i in 0..=100 {
    bar.bar().set_progress(i);
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

# a note on bar ordering
bars created with [`register_threadsafe`] will always be displayed after bars
created with [`register`]

[`print!`]: crate::print
[`println!`]: crate::print
[`register_threadsafe`]: BarManager::register_threadsafe
[`register`]: BarManager::register
*/
#[derive(Debug)]
pub struct BarManager<'bar> {
    bars: Vec<Rc<RefCell<dyn IsBarManagerInterface + 'bar>>>,
    threaded_bars: Vec<Arc<Mutex<dyn IsBarManagerInterface + 'bar>>>,
    print_queue: Vec<String>,
    last_lines: usize,
}

impl<'bar> BarManager<'bar> {
    /// Creates a new [`BarManager`]
    #[must_use]
    pub fn new() -> Self {
        Self {
            bars: vec![],
            threaded_bars: vec![],
            print_queue: vec![],
            last_lines: 0,
        }
    }

    /// Registers a progress bar with the bar manager, to be drawn with the manager.
    /// Returns what is effectively a reference to it, and when that refference is dropped or `.done()` is called,
    /// the bar is finished, and is completed according to `bar.close_method()`
    ///
    /// to register a bar so it can be used across threads, see [`register_threadsafe`]
    ///
    /// [`register_threadsafe`]: Self::register_threadsafe
    pub fn register<B: 'bar + IsBar + Debug>(&mut self, bar: B) -> BarWrapper<B> {
        let wrapped = Rc::new(RefCell::new(bar));
        self.bars.push(wrapped.clone());
        wrapped.into()
    }

    /// Like [`register`], however the wrapper returned by this can be used across threads
    ///
    /// [`register`]: Self::register
    pub fn register_threadsafe<B: 'bar + IsBar + Debug>(
        &mut self,
        bar: B,
    ) -> ThreadedBarWrapper<B> {
        let wrapped = Arc::new(Mutex::new(bar));
        self.threaded_bars.push(wrapped.clone());
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
                self.bars.remove(i);
            } else {
                i += 1;
            }
        }
        //& do it again, but with threaded bars this time
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
        while i < self.threaded_bars.len() {
            if bar_filterer(&mut self.threaded_bars[i]) {
                self.threaded_bars.remove(i);
            } else {
                i += 1;
            }
        }
        self.last_lines = self.bars.len() + self.threaded_bars.len();
        res
    }

    /// Attempts to flush the output, returning if it was sucsessfull or not
    #[allow(clippy::missing_errors_doc)]
    #[allow(clippy::unused_self)] //it may be used in the future
    pub fn try_flush(&mut self) -> std::io::Result<()> {
        std::io::stdout().flush()
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

    /// Attempts to print and flush stdout
    ///
    /// # Errors
    /// if stdout could not be flushed
    pub fn try_print(&mut self) -> std::io::Result<()> {
        self.print_no_flush();
        self.try_flush()
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
