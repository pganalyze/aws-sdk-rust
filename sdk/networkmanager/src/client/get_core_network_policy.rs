// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetCoreNetworkPolicy`](crate::operation::get_core_network_policy::builders::GetCoreNetworkPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`core_network_id(impl Into<String>)`](crate::operation::get_core_network_policy::builders::GetCoreNetworkPolicyFluentBuilder::core_network_id) / [`set_core_network_id(Option<String>)`](crate::operation::get_core_network_policy::builders::GetCoreNetworkPolicyFluentBuilder::set_core_network_id):<br>required: **true**<br><p>The ID of a core network.</p><br>
    ///   - [`policy_version_id(i32)`](crate::operation::get_core_network_policy::builders::GetCoreNetworkPolicyFluentBuilder::policy_version_id) / [`set_policy_version_id(Option<i32>)`](crate::operation::get_core_network_policy::builders::GetCoreNetworkPolicyFluentBuilder::set_policy_version_id):<br>required: **false**<br><p>The ID of a core network policy version.</p><br>
    ///   - [`alias(CoreNetworkPolicyAlias)`](crate::operation::get_core_network_policy::builders::GetCoreNetworkPolicyFluentBuilder::alias) / [`set_alias(Option<CoreNetworkPolicyAlias>)`](crate::operation::get_core_network_policy::builders::GetCoreNetworkPolicyFluentBuilder::set_alias):<br>required: **false**<br><p>The alias of a core network policy </p><br>
    /// - On success, responds with [`GetCoreNetworkPolicyOutput`](crate::operation::get_core_network_policy::GetCoreNetworkPolicyOutput) with field(s):
    ///   - [`core_network_policy(Option<CoreNetworkPolicy>)`](crate::operation::get_core_network_policy::GetCoreNetworkPolicyOutput::core_network_policy): <p>The details about a core network policy.</p>
    /// - On failure, responds with [`SdkError<GetCoreNetworkPolicyError>`](crate::operation::get_core_network_policy::GetCoreNetworkPolicyError)
    pub fn get_core_network_policy(&self) -> crate::operation::get_core_network_policy::builders::GetCoreNetworkPolicyFluentBuilder {
        crate::operation::get_core_network_policy::builders::GetCoreNetworkPolicyFluentBuilder::new(self.handle.clone())
    }
}
