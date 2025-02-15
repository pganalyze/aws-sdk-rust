// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetOperationDetail`](crate::operation::get_operation_detail::builders::GetOperationDetailFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`operation_id(impl Into<String>)`](crate::operation::get_operation_detail::builders::GetOperationDetailFluentBuilder::operation_id) / [`set_operation_id(Option<String>)`](crate::operation::get_operation_detail::builders::GetOperationDetailFluentBuilder::set_operation_id):<br>required: **true**<br><p>The identifier for the operation for which you want to get the status. Route 53 returned the identifier in the response to the original request.</p><br>
    /// - On success, responds with [`GetOperationDetailOutput`](crate::operation::get_operation_detail::GetOperationDetailOutput) with field(s):
    ///   - [`operation_id(Option<String>)`](crate::operation::get_operation_detail::GetOperationDetailOutput::operation_id): <p>The identifier for the operation.</p>
    ///   - [`status(Option<OperationStatus>)`](crate::operation::get_operation_detail::GetOperationDetailOutput::status): <p>The current status of the requested operation in the system.</p>
    ///   - [`message(Option<String>)`](crate::operation::get_operation_detail::GetOperationDetailOutput::message): <p>Detailed information on the status including possible errors.</p>
    ///   - [`domain_name(Option<String>)`](crate::operation::get_operation_detail::GetOperationDetailOutput::domain_name): <p>The name of a domain.</p>
    ///   - [`r#type(Option<OperationType>)`](crate::operation::get_operation_detail::GetOperationDetailOutput::type): <p>The type of operation that was requested.</p>
    ///   - [`submitted_date(Option<DateTime>)`](crate::operation::get_operation_detail::GetOperationDetailOutput::submitted_date): <p>The date when the request was submitted.</p>
    ///   - [`last_updated_date(Option<DateTime>)`](crate::operation::get_operation_detail::GetOperationDetailOutput::last_updated_date): <p> The date when the operation was last updated. </p>
    ///   - [`status_flag(Option<StatusFlag>)`](crate::operation::get_operation_detail::GetOperationDetailOutput::status_flag): <p> Lists any outstanding operations that require customer action. Valid values are:</p>  <ul>   <li> <p> <code>PENDING_ACCEPTANCE</code>: The operation is waiting for acceptance from the account that is receiving the domain.</p> </li>   <li> <p> <code>PENDING_CUSTOMER_ACTION</code>: The operation is waiting for customer action, for example, returning an email.</p> </li>   <li> <p> <code>PENDING_AUTHORIZATION</code>: The operation is waiting for the form of authorization. For more information, see <a href="https://docs.aws.amazon.com/Route53/latest/APIReference/API_domains_ResendOperationAuthorization.html">ResendOperationAuthorization</a>.</p> </li>   <li> <p> <code>PENDING_PAYMENT_VERIFICATION</code>: The operation is waiting for the payment method to validate.</p> </li>   <li> <p> <code>PENDING_SUPPORT_CASE</code>: The operation includes a support case and is waiting for its resolution.</p> </li>  </ul>
    /// - On failure, responds with [`SdkError<GetOperationDetailError>`](crate::operation::get_operation_detail::GetOperationDetailError)
    pub fn get_operation_detail(&self) -> crate::operation::get_operation_detail::builders::GetOperationDetailFluentBuilder {
        crate::operation::get_operation_detail::builders::GetOperationDetailFluentBuilder::new(self.handle.clone())
    }
}
