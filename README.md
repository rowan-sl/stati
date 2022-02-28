# stati

Rust library for progress bars

## WARNING

This crate is still in development,
and although it should be fine to use,
things may change at any time

## Crate Features

- fairness:
  - enables using `parking_lot`'s `FairMutex`
  - (adds some overhead but may fix some issues?)
- nightly:
  - enables using nighlty rust (generic_associated_types) for some extra optimizations

## Installation

Add this line to your `Cargo.toml`

```toml
stati = "0.11.0-beta"
```

## Usage

This is a simple example of how to create and progress a progress bar

```rust
use std::thread;

extern crate stati;

use stati::prelude::*;
use stati::BarManager;

let mut manager = BarManager::new();
let mut bar = manager.register(bars::SimpleBar::new(&"Working...", 100));
for i in 0..=100 {
    bar.bar().set_progress(i);
    manager.print();
    thread::sleep_ms(100);
}
```

To print text while using a progress bar, use the `println` and `print` macros like so

```rust
let mut manager = BarManager::new();
stati::println!(manager, "Made some progress");
```

## Why?

Many progress bars that I have used in the past
have suffered from issues such as not being able to have
multiple bars at once, or not being able to print other messages
while a bar was running. this crate aims to solve all of these issues,
and generaly make it much nicer to use.

## Lisence

MIT, see [LICENSE](LICENSE)

## TODO's

- [x] add fun spini spinner wheeeeeeeeeeeeeeeeee
- [x] non-nightly support
- [x] add optional nightly features
- [x] add builder pattern support for making progress bars
- [x] multithreading!
- [x] windows support
- [x] improve docs
- [ ] improve tests
- [ ] improve examples
- [ ] better iterator tracking
- [ ] create bar style with string formatting like indicatif?
- [ ] update to use Vec::drain_filter once it is stableized
- [ ] make bars use better formatting
