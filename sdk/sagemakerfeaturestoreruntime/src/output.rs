// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct PutRecordOutput {}
/// See [`PutRecordOutput`](crate::output::PutRecordOutput).
pub mod put_record_output {

    /// A builder for [`PutRecordOutput`](crate::output::PutRecordOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`PutRecordOutput`](crate::output::PutRecordOutput).
        pub fn build(self) -> crate::output::PutRecordOutput {
            crate::output::PutRecordOutput {}
        }
    }
}
impl PutRecordOutput {
    /// Creates a new builder-style object to manufacture [`PutRecordOutput`](crate::output::PutRecordOutput).
    pub fn builder() -> crate::output::put_record_output::Builder {
        crate::output::put_record_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct GetRecordOutput {
    /// <p>The record you requested. A list of <code>FeatureValues</code>.</p>
    #[doc(hidden)]
    pub record: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
}
impl GetRecordOutput {
    /// <p>The record you requested. A list of <code>FeatureValues</code>.</p>
    pub fn record(&self) -> std::option::Option<&[crate::model::FeatureValue]> {
        self.record.as_deref()
    }
}
/// See [`GetRecordOutput`](crate::output::GetRecordOutput).
pub mod get_record_output {

    /// A builder for [`GetRecordOutput`](crate::output::GetRecordOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) record: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
    }
    impl Builder {
        /// Appends an item to `record`.
        ///
        /// To override the contents of this collection use [`set_record`](Self::set_record).
        ///
        /// <p>The record you requested. A list of <code>FeatureValues</code>.</p>
        pub fn record(mut self, input: crate::model::FeatureValue) -> Self {
            let mut v = self.record.unwrap_or_default();
            v.push(input);
            self.record = Some(v);
            self
        }
        /// <p>The record you requested. A list of <code>FeatureValues</code>.</p>
        pub fn set_record(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FeatureValue>>,
        ) -> Self {
            self.record = input;
            self
        }
        /// Consumes the builder and constructs a [`GetRecordOutput`](crate::output::GetRecordOutput).
        pub fn build(self) -> crate::output::GetRecordOutput {
            crate::output::GetRecordOutput {
                record: self.record,
            }
        }
    }
}
impl GetRecordOutput {
    /// Creates a new builder-style object to manufacture [`GetRecordOutput`](crate::output::GetRecordOutput).
    pub fn builder() -> crate::output::get_record_output::Builder {
        crate::output::get_record_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteRecordOutput {}
/// See [`DeleteRecordOutput`](crate::output::DeleteRecordOutput).
pub mod delete_record_output {

    /// A builder for [`DeleteRecordOutput`](crate::output::DeleteRecordOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DeleteRecordOutput`](crate::output::DeleteRecordOutput).
        pub fn build(self) -> crate::output::DeleteRecordOutput {
            crate::output::DeleteRecordOutput {}
        }
    }
}
impl DeleteRecordOutput {
    /// Creates a new builder-style object to manufacture [`DeleteRecordOutput`](crate::output::DeleteRecordOutput).
    pub fn builder() -> crate::output::delete_record_output::Builder {
        crate::output::delete_record_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct BatchGetRecordOutput {
    /// <p>A list of Records you requested to be retrieved in batch.</p>
    #[doc(hidden)]
    pub records: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordResultDetail>>,
    /// <p>A list of errors that have occured when retrieving a batch of Records.</p>
    #[doc(hidden)]
    pub errors: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordError>>,
    /// <p>A unprocessed list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name.</p>
    #[doc(hidden)]
    pub unprocessed_identifiers:
        std::option::Option<std::vec::Vec<crate::model::BatchGetRecordIdentifier>>,
}
impl BatchGetRecordOutput {
    /// <p>A list of Records you requested to be retrieved in batch.</p>
    pub fn records(&self) -> std::option::Option<&[crate::model::BatchGetRecordResultDetail]> {
        self.records.as_deref()
    }
    /// <p>A list of errors that have occured when retrieving a batch of Records.</p>
    pub fn errors(&self) -> std::option::Option<&[crate::model::BatchGetRecordError]> {
        self.errors.as_deref()
    }
    /// <p>A unprocessed list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name.</p>
    pub fn unprocessed_identifiers(
        &self,
    ) -> std::option::Option<&[crate::model::BatchGetRecordIdentifier]> {
        self.unprocessed_identifiers.as_deref()
    }
}
/// See [`BatchGetRecordOutput`](crate::output::BatchGetRecordOutput).
pub mod batch_get_record_output {

    /// A builder for [`BatchGetRecordOutput`](crate::output::BatchGetRecordOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) records:
            std::option::Option<std::vec::Vec<crate::model::BatchGetRecordResultDetail>>,
        pub(crate) errors: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordError>>,
        pub(crate) unprocessed_identifiers:
            std::option::Option<std::vec::Vec<crate::model::BatchGetRecordIdentifier>>,
    }
    impl Builder {
        /// Appends an item to `records`.
        ///
        /// To override the contents of this collection use [`set_records`](Self::set_records).
        ///
        /// <p>A list of Records you requested to be retrieved in batch.</p>
        pub fn records(mut self, input: crate::model::BatchGetRecordResultDetail) -> Self {
            let mut v = self.records.unwrap_or_default();
            v.push(input);
            self.records = Some(v);
            self
        }
        /// <p>A list of Records you requested to be retrieved in batch.</p>
        pub fn set_records(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordResultDetail>>,
        ) -> Self {
            self.records = input;
            self
        }
        /// Appends an item to `errors`.
        ///
        /// To override the contents of this collection use [`set_errors`](Self::set_errors).
        ///
        /// <p>A list of errors that have occured when retrieving a batch of Records.</p>
        pub fn errors(mut self, input: crate::model::BatchGetRecordError) -> Self {
            let mut v = self.errors.unwrap_or_default();
            v.push(input);
            self.errors = Some(v);
            self
        }
        /// <p>A list of errors that have occured when retrieving a batch of Records.</p>
        pub fn set_errors(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordError>>,
        ) -> Self {
            self.errors = input;
            self
        }
        /// Appends an item to `unprocessed_identifiers`.
        ///
        /// To override the contents of this collection use [`set_unprocessed_identifiers`](Self::set_unprocessed_identifiers).
        ///
        /// <p>A unprocessed list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name.</p>
        pub fn unprocessed_identifiers(
            mut self,
            input: crate::model::BatchGetRecordIdentifier,
        ) -> Self {
            let mut v = self.unprocessed_identifiers.unwrap_or_default();
            v.push(input);
            self.unprocessed_identifiers = Some(v);
            self
        }
        /// <p>A unprocessed list of <code>FeatureGroup</code> names, with their corresponding <code>RecordIdentifier</code> value, and Feature name.</p>
        pub fn set_unprocessed_identifiers(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::BatchGetRecordIdentifier>>,
        ) -> Self {
            self.unprocessed_identifiers = input;
            self
        }
        /// Consumes the builder and constructs a [`BatchGetRecordOutput`](crate::output::BatchGetRecordOutput).
        pub fn build(self) -> crate::output::BatchGetRecordOutput {
            crate::output::BatchGetRecordOutput {
                records: self.records,
                errors: self.errors,
                unprocessed_identifiers: self.unprocessed_identifiers,
            }
        }
    }
}
impl BatchGetRecordOutput {
    /// Creates a new builder-style object to manufacture [`BatchGetRecordOutput`](crate::output::BatchGetRecordOutput).
    pub fn builder() -> crate::output::batch_get_record_output::Builder {
        crate::output::batch_get_record_output::Builder::default()
    }
}
