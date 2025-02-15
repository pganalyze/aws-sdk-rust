// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListRotationShifts`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`rotation_id(impl Into<String>)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::rotation_id) / [`set_rotation_id(Option<String>)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::set_rotation_id):<br>required: **true**<br><p>The Amazon Resource Name (ARN) of the rotation to retrieve shift information about. </p><br>
    ///   - [`start_time(DateTime)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::start_time) / [`set_start_time(Option<DateTime>)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::set_start_time):<br>required: **false**<br><p>The date and time for the beginning of the time range to list shifts for.</p><br>
    ///   - [`end_time(DateTime)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::end_time) / [`set_end_time(Option<DateTime>)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::set_end_time):<br>required: **true**<br><p>The date and time for the end of the time range to list shifts for.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::set_next_token):<br>required: **false**<br><p>A token to start the list. Use this token to get the next set of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return for this call. The call also returns a token that you can specify in a subsequent call to get the next set of results.</p><br>
    /// - On success, responds with [`ListRotationShiftsOutput`](crate::operation::list_rotation_shifts::ListRotationShiftsOutput) with field(s):
    ///   - [`rotation_shifts(Option<Vec::<RotationShift>>)`](crate::operation::list_rotation_shifts::ListRotationShiftsOutput::rotation_shifts): <p>Information about shifts that meet the filter criteria.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_rotation_shifts::ListRotationShiftsOutput::next_token): <p>The token for the next set of items to return. Use this token to get the next set of results.</p>
    /// - On failure, responds with [`SdkError<ListRotationShiftsError>`](crate::operation::list_rotation_shifts::ListRotationShiftsError)
    pub fn list_rotation_shifts(&self) -> crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder {
        crate::operation::list_rotation_shifts::builders::ListRotationShiftsFluentBuilder::new(self.handle.clone())
    }
}
