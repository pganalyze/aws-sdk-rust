// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The deployment configuration for an endpoint, which contains the desired deployment strategy and rollback configurations.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeploymentConfig {
    /// <p>Update policy for a blue/green deployment. If this update policy is specified, SageMaker creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips traffic to the new fleet according to the specified traffic routing configuration. Only one update policy should be used in the deployment configuration. If no update policy is specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting by default.</p>
    pub blue_green_update_policy: ::std::option::Option<crate::types::BlueGreenUpdatePolicy>,
    /// <p>Automatic rollback configuration for handling endpoint deployment failures and recovery.</p>
    pub auto_rollback_configuration: ::std::option::Option<crate::types::AutoRollbackConfig>,
    /// <p>Specifies a rolling deployment strategy for updating a SageMaker endpoint.</p>
    pub rolling_update_policy: ::std::option::Option<crate::types::RollingUpdatePolicy>,
}
impl DeploymentConfig {
    /// <p>Update policy for a blue/green deployment. If this update policy is specified, SageMaker creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips traffic to the new fleet according to the specified traffic routing configuration. Only one update policy should be used in the deployment configuration. If no update policy is specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting by default.</p>
    pub fn blue_green_update_policy(&self) -> ::std::option::Option<&crate::types::BlueGreenUpdatePolicy> {
        self.blue_green_update_policy.as_ref()
    }
    /// <p>Automatic rollback configuration for handling endpoint deployment failures and recovery.</p>
    pub fn auto_rollback_configuration(&self) -> ::std::option::Option<&crate::types::AutoRollbackConfig> {
        self.auto_rollback_configuration.as_ref()
    }
    /// <p>Specifies a rolling deployment strategy for updating a SageMaker endpoint.</p>
    pub fn rolling_update_policy(&self) -> ::std::option::Option<&crate::types::RollingUpdatePolicy> {
        self.rolling_update_policy.as_ref()
    }
}
impl DeploymentConfig {
    /// Creates a new builder-style object to manufacture [`DeploymentConfig`](crate::types::DeploymentConfig).
    pub fn builder() -> crate::types::builders::DeploymentConfigBuilder {
        crate::types::builders::DeploymentConfigBuilder::default()
    }
}

/// A builder for [`DeploymentConfig`](crate::types::DeploymentConfig).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeploymentConfigBuilder {
    pub(crate) blue_green_update_policy: ::std::option::Option<crate::types::BlueGreenUpdatePolicy>,
    pub(crate) auto_rollback_configuration: ::std::option::Option<crate::types::AutoRollbackConfig>,
    pub(crate) rolling_update_policy: ::std::option::Option<crate::types::RollingUpdatePolicy>,
}
impl DeploymentConfigBuilder {
    /// <p>Update policy for a blue/green deployment. If this update policy is specified, SageMaker creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips traffic to the new fleet according to the specified traffic routing configuration. Only one update policy should be used in the deployment configuration. If no update policy is specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting by default.</p>
    pub fn blue_green_update_policy(mut self, input: crate::types::BlueGreenUpdatePolicy) -> Self {
        self.blue_green_update_policy = ::std::option::Option::Some(input);
        self
    }
    /// <p>Update policy for a blue/green deployment. If this update policy is specified, SageMaker creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips traffic to the new fleet according to the specified traffic routing configuration. Only one update policy should be used in the deployment configuration. If no update policy is specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting by default.</p>
    pub fn set_blue_green_update_policy(mut self, input: ::std::option::Option<crate::types::BlueGreenUpdatePolicy>) -> Self {
        self.blue_green_update_policy = input;
        self
    }
    /// <p>Update policy for a blue/green deployment. If this update policy is specified, SageMaker creates a new fleet during the deployment while maintaining the old fleet. SageMaker flips traffic to the new fleet according to the specified traffic routing configuration. Only one update policy should be used in the deployment configuration. If no update policy is specified, SageMaker uses a blue/green deployment strategy with all at once traffic shifting by default.</p>
    pub fn get_blue_green_update_policy(&self) -> &::std::option::Option<crate::types::BlueGreenUpdatePolicy> {
        &self.blue_green_update_policy
    }
    /// <p>Automatic rollback configuration for handling endpoint deployment failures and recovery.</p>
    pub fn auto_rollback_configuration(mut self, input: crate::types::AutoRollbackConfig) -> Self {
        self.auto_rollback_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Automatic rollback configuration for handling endpoint deployment failures and recovery.</p>
    pub fn set_auto_rollback_configuration(mut self, input: ::std::option::Option<crate::types::AutoRollbackConfig>) -> Self {
        self.auto_rollback_configuration = input;
        self
    }
    /// <p>Automatic rollback configuration for handling endpoint deployment failures and recovery.</p>
    pub fn get_auto_rollback_configuration(&self) -> &::std::option::Option<crate::types::AutoRollbackConfig> {
        &self.auto_rollback_configuration
    }
    /// <p>Specifies a rolling deployment strategy for updating a SageMaker endpoint.</p>
    pub fn rolling_update_policy(mut self, input: crate::types::RollingUpdatePolicy) -> Self {
        self.rolling_update_policy = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies a rolling deployment strategy for updating a SageMaker endpoint.</p>
    pub fn set_rolling_update_policy(mut self, input: ::std::option::Option<crate::types::RollingUpdatePolicy>) -> Self {
        self.rolling_update_policy = input;
        self
    }
    /// <p>Specifies a rolling deployment strategy for updating a SageMaker endpoint.</p>
    pub fn get_rolling_update_policy(&self) -> &::std::option::Option<crate::types::RollingUpdatePolicy> {
        &self.rolling_update_policy
    }
    /// Consumes the builder and constructs a [`DeploymentConfig`](crate::types::DeploymentConfig).
    pub fn build(self) -> crate::types::DeploymentConfig {
        crate::types::DeploymentConfig {
            blue_green_update_policy: self.blue_green_update_policy,
            auto_rollback_configuration: self.auto_rollback_configuration,
            rolling_update_policy: self.rolling_update_policy,
        }
    }
}
