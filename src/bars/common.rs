/// cannot get the term width
#[derive(thiserror::Error, Debug)]
#[error("Failed to get terminal width")]
pub struct TermWidthError;

/// bar does not fit
#[derive(thiserror::Error, Debug)]
#[error("Terinal to small for bar to fit!")]
pub struct TermSizeError;
