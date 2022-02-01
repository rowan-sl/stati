extern crate stati;

use std::{thread::sleep, time::Duration};

use stati::{BarManager, bars};


fn main() {
    let mut bman = BarManager::new();
    let mut b1 = bman.new_bar::<bars::SimpleBar>("bar1".into());
    for i in 0..=50 {
        b1.set_progress(i);
        stati::println!(bman, "Progressed to {} in the first section", i);
        bman.print();
        sleep(Duration::from_millis(50));
    }
    let mut b2 = bman.new_bar::<bars::SimpleBar>("bar2".into());
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
