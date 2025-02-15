// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterToWorkMail`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`organization_id(impl Into<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::organization_id) / [`set_organization_id(Option<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::set_organization_id):<br>required: **true**<br><p>The identifier for the organization under which the user, group, or resource exists.</p><br>
    ///   - [`entity_id(impl Into<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::entity_id) / [`set_entity_id(Option<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::set_entity_id):<br>required: **true**<br><p>The identifier for the user, group, or resource to be updated.</p>  <p>The identifier can accept <i>UserId, ResourceId, or GroupId</i>, or <i>Username, Resourcename, or Groupname</i>. The following identity formats are available:</p>  <ul>   <li> <p>Entity ID: 12345678-1234-1234-1234-123456789012, r-0123456789a0123456789b0123456789, or S-1-1-12-1234567890-123456789-123456789-1234</p> </li>   <li> <p>Entity name: entity</p> </li>  </ul><br>
    ///   - [`email(impl Into<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::email) / [`set_email(Option<String>)`](crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::set_email):<br>required: **true**<br><p>The email for the user, group, or resource to be updated.</p><br>
    /// - On success, responds with [`RegisterToWorkMailOutput`](crate::operation::register_to_work_mail::RegisterToWorkMailOutput)
    /// - On failure, responds with [`SdkError<RegisterToWorkMailError>`](crate::operation::register_to_work_mail::RegisterToWorkMailError)
    pub fn register_to_work_mail(&self) -> crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder {
        crate::operation::register_to_work_mail::builders::RegisterToWorkMailFluentBuilder::new(self.handle.clone())
    }
}
