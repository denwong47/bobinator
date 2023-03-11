//! Library for interaction with the [Bob] HR resource management panel.
//!
//! [Bob]: https://app.hibob.com/

pub mod app;

mod common;
pub use common::*;

mod engine;
pub use engine::*;

mod structs;
pub use structs::*;

mod traits;
pub use traits::*;

pub mod bob;
