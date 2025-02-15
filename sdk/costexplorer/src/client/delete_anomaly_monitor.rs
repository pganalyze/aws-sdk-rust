// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteAnomalyMonitor`](crate::operation::delete_anomaly_monitor::builders::DeleteAnomalyMonitorFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`monitor_arn(impl Into<String>)`](crate::operation::delete_anomaly_monitor::builders::DeleteAnomalyMonitorFluentBuilder::monitor_arn) / [`set_monitor_arn(Option<String>)`](crate::operation::delete_anomaly_monitor::builders::DeleteAnomalyMonitorFluentBuilder::set_monitor_arn):<br>required: **true**<br><p>The unique identifier of the cost anomaly monitor that you want to delete. </p><br>
    /// - On success, responds with [`DeleteAnomalyMonitorOutput`](crate::operation::delete_anomaly_monitor::DeleteAnomalyMonitorOutput)
    /// - On failure, responds with [`SdkError<DeleteAnomalyMonitorError>`](crate::operation::delete_anomaly_monitor::DeleteAnomalyMonitorError)
    pub fn delete_anomaly_monitor(&self) -> crate::operation::delete_anomaly_monitor::builders::DeleteAnomalyMonitorFluentBuilder {
        crate::operation::delete_anomaly_monitor::builders::DeleteAnomalyMonitorFluentBuilder::new(self.handle.clone())
    }
}
