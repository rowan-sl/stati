//! trait imports for stati.
//!
//! please not that this is **only** trait imports,
//! and also renames all items to __stati_<name>
//! as to avoid namespace colision.
//!
//! if you want to use a trait yourself, and not just its methods,
//! import it seperatly

pub use crate::iterator::ProgressTrackingAdaptor as __stati_ProgressTrackingAdaptor;
pub use crate::IsBar as __stati_IsBar;
pub use crate::IsBarWrapper as __stati_IsBarWrapper;
