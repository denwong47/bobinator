//! API calls to bob using username/password authentication, followed
//! by cookie session.
//!
//! This uses the reverse engineered API end points as bob's own
//! website. This does not require service account privileges, and can
//! operate on a personal level.

pub mod employee;

mod login;
pub use login::*;

pub mod timeoff;

mod api_token;
pub use api_token::*;
