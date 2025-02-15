// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateClusterInput {
    /// <p>The name for the new SageMaker HyperPod cluster.</p>
    pub cluster_name: ::std::option::Option<::std::string::String>,
    /// <p>The instance groups to be created in the SageMaker HyperPod cluster.</p>
    pub instance_groups: ::std::option::Option<::std::vec::Vec<crate::types::ClusterInstanceGroupSpecification>>,
    /// <p>Specifies an Amazon Virtual Private Cloud (VPC) that your SageMaker jobs, hosted models, and compute resources have access to. You can control access to and from your resources by configuring a VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html">Give SageMaker Access to Resources in your Amazon VPC</a>. </p>
    pub vpc_config: ::std::option::Option<crate::types::VpcConfig>,
    /// <p>Custom tags for managing the SageMaker HyperPod cluster as an Amazon Web Services resource. You can add tags to your cluster in the same way you add them in other Amazon Web Services services that support tagging. To learn more about tagging Amazon Web Services resources in general, see <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/tagging.html">Tagging Amazon Web Services Resources User Guide</a>.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateClusterInput {
    /// <p>The name for the new SageMaker HyperPod cluster.</p>
    pub fn cluster_name(&self) -> ::std::option::Option<&str> {
        self.cluster_name.as_deref()
    }
    /// <p>The instance groups to be created in the SageMaker HyperPod cluster.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.instance_groups.is_none()`.
    pub fn instance_groups(&self) -> &[crate::types::ClusterInstanceGroupSpecification] {
        self.instance_groups.as_deref().unwrap_or_default()
    }
    /// <p>Specifies an Amazon Virtual Private Cloud (VPC) that your SageMaker jobs, hosted models, and compute resources have access to. You can control access to and from your resources by configuring a VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html">Give SageMaker Access to Resources in your Amazon VPC</a>. </p>
    pub fn vpc_config(&self) -> ::std::option::Option<&crate::types::VpcConfig> {
        self.vpc_config.as_ref()
    }
    /// <p>Custom tags for managing the SageMaker HyperPod cluster as an Amazon Web Services resource. You can add tags to your cluster in the same way you add them in other Amazon Web Services services that support tagging. To learn more about tagging Amazon Web Services resources in general, see <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/tagging.html">Tagging Amazon Web Services Resources User Guide</a>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl CreateClusterInput {
    /// Creates a new builder-style object to manufacture [`CreateClusterInput`](crate::operation::create_cluster::CreateClusterInput).
    pub fn builder() -> crate::operation::create_cluster::builders::CreateClusterInputBuilder {
        crate::operation::create_cluster::builders::CreateClusterInputBuilder::default()
    }
}

/// A builder for [`CreateClusterInput`](crate::operation::create_cluster::CreateClusterInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateClusterInputBuilder {
    pub(crate) cluster_name: ::std::option::Option<::std::string::String>,
    pub(crate) instance_groups: ::std::option::Option<::std::vec::Vec<crate::types::ClusterInstanceGroupSpecification>>,
    pub(crate) vpc_config: ::std::option::Option<crate::types::VpcConfig>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl CreateClusterInputBuilder {
    /// <p>The name for the new SageMaker HyperPod cluster.</p>
    /// This field is required.
    pub fn cluster_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cluster_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name for the new SageMaker HyperPod cluster.</p>
    pub fn set_cluster_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cluster_name = input;
        self
    }
    /// <p>The name for the new SageMaker HyperPod cluster.</p>
    pub fn get_cluster_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.cluster_name
    }
    /// Appends an item to `instance_groups`.
    ///
    /// To override the contents of this collection use [`set_instance_groups`](Self::set_instance_groups).
    ///
    /// <p>The instance groups to be created in the SageMaker HyperPod cluster.</p>
    pub fn instance_groups(mut self, input: crate::types::ClusterInstanceGroupSpecification) -> Self {
        let mut v = self.instance_groups.unwrap_or_default();
        v.push(input);
        self.instance_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>The instance groups to be created in the SageMaker HyperPod cluster.</p>
    pub fn set_instance_groups(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ClusterInstanceGroupSpecification>>) -> Self {
        self.instance_groups = input;
        self
    }
    /// <p>The instance groups to be created in the SageMaker HyperPod cluster.</p>
    pub fn get_instance_groups(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ClusterInstanceGroupSpecification>> {
        &self.instance_groups
    }
    /// <p>Specifies an Amazon Virtual Private Cloud (VPC) that your SageMaker jobs, hosted models, and compute resources have access to. You can control access to and from your resources by configuring a VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html">Give SageMaker Access to Resources in your Amazon VPC</a>. </p>
    pub fn vpc_config(mut self, input: crate::types::VpcConfig) -> Self {
        self.vpc_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies an Amazon Virtual Private Cloud (VPC) that your SageMaker jobs, hosted models, and compute resources have access to. You can control access to and from your resources by configuring a VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html">Give SageMaker Access to Resources in your Amazon VPC</a>. </p>
    pub fn set_vpc_config(mut self, input: ::std::option::Option<crate::types::VpcConfig>) -> Self {
        self.vpc_config = input;
        self
    }
    /// <p>Specifies an Amazon Virtual Private Cloud (VPC) that your SageMaker jobs, hosted models, and compute resources have access to. You can control access to and from your resources by configuring a VPC. For more information, see <a href="https://docs.aws.amazon.com/sagemaker/latest/dg/infrastructure-give-access.html">Give SageMaker Access to Resources in your Amazon VPC</a>. </p>
    pub fn get_vpc_config(&self) -> &::std::option::Option<crate::types::VpcConfig> {
        &self.vpc_config
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Custom tags for managing the SageMaker HyperPod cluster as an Amazon Web Services resource. You can add tags to your cluster in the same way you add them in other Amazon Web Services services that support tagging. To learn more about tagging Amazon Web Services resources in general, see <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/tagging.html">Tagging Amazon Web Services Resources User Guide</a>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Custom tags for managing the SageMaker HyperPod cluster as an Amazon Web Services resource. You can add tags to your cluster in the same way you add them in other Amazon Web Services services that support tagging. To learn more about tagging Amazon Web Services resources in general, see <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/tagging.html">Tagging Amazon Web Services Resources User Guide</a>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Custom tags for managing the SageMaker HyperPod cluster as an Amazon Web Services resource. You can add tags to your cluster in the same way you add them in other Amazon Web Services services that support tagging. To learn more about tagging Amazon Web Services resources in general, see <a href="https://docs.aws.amazon.com/tag-editor/latest/userguide/tagging.html">Tagging Amazon Web Services Resources User Guide</a>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`CreateClusterInput`](crate::operation::create_cluster::CreateClusterInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_cluster::CreateClusterInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_cluster::CreateClusterInput {
            cluster_name: self.cluster_name,
            instance_groups: self.instance_groups,
            vpc_config: self.vpc_config,
            tags: self.tags,
        })
    }
}
