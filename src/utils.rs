pub fn term_width() -> Option<u16> {
    use terminal_size::{terminal_size, Height, Width};
    let size = terminal_size();
    if let Some((Width(w), Height(_))) = size {
        Some(w)
    } else {
        None
    }
}
