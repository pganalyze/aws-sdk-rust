// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTagsForResource`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`resource_id(impl Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_resource_id):<br>required: **true**<br><p>Specifies the EFS resource you want to retrieve tags for. You can retrieve tags for EFS file systems and access points using this API endpoint.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_max_results):<br>required: **false**<br><p>(Optional) Specifies the maximum number of tag objects to return in the response. The default value is 100.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::set_next_token):<br>required: **false**<br><p>(Optional) You can use <code>NextToken</code> in a subsequent request to fetch the next page of access point descriptions if the response payload was paginated.</p><br>
    /// - On success, responds with [`ListTagsForResourceOutput`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput) with field(s):
    ///   - [`tags(Option<Vec::<Tag>>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::tags): <p>An array of the tags for the specified EFS resource.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_tags_for_resource::ListTagsForResourceOutput::next_token): <p> <code>NextToken</code> is present if the response payload is paginated. You can use <code>NextToken</code> in a subsequent request to fetch the next page of access point descriptions.</p>
    /// - On failure, responds with [`SdkError<ListTagsForResourceError>`](crate::operation::list_tags_for_resource::ListTagsForResourceError)
    pub fn list_tags_for_resource(&self) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder {
        crate::operation::list_tags_for_resource::builders::ListTagsForResourceFluentBuilder::new(self.handle.clone())
    }
}
