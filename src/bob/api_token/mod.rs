//! API calls to bob using official API token.
//!
//! This uses the the officially published API end points. It is meant
//! to be used by the company wide service account. Does not work with
//! personal API tokens.
//!
//! This module is currently not used and not guaranteed to work.
//!
mod employee;
pub use employee::*;
