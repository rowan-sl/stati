#[macro_export]
macro_rules! print {
    ($bm:ident, $($arg:tt)*) => ({
        $bm.queue_text(&format!($($arg)*));
    })
}

#[macro_export]
macro_rules! println {
    ($bm: ident) => ($bm.queue_text("\n"));
    ($bm:ident, $($arg:tt)*) => ({
        $bm.queue_text(&format!("{}\n", format!($($arg)*)));
    })
}

