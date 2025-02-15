/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_config::types::ResourceType;
use aws_sdk_config::{config::Region, meta::PKG_VERSION, Client, Error};
use clap::Parser;

#[derive(Debug, Parser)]
struct Opt {
    /// The AWS Region.
    #[structopt(short, long)]
    region: Option<String>,

    /// The resource id.
    #[structopt(long)]
    resource_id: String,

    /// The resource type, eg. "AWS::EC2::SecurityGroup"
    #[structopt(long)]
    resource_type: String,

    /// Whether to display additional information.
    #[structopt(short, long)]
    verbose: bool,
}

// Retrieves the configuration history for a resource.
// snippet-start:[config.rust.config-helloworld]
async fn get_history(client: &Client, id: &str, res: ResourceType) -> Result<(), Error> {
    let rsp = client
        .get_resource_config_history()
        .resource_id(id)
        .resource_type(res)
        .send()
        .await?;

    println!("configuration history for {}:", id);

    for item in rsp.configuration_items() {
        println!("item: {:?}", item);
    }

    Ok(())
}
// snippet-end:[config.rust.config-helloworld]

/// Lists the configuration history for a resource in the Region.
///
/// NOTE: AWS Config must be enabled to discover resources
/// # Arguments
///
/// * `[--resource_id RESOURCE-ID]` - The ID of the resource.
/// * `[--resource_type RESOURCE-TYPE]` - The type of resource, such as **AWS::EC2::SecurityGroup**.
/// * `[-r REGION]` - The AWS Region in which the client is created.
///   If not supplied, uses the value of the **AWS_REGION** environment variable.
///   If the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();
    let Opt {
        region,
        resource_id,
        resource_type,
        verbose,
    } = Opt::parse();

    let region_provider = RegionProviderChain::first_try(region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    println!();

    if verbose {
        println!("Config client version: {}", PKG_VERSION);
        println!(
            "Region:                {}",
            region_provider.region().await.unwrap().as_ref()
        );
        println!();
    }

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    // parse resource type from user input
    let parsed = match ResourceType::try_parse(resource_type.as_str()) {
        Ok(parsed) => parsed,
        Err(_) => panic!(
            "unknown resource type: `{}`. Valid resource types: {:#?}",
            &resource_type,
            ResourceType::values()
        ),
    };

    get_history(&client, &resource_id, parsed).await
}
