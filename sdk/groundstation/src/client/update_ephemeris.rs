// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateEphemeris`](crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`ephemeris_id(impl Into<String>)`](crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder::ephemeris_id) / [`set_ephemeris_id(Option<String>)`](crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder::set_ephemeris_id):<br>required: **true**<br><p>The AWS Ground Station ephemeris ID.</p><br>
    ///   - [`enabled(bool)`](crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder::enabled) / [`set_enabled(Option<bool>)`](crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder::set_enabled):<br>required: **true**<br><p>Whether the ephemeris is enabled or not. Changing this value will not require the ephemeris to be re-validated.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder::set_name):<br>required: **false**<br><p>A name string associated with the ephemeris. Used as a human-readable identifier for the ephemeris.</p><br>
    ///   - [`priority(i32)`](crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder::priority) / [`set_priority(Option<i32>)`](crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder::set_priority):<br>required: **false**<br><p>Customer-provided priority score to establish the order in which overlapping ephemerides should be used.</p>  <p>The default for customer-provided ephemeris priority is 1, and higher numbers take precedence.</p>  <p>Priority must be 1 or greater</p><br>
    /// - On success, responds with [`UpdateEphemerisOutput`](crate::operation::update_ephemeris::UpdateEphemerisOutput) with field(s):
    ///   - [`ephemeris_id(Option<String>)`](crate::operation::update_ephemeris::UpdateEphemerisOutput::ephemeris_id): <p>The AWS Ground Station ephemeris ID.</p>
    /// - On failure, responds with [`SdkError<UpdateEphemerisError>`](crate::operation::update_ephemeris::UpdateEphemerisError)
    pub fn update_ephemeris(&self) -> crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder {
        crate::operation::update_ephemeris::builders::UpdateEphemerisFluentBuilder::new(self.handle.clone())
    }
}
