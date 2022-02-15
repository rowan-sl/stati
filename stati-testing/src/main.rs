#![allow(unused_must_use)]

extern crate stati;

use std::{thread::{sleep, self}, time::Duration};

use stati::{bars, prelude::*, BarManager, ThreadedBarManager};

fn main() {
    {
        let mut bman = BarManager::new();
        let mut s1 = bman.register_bar(
            bars::SpinniBuilder::new("Spinni whee".into())
                .task_name("doing thing".into())
                .close_method(stati::BarCloseMethod::Clear)
                .build()
        );
        for i in 0..1000 {
            if i > 700 {
                s1.set_progress(format!("almost done! {} way there", i));
            }
            bman.print();
            sleep(Duration::from_millis(10));
        }
        s1.done();
        for i in (0..50).display_bar(bman.register_bar(bars::SimpleBar::new("Iterator", ()))) {
            stati::println!(bman, "Progressed to {} with iterator", i);
            sleep(Duration::from_millis(50));
        }
        let mut b1 = bman.register_bar(bars::SimpleBar::new("bar1", ()));
        for i in 0..=50 {
            b1.set_progress(i);
            stati::println!(bman, "Progressed to {} in the first section", i);
            bman.print();
            sleep(Duration::from_millis(50));
        }
        let mut b2 = bman.register_bar(bars::SimpleBar::new("bar2", ()));
        for i in 0..=50 {
            b1.set_progress(i+50);
            b2.set_progress(i);
            stati::println!(bman, "Progressed to {} in the second section", i);
            bman.print();
            sleep(Duration::from_millis(50));
        }
        b1.done();
        for i in 50..=100 {
            stati::println!(bman, "Progressed to {} in the third section", i);
            b2.set_progress(i);
            bman.print();
            sleep(Duration::from_millis(50));
        }
    }
    {
        let mut manager = ThreadedBarManager::new();

        let mut b1 = manager.register_bar(bars::SimpleBar::new("progressing from main thread", ()));
        let b2 = manager.register_bar(bars::SimpleBar::new("progressing from new thread", ()));

        let h = thread::spawn(move || {
            // for i in 0..=100 {
            //     b2.set_progress(i);
            //     thread::sleep(Duration::from_millis(50));
            // }
            for _ in (0..=100).display_bar(b2) {
                thread::sleep(Duration::from_millis(50));
            }
        });

        for i in 0..=100 {
            b1.set_progress(i);
            manager.print();
            thread::sleep(Duration::from_millis(50));
        }

        h.join().unwrap();
    }


}
