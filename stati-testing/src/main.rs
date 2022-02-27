#![allow(unused_must_use)]

extern crate stati;

use std::{
    thread::{self, sleep},
    time::Duration,
};

use stati::{bars, prelude::*, BarManager};

fn main() {
    let mut bman = BarManager::new();
    let mut s1 = bman.register(
        bars::SpinniBuilder::new("Spinni whee".into())
            .task_name("doing thing".into())
            .close_method(stati::BarCloseMethod::Clear)
            .build(),
    );
    for i in 0..1000 {
        if i > 700 {
            s1.bar()
                .set_subtask(format!("almost done! {} way there", i));
            s1.bar().tick();
        }
        bman.print();
        sleep(Duration::from_millis(10));
    }
    s1.bar().done();
    for i in (0..=200)
        .display_bar(bman.register(bars::SimpleBar::new("Iterator", 200)))
        .manual_hint(200)
    {
        stati::println!(bman, "Progressed to {} with iterator", i);
        sleep(Duration::from_millis(50));
    }
    let mut b1 = bman.register(bars::SimpleBar::new("bar1", 100));
    for i in 0..=50 {
        b1.bar().set_progress(i);
        stati::println!(bman, "Progressed to {} in the first section", i);
        bman.print();
        sleep(Duration::from_millis(50));
    }
    let mut b2 = bman.register(bars::SimpleBar::new("bar2", 100));
    stati::println!(bman, "Debugging manager\n{:#?}", bman);
    for i in 0..=50 {
        b1.bar().set_progress(i + 50);
        b2.bar().set_progress(i);
        stati::println!(bman, "Progressed to {} in the second section", i);
        bman.print();
        sleep(Duration::from_millis(50));
    }
    b1.bar().done();
    for i in 50..=100 {
        stati::println!(bman, "Progressed to {} in the third section", i);
        b2.bar().set_progress(i);
        bman.print();
        sleep(Duration::from_millis(50));
    }
    let mut b3 =
        bman.register_threadsafe(bars::SimpleBar::new("progressing from main thread", 100));
    let b4 = bman.register_threadsafe(bars::SimpleBar::new("progressing from new thread", 100));

    let h = thread::spawn(move || {
        for _ in (0..=100).display_bar(b4) {
            thread::sleep(Duration::from_millis(50));
        }
    });

    for i in 0..=100 {
        b3.bar().set_progress(i);
        bman.print();
        thread::sleep(Duration::from_millis(50));
    }

    h.join().unwrap();

    stati::println!(bman, "done!");
}
