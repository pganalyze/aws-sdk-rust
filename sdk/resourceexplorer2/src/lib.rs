#![allow(deprecated)]
#![allow(clippy::module_inception)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::should_implement_trait)]
#![allow(clippy::blacklisted_name)]
#![allow(clippy::vec_init_then_push)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_return)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Amazon Web Services Resource Explorer is a resource search and discovery service. By using Resource Explorer, you can
//! explore your resources using an internet search engine-like experience. Examples of
//! resources include Amazon Relational Database Service (Amazon RDS) instances, Amazon Simple Storage Service (Amazon S3) buckets, or Amazon DynamoDB
//! tables. You can search for your resources using resource metadata like names, tags, and
//! IDs. Resource Explorer can search across all of the Amazon Web Services Regions in your account in which you turn
//! the service on, to simplify your cross-Region workloads.</p>
//! <p>Resource Explorer scans the resources in each of the Amazon Web Services Regions in your Amazon Web Services account in which
//! you turn on Resource Explorer. Resource Explorer <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/getting-started-terms-and-concepts.html#term-index">creates
//! and maintains an index</a> in each Region, with the details of that Region's
//! resources.</p>
//! <p>You can <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-aggregator-region.html">search across all of the
//! indexed Regions in your account</a> by designating one of your Amazon Web Services Regions to
//! contain the aggregator index for the account. When you <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/manage-aggregator-region-turn-on.html">promote a local index
//! in a Region to become the aggregator index for the account</a>, Resource Explorer automatically
//! replicates the index information from all local indexes in the other Regions to the
//! aggregator index. Therefore, the Region with the aggregator index has a copy of all resource
//! information for all Regions in the account where you turned on Resource Explorer. As a result,
//! views in the aggregator index Region include resources from all of the indexed Regions in your
//! account.</p>
//! <p>For more information about Amazon Web Services Resource Explorer, including how to enable and configure the
//! service, see the <a href="https://docs.aws.amazon.com/resource-explorer/latest/userguide/">Amazon Web Services Resource Explorer User Guide</a>.</p>
//!
//! # Crate Organization
//!
//! The entry point for most customers will be [`Client`]. [`Client`] exposes one method for each API offered
//! by the service.
//!
//! Some APIs require complex or nested arguments. These exist in [`model`](crate::model).
//!
//! Lastly, errors that can be returned by the service are contained within [`error`]. [`Error`] defines a meta
//! error encompassing all possible errors that can be returned by the service.
//!
//! The other modules within this crate are not required for normal usage.

// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use error_meta::Error;

#[doc(inline)]
pub use config::Config;

pub use aws_credential_types::Credentials;

pub use aws_types::region::Region;

pub use aws_smithy_http::endpoint::Endpoint;

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

/// Endpoint resolution functionality
pub mod endpoint;

/// All error types that operations can return. Documentation on these types is copied from the model.
pub mod error;

mod error_meta;

/// Input structures for operations. Documentation on these types is copied from the model.
pub mod input;

/// Data structures used by operation inputs/outputs. Documentation on these types is copied from the model.
pub mod model;

/// All operations that this crate can perform.
pub mod operation;

/// Output structures for operations. Documentation on these types is copied from the model.
pub mod output;

/// Data primitives referenced by other data types.
pub mod types;

mod idempotency_token;

pub mod middleware;

mod no_credentials;

mod operation_deser;

mod operation_ser;

/// Paginators for the service
pub mod paginator;

mod json_deser;

mod json_ser;

/// Generated accessors for nested fields
mod lens;

/// Endpoints standard library functions
mod endpoint_lib;

mod json_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("resourceexplorer2", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]
pub use client::Client;
