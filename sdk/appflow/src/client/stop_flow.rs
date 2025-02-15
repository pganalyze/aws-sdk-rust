// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StopFlow`](crate::operation::stop_flow::builders::StopFlowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`flow_name(impl Into<String>)`](crate::operation::stop_flow::builders::StopFlowFluentBuilder::flow_name) / [`set_flow_name(Option<String>)`](crate::operation::stop_flow::builders::StopFlowFluentBuilder::set_flow_name):<br>required: **true**<br><p> The specified name of the flow. Spaces are not allowed. Use underscores (_) or hyphens (-) only. </p><br>
    /// - On success, responds with [`StopFlowOutput`](crate::operation::stop_flow::StopFlowOutput) with field(s):
    ///   - [`flow_arn(Option<String>)`](crate::operation::stop_flow::StopFlowOutput::flow_arn): <p> The flow's Amazon Resource Name (ARN). </p>
    ///   - [`flow_status(Option<FlowStatus>)`](crate::operation::stop_flow::StopFlowOutput::flow_status): <p> Indicates the current status of the flow. </p>
    /// - On failure, responds with [`SdkError<StopFlowError>`](crate::operation::stop_flow::StopFlowError)
    pub fn stop_flow(&self) -> crate::operation::stop_flow::builders::StopFlowFluentBuilder {
        crate::operation::stop_flow::builders::StopFlowFluentBuilder::new(self.handle.clone())
    }
}
