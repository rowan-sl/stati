#![feature(format_args_nl)]

extern crate stati;

use std::{thread::sleep, time::Duration};

use stati::*;


fn main() {
    let mut bman = BarManager::new();
    let mut b1 = bman.new_bar("bar1".into());
    for i in 0..=50 {
        b1.set_precent(i);
        bman.print();
        sleep(Duration::from_millis(50));
    }
    let mut b2 = bman.new_bar("bar2".into());
    for i in 0..=50 {
        b1.set_precent(i+50);
        b2.set_precent(i);
        bman.print();
        sleep(Duration::from_millis(50));
    }
    b1.done();
    for i in 50..=100 {
        b2.set_precent(i);
        bman.print();
        sleep(Duration::from_millis(50));
    }
}
