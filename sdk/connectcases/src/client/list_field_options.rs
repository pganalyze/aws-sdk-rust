// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListFieldOptions`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`domain_id(impl Into<String>)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::domain_id) / [`set_domain_id(Option<String>)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::set_domain_id):<br>required: **true**<br><p>The unique identifier of the Cases domain. </p><br>
    ///   - [`field_id(impl Into<String>)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::field_id) / [`set_field_id(Option<String>)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::set_field_id):<br>required: **true**<br><p>The unique identifier of a field.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return per page.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results.</p><br>
    ///   - [`values(impl Into<String>)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::values) / [`set_values(Option<Vec::<String>>)`](crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::set_values):<br>required: **false**<br><p>A list of <code>FieldOption</code> values to filter on for <code>ListFieldOptions</code>.</p><br>
    /// - On success, responds with [`ListFieldOptionsOutput`](crate::operation::list_field_options::ListFieldOptionsOutput) with field(s):
    ///   - [`options(Vec::<FieldOption>)`](crate::operation::list_field_options::ListFieldOptionsOutput::options): <p>A list of <code>FieldOption</code> objects.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_field_options::ListFieldOptionsOutput::next_token): <p>The token for the next set of results. This is null if there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<ListFieldOptionsError>`](crate::operation::list_field_options::ListFieldOptionsError)
    pub fn list_field_options(&self) -> crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder {
        crate::operation::list_field_options::builders::ListFieldOptionsFluentBuilder::new(self.handle.clone())
    }
}
