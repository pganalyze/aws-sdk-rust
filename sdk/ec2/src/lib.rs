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
//! <fullname>Amazon Elastic Compute Cloud</fullname>
//! <p>Amazon Elastic Compute Cloud (Amazon EC2) provides secure and resizable computing capacity in the Amazon Web Services Cloud.
//! Using Amazon EC2 eliminates the need to invest in hardware up front, so you can develop and deploy applications
//! faster. Amazon Virtual Private Cloud (Amazon VPC) enables you to provision a logically isolated section of the
//! Amazon Web Services Cloud where you can launch Amazon Web Services resources in a virtual network that you've defined. Amazon Elastic Block Store
//! (Amazon EBS) provides block level storage volumes for use with EC2 instances. EBS volumes are highly available  
//! and reliable storage volumes that can be attached to any running instance and used like a hard drive.</p>
//! <p>To learn more, see the following resources:</p>
//! <ul>
//! <li>
//! <p>Amazon EC2: <a href="http://aws.amazon.com/ec2">Amazon EC2 product page</a>, <a href="https://docs.aws.amazon.com/ec2/index.html">Amazon EC2 documentation</a>
//! </p>
//! </li>
//! <li>
//! <p>Amazon EBS: <a href="http://aws.amazon.com/ebs">Amazon EBS product page</a>, <a href="https://docs.aws.amazon.com/ebs/index.html">Amazon EBS documentation</a>
//! </p>
//! </li>
//! <li>
//! <p>Amazon VPC: <a href="http://aws.amazon.com/vpc">Amazon VPC product page</a>, <a href="https://docs.aws.amazon.com/vpc/index.html">Amazon VPC documentation</a>
//! </p>
//! </li>
//! <li>
//! <p>VPN: <a href="http://aws.amazon.com/vpn">VPN product page</a>, <a href="https://docs.aws.amazon.com/vpn/index.html">VPN documentation</a>
//! </p>
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
//!
//! # Examples
//! Examples can be found [here](https://github.com/awslabs/aws-sdk-rust/tree/main/examples/ec2).

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

/// Generated accessors for nested fields
mod lens;

mod query_ser;

mod xml_deser;

mod ec2_query_errors;

/// Endpoints standard library functions
mod endpoint_lib;

/// Crate version number.
pub static PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
static API_METADATA: aws_http::user_agent::ApiMetadata =
    aws_http::user_agent::ApiMetadata::new("ec2", PKG_VERSION);
pub use aws_types::app_name::AppName;
#[doc(inline)]
pub use client::Client;
