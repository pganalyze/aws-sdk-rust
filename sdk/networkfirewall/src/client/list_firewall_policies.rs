// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFirewallPolicies`](crate::operation::list_firewall_policies::builders::ListFirewallPoliciesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_firewall_policies::builders::ListFirewallPoliciesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_firewall_policies::builders::ListFirewallPoliciesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_firewall_policies::builders::ListFirewallPoliciesFluentBuilder::set_next_token):<br>required: **false**<br><p>When you request a list of objects with a <code>MaxResults</code> setting, if the number of objects that are still available for retrieval exceeds the maximum you requested, Network Firewall returns a <code>NextToken</code> value in the response. To retrieve the next batch of objects, use the token returned from the prior request in your next request.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_firewall_policies::builders::ListFirewallPoliciesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_firewall_policies::builders::ListFirewallPoliciesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of objects that you want Network Firewall to return for this request. If more objects are available, in the response, Network Firewall provides a <code>NextToken</code> value that you can use in a subsequent call to get the next batch of objects.</p><br>
    /// - On success, responds with [`ListFirewallPoliciesOutput`](crate::operation::list_firewall_policies::ListFirewallPoliciesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_firewall_policies::ListFirewallPoliciesOutput::next_token): <p>When you request a list of objects with a <code>MaxResults</code> setting, if the number of objects that are still available for retrieval exceeds the maximum you requested, Network Firewall returns a <code>NextToken</code> value in the response. To retrieve the next batch of objects, use the token returned from the prior request in your next request.</p>
    ///   - [`firewall_policies(Option<Vec::<FirewallPolicyMetadata>>)`](crate::operation::list_firewall_policies::ListFirewallPoliciesOutput::firewall_policies): <p>The metadata for the firewall policies. Depending on your setting for max results and the number of firewall policies that you have, this might not be the full list. </p>
    /// - On failure, responds with [`SdkError<ListFirewallPoliciesError>`](crate::operation::list_firewall_policies::ListFirewallPoliciesError)
    pub fn list_firewall_policies(&self) -> crate::operation::list_firewall_policies::builders::ListFirewallPoliciesFluentBuilder {
        crate::operation::list_firewall_policies::builders::ListFirewallPoliciesFluentBuilder::new(self.handle.clone())
    }
}
