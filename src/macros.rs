
/// the same as [`print!`] from the std lib,
/// but it displays text through the provided [`BarManager`]
/// instead of printing it directly, allowing printing without
/// breaking the progressbar
/// 
/// [`BarManager`]: crate::manager::BarManager
/// [`print!`]: std::print
#[macro_export]
macro_rules! print {
    ($bm:ident, $($arg:tt)*) => ({
        $bm.queue_text(&format!($($arg)*));
    })
}

/// the same as [`println!`] from the std lib,
/// but it displays text through the provided [`BarManager`]
/// instead of printing it directly, allowing printing without
/// breaking the progressbar
/// 
/// [`BarManager`]: crate::manager::BarManager
/// [`println!`]: std::println
#[macro_export]
macro_rules! println {
    ($bm: ident) => ($bm.queue_text("\n"));
    ($bm:ident, $($arg:tt)*) => ({
        $bm.queue_text(&format!("{}\n", format!($($arg)*)));
    })
}

