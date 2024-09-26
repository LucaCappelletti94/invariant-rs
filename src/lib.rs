#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![no_std]

mod invariant_eq_macro;
mod invariant_ge_macro;
mod invariant_gt_macro;
mod invariant_le_macro;
mod invariant_lt_macro;
mod invariant_macro;
mod invariant_ne_macro;

/// Module re-exporting the invariant check macros.
pub mod prelude {
    pub use crate::invariant;
    pub use crate::invariant_eq;
    pub use crate::invariant_ge;
    pub use crate::invariant_gt;
    pub use crate::invariant_le;
    pub use crate::invariant_lt;
    pub use crate::invariant_ne;
}
