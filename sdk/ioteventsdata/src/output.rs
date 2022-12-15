// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListDetectorsOutput {
    /// <p>A list of summary information about the detectors (instances).</p>
    #[doc(hidden)]
    pub detector_summaries: std::option::Option<std::vec::Vec<crate::model::DetectorSummary>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListDetectorsOutput {
    /// <p>A list of summary information about the detectors (instances).</p>
    pub fn detector_summaries(&self) -> std::option::Option<&[crate::model::DetectorSummary]> {
        self.detector_summaries.as_deref()
    }
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListDetectorsOutput`](crate::output::ListDetectorsOutput).
pub mod list_detectors_output {

    /// A builder for [`ListDetectorsOutput`](crate::output::ListDetectorsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) detector_summaries:
            std::option::Option<std::vec::Vec<crate::model::DetectorSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `detector_summaries`.
        ///
        /// To override the contents of this collection use [`set_detector_summaries`](Self::set_detector_summaries).
        ///
        /// <p>A list of summary information about the detectors (instances).</p>
        pub fn detector_summaries(mut self, input: crate::model::DetectorSummary) -> Self {
            let mut v = self.detector_summaries.unwrap_or_default();
            v.push(input);
            self.detector_summaries = Some(v);
            self
        }
        /// <p>A list of summary information about the detectors (instances).</p>
        pub fn set_detector_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::DetectorSummary>>,
        ) -> Self {
            self.detector_summaries = input;
            self
        }
        /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListDetectorsOutput`](crate::output::ListDetectorsOutput).
        pub fn build(self) -> crate::output::ListDetectorsOutput {
            crate::output::ListDetectorsOutput {
                detector_summaries: self.detector_summaries,
                next_token: self.next_token,
            }
        }
    }
}
impl ListDetectorsOutput {
    /// Creates a new builder-style object to manufacture [`ListDetectorsOutput`](crate::output::ListDetectorsOutput).
    pub fn builder() -> crate::output::list_detectors_output::Builder {
        crate::output::list_detectors_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListAlarmsOutput {
    /// <p>A list that summarizes each alarm.</p>
    #[doc(hidden)]
    pub alarm_summaries: std::option::Option<std::vec::Vec<crate::model::AlarmSummary>>,
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListAlarmsOutput {
    /// <p>A list that summarizes each alarm.</p>
    pub fn alarm_summaries(&self) -> std::option::Option<&[crate::model::AlarmSummary]> {
        self.alarm_summaries.as_deref()
    }
    /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListAlarmsOutput`](crate::output::ListAlarmsOutput).
pub mod list_alarms_output {

    /// A builder for [`ListAlarmsOutput`](crate::output::ListAlarmsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) alarm_summaries: std::option::Option<std::vec::Vec<crate::model::AlarmSummary>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `alarm_summaries`.
        ///
        /// To override the contents of this collection use [`set_alarm_summaries`](Self::set_alarm_summaries).
        ///
        /// <p>A list that summarizes each alarm.</p>
        pub fn alarm_summaries(mut self, input: crate::model::AlarmSummary) -> Self {
            let mut v = self.alarm_summaries.unwrap_or_default();
            v.push(input);
            self.alarm_summaries = Some(v);
            self
        }
        /// <p>A list that summarizes each alarm.</p>
        pub fn set_alarm_summaries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::AlarmSummary>>,
        ) -> Self {
            self.alarm_summaries = input;
            self
        }
        /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>The token that you can use to return the next set of results, or <code>null</code> if there are no more results.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListAlarmsOutput`](crate::output::ListAlarmsOutput).
        pub fn build(self) -> crate::output::ListAlarmsOutput {
            crate::output::ListAlarmsOutput {
                alarm_summaries: self.alarm_summaries,
                next_token: self.next_token,
            }
        }
    }
}
impl ListAlarmsOutput {
    /// Creates a new builder-style object to manufacture [`ListAlarmsOutput`](crate::output::ListAlarmsOutput).
    pub fn builder() -> crate::output::list_alarms_output::Builder {
        crate::output::list_alarms_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeDetectorOutput {
    /// <p>Information about the detector (instance).</p>
    #[doc(hidden)]
    pub detector: std::option::Option<crate::model::Detector>,
}
impl DescribeDetectorOutput {
    /// <p>Information about the detector (instance).</p>
    pub fn detector(&self) -> std::option::Option<&crate::model::Detector> {
        self.detector.as_ref()
    }
}
/// See [`DescribeDetectorOutput`](crate::output::DescribeDetectorOutput).
pub mod describe_detector_output {

    /// A builder for [`DescribeDetectorOutput`](crate::output::DescribeDetectorOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) detector: std::option::Option<crate::model::Detector>,
    }
    impl Builder {
        /// <p>Information about the detector (instance).</p>
        pub fn detector(mut self, input: crate::model::Detector) -> Self {
            self.detector = Some(input);
            self
        }
        /// <p>Information about the detector (instance).</p>
        pub fn set_detector(mut self, input: std::option::Option<crate::model::Detector>) -> Self {
            self.detector = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeDetectorOutput`](crate::output::DescribeDetectorOutput).
        pub fn build(self) -> crate::output::DescribeDetectorOutput {
            crate::output::DescribeDetectorOutput {
                detector: self.detector,
            }
        }
    }
}
impl DescribeDetectorOutput {
    /// Creates a new builder-style object to manufacture [`DescribeDetectorOutput`](crate::output::DescribeDetectorOutput).
    pub fn builder() -> crate::output::describe_detector_output::Builder {
        crate::output::describe_detector_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeAlarmOutput {
    /// <p>Contains information about an alarm.</p>
    #[doc(hidden)]
    pub alarm: std::option::Option<crate::model::Alarm>,
}
impl DescribeAlarmOutput {
    /// <p>Contains information about an alarm.</p>
    pub fn alarm(&self) -> std::option::Option<&crate::model::Alarm> {
        self.alarm.as_ref()
    }
}
/// See [`DescribeAlarmOutput`](crate::output::DescribeAlarmOutput).
pub mod describe_alarm_output {

    /// A builder for [`DescribeAlarmOutput`](crate::output::DescribeAlarmOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) alarm: std::option::Option<crate::model::Alarm>,
    }
    impl Builder {
        /// <p>Contains information about an alarm.</p>
        pub fn alarm(mut self, input: crate::model::Alarm) -> Self {
            self.alarm = Some(input);
            self
        }
        /// <p>Contains information about an alarm.</p>
        pub fn set_alarm(mut self, input: std::option::Option<crate::model::Alarm>) -> Self {
            self.alarm = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeAlarmOutput`](crate::output::DescribeAlarmOutput).
        pub fn build(self) -> crate::output::DescribeAlarmOutput {
            crate::output::DescribeAlarmOutput { alarm: self.alarm }
        }
    }
}
impl DescribeAlarmOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAlarmOutput`](crate::output::DescribeAlarmOutput).
    pub fn builder() -> crate::output::describe_alarm_output::Builder {
        crate::output::describe_alarm_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchUpdateDetectorOutput {
    /// <p>A list of those detector updates that resulted in errors. (If an error is listed here, the specific update did not occur.)</p>
    #[doc(hidden)]
    pub batch_update_detector_error_entries:
        std::option::Option<std::vec::Vec<crate::model::BatchUpdateDetectorErrorEntry>>,
}
impl BatchUpdateDetectorOutput {
    /// <p>A list of those detector updates that resulted in errors. (If an error is listed here, the specific update did not occur.)</p>
    pub fn batch_update_detector_error_entries(
        &self,
    ) -> std::option::Option<&[crate::model::BatchUpdateDetectorErrorEntry]> {
        self.batch_update_detector_error_entries.as_deref()
    }
}
/// See [`BatchUpdateDetectorOutput`](crate::output::BatchUpdateDetectorOutput).
pub mod batch_update_detector_output {

    /// A builder for [`BatchUpdateDetectorOutput`](crate::output::BatchUpdateDetectorOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) batch_update_detector_error_entries:
            std::option::Option<std::vec::Vec<crate::model::BatchUpdateDetectorErrorEntry>>,
    }
    impl Builder {
        /// Appends an item to `batch_update_detector_error_entries`.
        ///
        /// To override the contents of this collection use [`set_batch_update_detector_error_entries`](Self::set_batch_update_detector_error_entries).
        ///
        /// <p>A list of those detector updates that resulted in errors. (If an error is listed here, the specific update did not occur.)</p>
        pub fn batch_update_detector_error_entries(
            mut self,
            input: crate::model::BatchUpdateDetectorErrorEntry,
        ) -> Self {
            let mut v = self.batch_update_detector_error_entries.unwrap_or_default();
            v.push(input);
            self.batch_update_detector_error_entries = Some(v);
            self
        }
        /// <p>A list of those detector updates that resulted in errors. (If an error is listed here, the specific update did not occur.)</p>
        pub fn set_batch_update_detector_error_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchUpdateDetectorErrorEntry>>,
        ) -> Self {
            self.batch_update_detector_error_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchUpdateDetectorOutput`](crate::output::BatchUpdateDetectorOutput).
        pub fn build(self) -> crate::output::BatchUpdateDetectorOutput {
            crate::output::BatchUpdateDetectorOutput {
                batch_update_detector_error_entries: self.batch_update_detector_error_entries,
            }
        }
    }
}
impl BatchUpdateDetectorOutput {
    /// Creates a new builder-style object to manufacture [`BatchUpdateDetectorOutput`](crate::output::BatchUpdateDetectorOutput).
    pub fn builder() -> crate::output::batch_update_detector_output::Builder {
        crate::output::batch_update_detector_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchSnoozeAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[doc(hidden)]
    pub error_entries: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
}
impl BatchSnoozeAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    pub fn error_entries(
        &self,
    ) -> std::option::Option<&[crate::model::BatchAlarmActionErrorEntry]> {
        self.error_entries.as_deref()
    }
}
/// See [`BatchSnoozeAlarmOutput`](crate::output::BatchSnoozeAlarmOutput).
pub mod batch_snooze_alarm_output {

    /// A builder for [`BatchSnoozeAlarmOutput`](crate::output::BatchSnoozeAlarmOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) error_entries:
            std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
    }
    impl Builder {
        /// Appends an item to `error_entries`.
        ///
        /// To override the contents of this collection use [`set_error_entries`](Self::set_error_entries).
        ///
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn error_entries(mut self, input: crate::model::BatchAlarmActionErrorEntry) -> Self {
            let mut v = self.error_entries.unwrap_or_default();
            v.push(input);
            self.error_entries = Some(v);
            self
        }
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn set_error_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
        ) -> Self {
            self.error_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchSnoozeAlarmOutput`](crate::output::BatchSnoozeAlarmOutput).
        pub fn build(self) -> crate::output::BatchSnoozeAlarmOutput {
            crate::output::BatchSnoozeAlarmOutput {
                error_entries: self.error_entries,
            }
        }
    }
}
impl BatchSnoozeAlarmOutput {
    /// Creates a new builder-style object to manufacture [`BatchSnoozeAlarmOutput`](crate::output::BatchSnoozeAlarmOutput).
    pub fn builder() -> crate::output::batch_snooze_alarm_output::Builder {
        crate::output::batch_snooze_alarm_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchResetAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[doc(hidden)]
    pub error_entries: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
}
impl BatchResetAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    pub fn error_entries(
        &self,
    ) -> std::option::Option<&[crate::model::BatchAlarmActionErrorEntry]> {
        self.error_entries.as_deref()
    }
}
/// See [`BatchResetAlarmOutput`](crate::output::BatchResetAlarmOutput).
pub mod batch_reset_alarm_output {

    /// A builder for [`BatchResetAlarmOutput`](crate::output::BatchResetAlarmOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) error_entries:
            std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
    }
    impl Builder {
        /// Appends an item to `error_entries`.
        ///
        /// To override the contents of this collection use [`set_error_entries`](Self::set_error_entries).
        ///
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn error_entries(mut self, input: crate::model::BatchAlarmActionErrorEntry) -> Self {
            let mut v = self.error_entries.unwrap_or_default();
            v.push(input);
            self.error_entries = Some(v);
            self
        }
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn set_error_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
        ) -> Self {
            self.error_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchResetAlarmOutput`](crate::output::BatchResetAlarmOutput).
        pub fn build(self) -> crate::output::BatchResetAlarmOutput {
            crate::output::BatchResetAlarmOutput {
                error_entries: self.error_entries,
            }
        }
    }
}
impl BatchResetAlarmOutput {
    /// Creates a new builder-style object to manufacture [`BatchResetAlarmOutput`](crate::output::BatchResetAlarmOutput).
    pub fn builder() -> crate::output::batch_reset_alarm_output::Builder {
        crate::output::batch_reset_alarm_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchPutMessageOutput {
    /// <p>A list of any errors encountered when sending the messages.</p>
    #[doc(hidden)]
    pub batch_put_message_error_entries:
        std::option::Option<std::vec::Vec<crate::model::BatchPutMessageErrorEntry>>,
}
impl BatchPutMessageOutput {
    /// <p>A list of any errors encountered when sending the messages.</p>
    pub fn batch_put_message_error_entries(
        &self,
    ) -> std::option::Option<&[crate::model::BatchPutMessageErrorEntry]> {
        self.batch_put_message_error_entries.as_deref()
    }
}
/// See [`BatchPutMessageOutput`](crate::output::BatchPutMessageOutput).
pub mod batch_put_message_output {

    /// A builder for [`BatchPutMessageOutput`](crate::output::BatchPutMessageOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) batch_put_message_error_entries:
            std::option::Option<std::vec::Vec<crate::model::BatchPutMessageErrorEntry>>,
    }
    impl Builder {
        /// Appends an item to `batch_put_message_error_entries`.
        ///
        /// To override the contents of this collection use [`set_batch_put_message_error_entries`](Self::set_batch_put_message_error_entries).
        ///
        /// <p>A list of any errors encountered when sending the messages.</p>
        pub fn batch_put_message_error_entries(
            mut self,
            input: crate::model::BatchPutMessageErrorEntry,
        ) -> Self {
            let mut v = self.batch_put_message_error_entries.unwrap_or_default();
            v.push(input);
            self.batch_put_message_error_entries = Some(v);
            self
        }
        /// <p>A list of any errors encountered when sending the messages.</p>
        pub fn set_batch_put_message_error_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchPutMessageErrorEntry>>,
        ) -> Self {
            self.batch_put_message_error_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchPutMessageOutput`](crate::output::BatchPutMessageOutput).
        pub fn build(self) -> crate::output::BatchPutMessageOutput {
            crate::output::BatchPutMessageOutput {
                batch_put_message_error_entries: self.batch_put_message_error_entries,
            }
        }
    }
}
impl BatchPutMessageOutput {
    /// Creates a new builder-style object to manufacture [`BatchPutMessageOutput`](crate::output::BatchPutMessageOutput).
    pub fn builder() -> crate::output::batch_put_message_output::Builder {
        crate::output::batch_put_message_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchEnableAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[doc(hidden)]
    pub error_entries: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
}
impl BatchEnableAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    pub fn error_entries(
        &self,
    ) -> std::option::Option<&[crate::model::BatchAlarmActionErrorEntry]> {
        self.error_entries.as_deref()
    }
}
/// See [`BatchEnableAlarmOutput`](crate::output::BatchEnableAlarmOutput).
pub mod batch_enable_alarm_output {

    /// A builder for [`BatchEnableAlarmOutput`](crate::output::BatchEnableAlarmOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) error_entries:
            std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
    }
    impl Builder {
        /// Appends an item to `error_entries`.
        ///
        /// To override the contents of this collection use [`set_error_entries`](Self::set_error_entries).
        ///
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn error_entries(mut self, input: crate::model::BatchAlarmActionErrorEntry) -> Self {
            let mut v = self.error_entries.unwrap_or_default();
            v.push(input);
            self.error_entries = Some(v);
            self
        }
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn set_error_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
        ) -> Self {
            self.error_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchEnableAlarmOutput`](crate::output::BatchEnableAlarmOutput).
        pub fn build(self) -> crate::output::BatchEnableAlarmOutput {
            crate::output::BatchEnableAlarmOutput {
                error_entries: self.error_entries,
            }
        }
    }
}
impl BatchEnableAlarmOutput {
    /// Creates a new builder-style object to manufacture [`BatchEnableAlarmOutput`](crate::output::BatchEnableAlarmOutput).
    pub fn builder() -> crate::output::batch_enable_alarm_output::Builder {
        crate::output::batch_enable_alarm_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchDisableAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[doc(hidden)]
    pub error_entries: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
}
impl BatchDisableAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    pub fn error_entries(
        &self,
    ) -> std::option::Option<&[crate::model::BatchAlarmActionErrorEntry]> {
        self.error_entries.as_deref()
    }
}
/// See [`BatchDisableAlarmOutput`](crate::output::BatchDisableAlarmOutput).
pub mod batch_disable_alarm_output {

    /// A builder for [`BatchDisableAlarmOutput`](crate::output::BatchDisableAlarmOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) error_entries:
            std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
    }
    impl Builder {
        /// Appends an item to `error_entries`.
        ///
        /// To override the contents of this collection use [`set_error_entries`](Self::set_error_entries).
        ///
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn error_entries(mut self, input: crate::model::BatchAlarmActionErrorEntry) -> Self {
            let mut v = self.error_entries.unwrap_or_default();
            v.push(input);
            self.error_entries = Some(v);
            self
        }
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn set_error_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
        ) -> Self {
            self.error_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchDisableAlarmOutput`](crate::output::BatchDisableAlarmOutput).
        pub fn build(self) -> crate::output::BatchDisableAlarmOutput {
            crate::output::BatchDisableAlarmOutput {
                error_entries: self.error_entries,
            }
        }
    }
}
impl BatchDisableAlarmOutput {
    /// Creates a new builder-style object to manufacture [`BatchDisableAlarmOutput`](crate::output::BatchDisableAlarmOutput).
    pub fn builder() -> crate::output::batch_disable_alarm_output::Builder {
        crate::output::batch_disable_alarm_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchDeleteDetectorOutput {
    /// <p>A list of errors associated with the request, or an empty array (<code>[]</code>) if there are no errors. Each error entry contains a <code>messageId</code> that helps you identify the entry that failed.</p>
    #[doc(hidden)]
    pub batch_delete_detector_error_entries:
        std::option::Option<std::vec::Vec<crate::model::BatchDeleteDetectorErrorEntry>>,
}
impl BatchDeleteDetectorOutput {
    /// <p>A list of errors associated with the request, or an empty array (<code>[]</code>) if there are no errors. Each error entry contains a <code>messageId</code> that helps you identify the entry that failed.</p>
    pub fn batch_delete_detector_error_entries(
        &self,
    ) -> std::option::Option<&[crate::model::BatchDeleteDetectorErrorEntry]> {
        self.batch_delete_detector_error_entries.as_deref()
    }
}
/// See [`BatchDeleteDetectorOutput`](crate::output::BatchDeleteDetectorOutput).
pub mod batch_delete_detector_output {

    /// A builder for [`BatchDeleteDetectorOutput`](crate::output::BatchDeleteDetectorOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) batch_delete_detector_error_entries:
            std::option::Option<std::vec::Vec<crate::model::BatchDeleteDetectorErrorEntry>>,
    }
    impl Builder {
        /// Appends an item to `batch_delete_detector_error_entries`.
        ///
        /// To override the contents of this collection use [`set_batch_delete_detector_error_entries`](Self::set_batch_delete_detector_error_entries).
        ///
        /// <p>A list of errors associated with the request, or an empty array (<code>[]</code>) if there are no errors. Each error entry contains a <code>messageId</code> that helps you identify the entry that failed.</p>
        pub fn batch_delete_detector_error_entries(
            mut self,
            input: crate::model::BatchDeleteDetectorErrorEntry,
        ) -> Self {
            let mut v = self.batch_delete_detector_error_entries.unwrap_or_default();
            v.push(input);
            self.batch_delete_detector_error_entries = Some(v);
            self
        }
        /// <p>A list of errors associated with the request, or an empty array (<code>[]</code>) if there are no errors. Each error entry contains a <code>messageId</code> that helps you identify the entry that failed.</p>
        pub fn set_batch_delete_detector_error_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchDeleteDetectorErrorEntry>>,
        ) -> Self {
            self.batch_delete_detector_error_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchDeleteDetectorOutput`](crate::output::BatchDeleteDetectorOutput).
        pub fn build(self) -> crate::output::BatchDeleteDetectorOutput {
            crate::output::BatchDeleteDetectorOutput {
                batch_delete_detector_error_entries: self.batch_delete_detector_error_entries,
            }
        }
    }
}
impl BatchDeleteDetectorOutput {
    /// Creates a new builder-style object to manufacture [`BatchDeleteDetectorOutput`](crate::output::BatchDeleteDetectorOutput).
    pub fn builder() -> crate::output::batch_delete_detector_output::Builder {
        crate::output::batch_delete_detector_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchAcknowledgeAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    #[doc(hidden)]
    pub error_entries: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
}
impl BatchAcknowledgeAlarmOutput {
    /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
    pub fn error_entries(
        &self,
    ) -> std::option::Option<&[crate::model::BatchAlarmActionErrorEntry]> {
        self.error_entries.as_deref()
    }
}
/// See [`BatchAcknowledgeAlarmOutput`](crate::output::BatchAcknowledgeAlarmOutput).
pub mod batch_acknowledge_alarm_output {

    /// A builder for [`BatchAcknowledgeAlarmOutput`](crate::output::BatchAcknowledgeAlarmOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) error_entries:
            std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
    }
    impl Builder {
        /// Appends an item to `error_entries`.
        ///
        /// To override the contents of this collection use [`set_error_entries`](Self::set_error_entries).
        ///
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn error_entries(mut self, input: crate::model::BatchAlarmActionErrorEntry) -> Self {
            let mut v = self.error_entries.unwrap_or_default();
            v.push(input);
            self.error_entries = Some(v);
            self
        }
        /// <p>A list of errors associated with the request, or <code>null</code> if there are no errors. Each error entry contains an entry ID that helps you identify the entry that failed.</p>
        pub fn set_error_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchAlarmActionErrorEntry>>,
        ) -> Self {
            self.error_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchAcknowledgeAlarmOutput`](crate::output::BatchAcknowledgeAlarmOutput).
        pub fn build(self) -> crate::output::BatchAcknowledgeAlarmOutput {
            crate::output::BatchAcknowledgeAlarmOutput {
                error_entries: self.error_entries,
            }
        }
    }
}
impl BatchAcknowledgeAlarmOutput {
    /// Creates a new builder-style object to manufacture [`BatchAcknowledgeAlarmOutput`](crate::output::BatchAcknowledgeAlarmOutput).
    pub fn builder() -> crate::output::batch_acknowledge_alarm_output::Builder {
        crate::output::batch_acknowledge_alarm_output::Builder::default()
    }
}
