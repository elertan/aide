//! Support for `QsQuery` from [serde_qs](https://docs.rs/serde_qs) in Aide.
//!
//! Enabling this feature allow you to use `QsQuery` as an input for your handlers to be used within Aide's routing functions.
//! ```no_run
//! use serde::Deserialize;
//! use serde_qs::axum::QsQuery;
//! use aide::axum::IntoApiResponse;
//!
//! #[derive(Deserialize)]
//! struct MyQuery {
//!     sequence: Vec<i32>,
//! }
//!
//! async fn handler(QsQuery(qs): QsQuery<MyQuery>) -> impl IntoApiResponse {
//!     format!("I received the following sequence: {:?}", qs.sequence)
//! }
//! ```

pub mod inputs;