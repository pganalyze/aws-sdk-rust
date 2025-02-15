// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ExecuteProvisionedProductPlan`](crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`accept_language(impl Into<String>)`](crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanFluentBuilder::accept_language) / [`set_accept_language(Option<String>)`](crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanFluentBuilder::set_accept_language):<br>required: **false**<br><p>The language code.</p>  <ul>   <li> <p> <code>jp</code> - Japanese</p> </li>   <li> <p> <code>zh</code> - Chinese</p> </li>  </ul><br>
    ///   - [`plan_id(impl Into<String>)`](crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanFluentBuilder::plan_id) / [`set_plan_id(Option<String>)`](crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanFluentBuilder::set_plan_id):<br>required: **true**<br><p>The plan identifier.</p><br>
    ///   - [`idempotency_token(impl Into<String>)`](crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanFluentBuilder::idempotency_token) / [`set_idempotency_token(Option<String>)`](crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanFluentBuilder::set_idempotency_token):<br>required: **true**<br><p>A unique identifier that you provide to ensure idempotency. If multiple requests differ only by the idempotency token, the same response is returned for each repeated request.</p><br>
    /// - On success, responds with [`ExecuteProvisionedProductPlanOutput`](crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanOutput) with field(s):
    ///   - [`record_detail(Option<RecordDetail>)`](crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanOutput::record_detail): <p>Information about the result of provisioning the product.</p>
    /// - On failure, responds with [`SdkError<ExecuteProvisionedProductPlanError>`](crate::operation::execute_provisioned_product_plan::ExecuteProvisionedProductPlanError)
    pub fn execute_provisioned_product_plan(
        &self,
    ) -> crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanFluentBuilder {
        crate::operation::execute_provisioned_product_plan::builders::ExecuteProvisionedProductPlanFluentBuilder::new(self.handle.clone())
    }
}
