pub(crate) fn term_width() -> std::io::Result<u16> {
    use termion::terminal_size;
    Ok(terminal_size()?.0)
}