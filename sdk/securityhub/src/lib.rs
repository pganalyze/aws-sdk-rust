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
#![allow(rustdoc::bare_urls)]
#![warn(missing_docs)]
//! <p>Security Hub provides you with a comprehensive view of the security state of your Amazon Web Services environment and resources. It also provides you with the readiness status
//! of your environment based on controls from supported security standards. Security Hub collects
//! security data from Amazon Web Services accounts, services, and integrated third-party products and helps
//! you analyze security trends in your environment to identify the highest priority security
//! issues. For more information about Security Hub, see the <a href="https://docs.aws.amazon.com/securityhub/latest/userguide/what-is-securityhub.html">
//! <i>Security HubUser
//! Guide</i>
//! </a>.</p>
//! <p>When you use operations in the Security Hub API, the requests are executed only in the Amazon Web Services
//! Region that is currently active or in the specific Amazon Web Services Region that you specify in your
//! request. Any configuration or settings change that results from the operation is applied
//! only to that Region. To make the same change in other Regions, execute the same command for
//! each Region to apply the change to.</p>
//! <p>For example, if your Region is set to <code>us-west-2</code>, when you use <code>CreateMembers</code> to add a member account to Security Hub, the association of
//! the member account with the administrator account is created only in the <code>us-west-2</code>
//! Region. Security Hub must be enabled for the member account in the same Region that the invitation
//! was sent from.</p>
//! <p>The following throttling limits apply to using Security Hub API operations.</p>
//! <ul>
//! <li>
//! <p>
//! <code>BatchEnableStandards</code> - <code>RateLimit</code> of 1
//! request per second, <code>BurstLimit</code> of 1 request per second.</p>
//! </li>
//! <li>
//! <p>
//! <code>GetFindings</code> - <code>RateLimit</code> of 3 requests per second.
//! <code>BurstLimit</code> of 6 requests per second.</p>
//! </li>
//! <li>
//! <p>
//! <code>BatchImportFindings</code> - <code>RateLimit</code> of 10 requests per second.
//! <code>BurstLimit</code> of 30 requests per second.</p>
//! </li>
//! <li>
//! <p>
//! <code>BatchUpdateFindings</code> - <code>RateLimit</code> of 10 requests per second.
//! <code>BurstLimit</code> of 30 requests per second.</p>
//! </li>
//! <li>
//! <p>
//! <code>UpdateStandardsControl</code> - <code>RateLimit</code> of
//! 1 request per second, <code>BurstLimit</code> of 5 requests per second.</p>
//! </li>
//! <li>
//! <p>All other operations - <code>RateLimit</code> of 10 requests per second.
//! <code>BurstLimit</code> of 30 requests per second.</p>
//! </li>
//! </ul>
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

/// Client and fluent builders for calling the service.
pub mod client;

/// Configuration for the service.
pub mod config;

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

mod aws_endpoint;

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

mod json_errors;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("securityhub", PKG_VERSION);
pub use aws_smithy_http::endpoint::Endpoint;
pub use aws_types::app_name::AppName;
pub use aws_types::region::Region;
pub use aws_types::Credentials;
#[doc(inline)]
pub use client::Client;
