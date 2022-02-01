# stati

Rust library for progress bars

## WARNING

currently very unfinished, semi-undocumented, and requires nightly rust to build,
as it is still very much in development.

## Installation

This currently requires nightly rust, so first setup the nightly toolchain:

```
rustup override set nightly
```

Then add this line to your `Cargo.toml`

```toml
stati = "0.5.0-beta"
```

## Usage

This is a simple example of how to create and progress a progress bar

```rust
use std::thread;

extern crate stati;

let mut manager = BarManager::<stati::bars::SimpleBar>::new();
let mut bar = manager.new_bar::<bars::SimpleBar>("Working...".into(), ());
for i in 0..=100 {
    bar.set_progress(i);
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

MIT, see [LISENCE](LICENSE)

## TODO's

- [ ] improve docs
- [ ] improve tests
- [ ] improve examples
- [ ] add builder pattern support for making progress bars
- [ ] better iterator tracking
- [ ] multithreading?
- [ ] create bar style with string formatting like indicatif?
