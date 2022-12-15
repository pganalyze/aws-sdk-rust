// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct UntagResourceOutput {}
/// See [`UntagResourceOutput`](crate::output::UntagResourceOutput).
pub mod untag_resource_output {

    /// A builder for [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`UntagResourceOutput`](crate::output::UntagResourceOutput).
        pub fn build(self) -> crate::output::UntagResourceOutput {
            crate::output::UntagResourceOutput {}
        }
    }
}
impl UntagResourceOutput {
    /// Creates a new builder-style object to manufacture [`UntagResourceOutput`](crate::output::UntagResourceOutput).
    pub fn builder() -> crate::output::untag_resource_output::Builder {
        crate::output::untag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct TagResourceOutput {}
/// See [`TagResourceOutput`](crate::output::TagResourceOutput).
pub mod tag_resource_output {

    /// A builder for [`TagResourceOutput`](crate::output::TagResourceOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`TagResourceOutput`](crate::output::TagResourceOutput).
        pub fn build(self) -> crate::output::TagResourceOutput {
            crate::output::TagResourceOutput {}
        }
    }
}
impl TagResourceOutput {
    /// Creates a new builder-style object to manufacture [`TagResourceOutput`](crate::output::TagResourceOutput).
    pub fn builder() -> crate::output::tag_resource_output::Builder {
        crate::output::tag_resource_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct RestoreBackupOutput {
    /// <p>Information on the <code>Backup</code> object created.</p>
    #[doc(hidden)]
    pub backup: std::option::Option<crate::model::Backup>,
}
impl RestoreBackupOutput {
    /// <p>Information on the <code>Backup</code> object created.</p>
    pub fn backup(&self) -> std::option::Option<&crate::model::Backup> {
        self.backup.as_ref()
    }
}
/// See [`RestoreBackupOutput`](crate::output::RestoreBackupOutput).
pub mod restore_backup_output {

    /// A builder for [`RestoreBackupOutput`](crate::output::RestoreBackupOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) backup: std::option::Option<crate::model::Backup>,
    }
    impl Builder {
        /// <p>Information on the <code>Backup</code> object created.</p>
        pub fn backup(mut self, input: crate::model::Backup) -> Self {
            self.backup = Some(input);
            self
        }
        /// <p>Information on the <code>Backup</code> object created.</p>
        pub fn set_backup(mut self, input: std::option::Option<crate::model::Backup>) -> Self {
            self.backup = input;
            self
        }
        /// Consumes the builder and constructs a [`RestoreBackupOutput`](crate::output::RestoreBackupOutput).
        pub fn build(self) -> crate::output::RestoreBackupOutput {
            crate::output::RestoreBackupOutput {
                backup: self.backup,
            }
        }
    }
}
impl RestoreBackupOutput {
    /// Creates a new builder-style object to manufacture [`RestoreBackupOutput`](crate::output::RestoreBackupOutput).
    pub fn builder() -> crate::output::restore_backup_output::Builder {
        crate::output::restore_backup_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ModifyClusterOutput {
    /// <p>Contains information about an AWS CloudHSM cluster.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<crate::model::Cluster>,
}
impl ModifyClusterOutput {
    /// <p>Contains information about an AWS CloudHSM cluster.</p>
    pub fn cluster(&self) -> std::option::Option<&crate::model::Cluster> {
        self.cluster.as_ref()
    }
}
/// See [`ModifyClusterOutput`](crate::output::ModifyClusterOutput).
pub mod modify_cluster_output {

    /// A builder for [`ModifyClusterOutput`](crate::output::ModifyClusterOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) cluster: std::option::Option<crate::model::Cluster>,
    }
    impl Builder {
        /// <p>Contains information about an AWS CloudHSM cluster.</p>
        pub fn cluster(mut self, input: crate::model::Cluster) -> Self {
            self.cluster = Some(input);
            self
        }
        /// <p>Contains information about an AWS CloudHSM cluster.</p>
        pub fn set_cluster(mut self, input: std::option::Option<crate::model::Cluster>) -> Self {
            self.cluster = input;
            self
        }
        /// Consumes the builder and constructs a [`ModifyClusterOutput`](crate::output::ModifyClusterOutput).
        pub fn build(self) -> crate::output::ModifyClusterOutput {
            crate::output::ModifyClusterOutput {
                cluster: self.cluster,
            }
        }
    }
}
impl ModifyClusterOutput {
    /// Creates a new builder-style object to manufacture [`ModifyClusterOutput`](crate::output::ModifyClusterOutput).
    pub fn builder() -> crate::output::modify_cluster_output::Builder {
        crate::output::modify_cluster_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ModifyBackupAttributesOutput {
    /// <p>Contains information about a backup of an AWS CloudHSM cluster. All backup objects contain the <code>BackupId</code>, <code>BackupState</code>, <code>ClusterId</code>, and <code>CreateTimestamp</code> parameters. Backups that were copied into a destination region additionally contain the <code>CopyTimestamp</code>, <code>SourceBackup</code>, <code>SourceCluster</code>, and <code>SourceRegion</code> parameters. A backup that is pending deletion will include the <code>DeleteTimestamp</code> parameter.</p>
    #[doc(hidden)]
    pub backup: std::option::Option<crate::model::Backup>,
}
impl ModifyBackupAttributesOutput {
    /// <p>Contains information about a backup of an AWS CloudHSM cluster. All backup objects contain the <code>BackupId</code>, <code>BackupState</code>, <code>ClusterId</code>, and <code>CreateTimestamp</code> parameters. Backups that were copied into a destination region additionally contain the <code>CopyTimestamp</code>, <code>SourceBackup</code>, <code>SourceCluster</code>, and <code>SourceRegion</code> parameters. A backup that is pending deletion will include the <code>DeleteTimestamp</code> parameter.</p>
    pub fn backup(&self) -> std::option::Option<&crate::model::Backup> {
        self.backup.as_ref()
    }
}
/// See [`ModifyBackupAttributesOutput`](crate::output::ModifyBackupAttributesOutput).
pub mod modify_backup_attributes_output {

    /// A builder for [`ModifyBackupAttributesOutput`](crate::output::ModifyBackupAttributesOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) backup: std::option::Option<crate::model::Backup>,
    }
    impl Builder {
        /// <p>Contains information about a backup of an AWS CloudHSM cluster. All backup objects contain the <code>BackupId</code>, <code>BackupState</code>, <code>ClusterId</code>, and <code>CreateTimestamp</code> parameters. Backups that were copied into a destination region additionally contain the <code>CopyTimestamp</code>, <code>SourceBackup</code>, <code>SourceCluster</code>, and <code>SourceRegion</code> parameters. A backup that is pending deletion will include the <code>DeleteTimestamp</code> parameter.</p>
        pub fn backup(mut self, input: crate::model::Backup) -> Self {
            self.backup = Some(input);
            self
        }
        /// <p>Contains information about a backup of an AWS CloudHSM cluster. All backup objects contain the <code>BackupId</code>, <code>BackupState</code>, <code>ClusterId</code>, and <code>CreateTimestamp</code> parameters. Backups that were copied into a destination region additionally contain the <code>CopyTimestamp</code>, <code>SourceBackup</code>, <code>SourceCluster</code>, and <code>SourceRegion</code> parameters. A backup that is pending deletion will include the <code>DeleteTimestamp</code> parameter.</p>
        pub fn set_backup(mut self, input: std::option::Option<crate::model::Backup>) -> Self {
            self.backup = input;
            self
        }
        /// Consumes the builder and constructs a [`ModifyBackupAttributesOutput`](crate::output::ModifyBackupAttributesOutput).
        pub fn build(self) -> crate::output::ModifyBackupAttributesOutput {
            crate::output::ModifyBackupAttributesOutput {
                backup: self.backup,
            }
        }
    }
}
impl ModifyBackupAttributesOutput {
    /// Creates a new builder-style object to manufacture [`ModifyBackupAttributesOutput`](crate::output::ModifyBackupAttributesOutput).
    pub fn builder() -> crate::output::modify_backup_attributes_output::Builder {
        crate::output::modify_backup_attributes_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct ListTagsOutput {
    /// <p>A list of tags.</p>
    #[doc(hidden)]
    pub tag_list: std::option::Option<std::vec::Vec<crate::model::Tag>>,
    /// <p>An opaque string that indicates that the response contains only a subset of tags. Use this value in a subsequent <code>ListTags</code> request to get more tags.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl ListTagsOutput {
    /// <p>A list of tags.</p>
    pub fn tag_list(&self) -> std::option::Option<&[crate::model::Tag]> {
        self.tag_list.as_deref()
    }
    /// <p>An opaque string that indicates that the response contains only a subset of tags. Use this value in a subsequent <code>ListTags</code> request to get more tags.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`ListTagsOutput`](crate::output::ListTagsOutput).
pub mod list_tags_output {

    /// A builder for [`ListTagsOutput`](crate::output::ListTagsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) tag_list: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `tag_list`.
        ///
        /// To override the contents of this collection use [`set_tag_list`](Self::set_tag_list).
        ///
        /// <p>A list of tags.</p>
        pub fn tag_list(mut self, input: crate::model::Tag) -> Self {
            let mut v = self.tag_list.unwrap_or_default();
            v.push(input);
            self.tag_list = Some(v);
            self
        }
        /// <p>A list of tags.</p>
        pub fn set_tag_list(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Tag>>,
        ) -> Self {
            self.tag_list = input;
            self
        }
        /// <p>An opaque string that indicates that the response contains only a subset of tags. Use this value in a subsequent <code>ListTags</code> request to get more tags.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>An opaque string that indicates that the response contains only a subset of tags. Use this value in a subsequent <code>ListTags</code> request to get more tags.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`ListTagsOutput`](crate::output::ListTagsOutput).
        pub fn build(self) -> crate::output::ListTagsOutput {
            crate::output::ListTagsOutput {
                tag_list: self.tag_list,
                next_token: self.next_token,
            }
        }
    }
}
impl ListTagsOutput {
    /// Creates a new builder-style object to manufacture [`ListTagsOutput`](crate::output::ListTagsOutput).
    pub fn builder() -> crate::output::list_tags_output::Builder {
        crate::output::list_tags_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct InitializeClusterOutput {
    /// <p>The cluster's state.</p>
    #[doc(hidden)]
    pub state: std::option::Option<crate::model::ClusterState>,
    /// <p>A description of the cluster's state.</p>
    #[doc(hidden)]
    pub state_message: std::option::Option<std::string::String>,
}
impl InitializeClusterOutput {
    /// <p>The cluster's state.</p>
    pub fn state(&self) -> std::option::Option<&crate::model::ClusterState> {
        self.state.as_ref()
    }
    /// <p>A description of the cluster's state.</p>
    pub fn state_message(&self) -> std::option::Option<&str> {
        self.state_message.as_deref()
    }
}
/// See [`InitializeClusterOutput`](crate::output::InitializeClusterOutput).
pub mod initialize_cluster_output {

    /// A builder for [`InitializeClusterOutput`](crate::output::InitializeClusterOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) state: std::option::Option<crate::model::ClusterState>,
        pub(crate) state_message: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The cluster's state.</p>
        pub fn state(mut self, input: crate::model::ClusterState) -> Self {
            self.state = Some(input);
            self
        }
        /// <p>The cluster's state.</p>
        pub fn set_state(mut self, input: std::option::Option<crate::model::ClusterState>) -> Self {
            self.state = input;
            self
        }
        /// <p>A description of the cluster's state.</p>
        pub fn state_message(mut self, input: impl Into<std::string::String>) -> Self {
            self.state_message = Some(input.into());
            self
        }
        /// <p>A description of the cluster's state.</p>
        pub fn set_state_message(
            mut self,
            input: std::option::Option<std::string::String>,
        ) -> Self {
            self.state_message = input;
            self
        }
        /// Consumes the builder and constructs a [`InitializeClusterOutput`](crate::output::InitializeClusterOutput).
        pub fn build(self) -> crate::output::InitializeClusterOutput {
            crate::output::InitializeClusterOutput {
                state: self.state,
                state_message: self.state_message,
            }
        }
    }
}
impl InitializeClusterOutput {
    /// Creates a new builder-style object to manufacture [`InitializeClusterOutput`](crate::output::InitializeClusterOutput).
    pub fn builder() -> crate::output::initialize_cluster_output::Builder {
        crate::output::initialize_cluster_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeClustersOutput {
    /// <p>A list of clusters.</p>
    #[doc(hidden)]
    pub clusters: std::option::Option<std::vec::Vec<crate::model::Cluster>>,
    /// <p>An opaque string that indicates that the response contains only a subset of clusters. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeClustersOutput {
    /// <p>A list of clusters.</p>
    pub fn clusters(&self) -> std::option::Option<&[crate::model::Cluster]> {
        self.clusters.as_deref()
    }
    /// <p>An opaque string that indicates that the response contains only a subset of clusters. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`DescribeClustersOutput`](crate::output::DescribeClustersOutput).
pub mod describe_clusters_output {

    /// A builder for [`DescribeClustersOutput`](crate::output::DescribeClustersOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) clusters: std::option::Option<std::vec::Vec<crate::model::Cluster>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `clusters`.
        ///
        /// To override the contents of this collection use [`set_clusters`](Self::set_clusters).
        ///
        /// <p>A list of clusters.</p>
        pub fn clusters(mut self, input: crate::model::Cluster) -> Self {
            let mut v = self.clusters.unwrap_or_default();
            v.push(input);
            self.clusters = Some(v);
            self
        }
        /// <p>A list of clusters.</p>
        pub fn set_clusters(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Cluster>>,
        ) -> Self {
            self.clusters = input;
            self
        }
        /// <p>An opaque string that indicates that the response contains only a subset of clusters. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>An opaque string that indicates that the response contains only a subset of clusters. Use this value in a subsequent <code>DescribeClusters</code> request to get more clusters.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeClustersOutput`](crate::output::DescribeClustersOutput).
        pub fn build(self) -> crate::output::DescribeClustersOutput {
            crate::output::DescribeClustersOutput {
                clusters: self.clusters,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeClustersOutput {
    /// Creates a new builder-style object to manufacture [`DescribeClustersOutput`](crate::output::DescribeClustersOutput).
    pub fn builder() -> crate::output::describe_clusters_output::Builder {
        crate::output::describe_clusters_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DescribeBackupsOutput {
    /// <p>A list of backups.</p>
    #[doc(hidden)]
    pub backups: std::option::Option<std::vec::Vec<crate::model::Backup>>,
    /// <p>An opaque string that indicates that the response contains only a subset of backups. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups.</p>
    #[doc(hidden)]
    pub next_token: std::option::Option<std::string::String>,
}
impl DescribeBackupsOutput {
    /// <p>A list of backups.</p>
    pub fn backups(&self) -> std::option::Option<&[crate::model::Backup]> {
        self.backups.as_deref()
    }
    /// <p>An opaque string that indicates that the response contains only a subset of backups. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups.</p>
    pub fn next_token(&self) -> std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
/// See [`DescribeBackupsOutput`](crate::output::DescribeBackupsOutput).
pub mod describe_backups_output {

    /// A builder for [`DescribeBackupsOutput`](crate::output::DescribeBackupsOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) backups: std::option::Option<std::vec::Vec<crate::model::Backup>>,
        pub(crate) next_token: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// Appends an item to `backups`.
        ///
        /// To override the contents of this collection use [`set_backups`](Self::set_backups).
        ///
        /// <p>A list of backups.</p>
        pub fn backups(mut self, input: crate::model::Backup) -> Self {
            let mut v = self.backups.unwrap_or_default();
            v.push(input);
            self.backups = Some(v);
            self
        }
        /// <p>A list of backups.</p>
        pub fn set_backups(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::Backup>>,
        ) -> Self {
            self.backups = input;
            self
        }
        /// <p>An opaque string that indicates that the response contains only a subset of backups. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups.</p>
        pub fn next_token(mut self, input: impl Into<std::string::String>) -> Self {
            self.next_token = Some(input.into());
            self
        }
        /// <p>An opaque string that indicates that the response contains only a subset of backups. Use this value in a subsequent <code>DescribeBackups</code> request to get more backups.</p>
        pub fn set_next_token(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.next_token = input;
            self
        }
        /// Consumes the builder and constructs a [`DescribeBackupsOutput`](crate::output::DescribeBackupsOutput).
        pub fn build(self) -> crate::output::DescribeBackupsOutput {
            crate::output::DescribeBackupsOutput {
                backups: self.backups,
                next_token: self.next_token,
            }
        }
    }
}
impl DescribeBackupsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeBackupsOutput`](crate::output::DescribeBackupsOutput).
    pub fn builder() -> crate::output::describe_backups_output::Builder {
        crate::output::describe_backups_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteHsmOutput {
    /// <p>The identifier (ID) of the HSM that was deleted.</p>
    #[doc(hidden)]
    pub hsm_id: std::option::Option<std::string::String>,
}
impl DeleteHsmOutput {
    /// <p>The identifier (ID) of the HSM that was deleted.</p>
    pub fn hsm_id(&self) -> std::option::Option<&str> {
        self.hsm_id.as_deref()
    }
}
/// See [`DeleteHsmOutput`](crate::output::DeleteHsmOutput).
pub mod delete_hsm_output {

    /// A builder for [`DeleteHsmOutput`](crate::output::DeleteHsmOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) hsm_id: std::option::Option<std::string::String>,
    }
    impl Builder {
        /// <p>The identifier (ID) of the HSM that was deleted.</p>
        pub fn hsm_id(mut self, input: impl Into<std::string::String>) -> Self {
            self.hsm_id = Some(input.into());
            self
        }
        /// <p>The identifier (ID) of the HSM that was deleted.</p>
        pub fn set_hsm_id(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.hsm_id = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteHsmOutput`](crate::output::DeleteHsmOutput).
        pub fn build(self) -> crate::output::DeleteHsmOutput {
            crate::output::DeleteHsmOutput {
                hsm_id: self.hsm_id,
            }
        }
    }
}
impl DeleteHsmOutput {
    /// Creates a new builder-style object to manufacture [`DeleteHsmOutput`](crate::output::DeleteHsmOutput).
    pub fn builder() -> crate::output::delete_hsm_output::Builder {
        crate::output::delete_hsm_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteClusterOutput {
    /// <p>Information about the cluster that was deleted.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<crate::model::Cluster>,
}
impl DeleteClusterOutput {
    /// <p>Information about the cluster that was deleted.</p>
    pub fn cluster(&self) -> std::option::Option<&crate::model::Cluster> {
        self.cluster.as_ref()
    }
}
/// See [`DeleteClusterOutput`](crate::output::DeleteClusterOutput).
pub mod delete_cluster_output {

    /// A builder for [`DeleteClusterOutput`](crate::output::DeleteClusterOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) cluster: std::option::Option<crate::model::Cluster>,
    }
    impl Builder {
        /// <p>Information about the cluster that was deleted.</p>
        pub fn cluster(mut self, input: crate::model::Cluster) -> Self {
            self.cluster = Some(input);
            self
        }
        /// <p>Information about the cluster that was deleted.</p>
        pub fn set_cluster(mut self, input: std::option::Option<crate::model::Cluster>) -> Self {
            self.cluster = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteClusterOutput`](crate::output::DeleteClusterOutput).
        pub fn build(self) -> crate::output::DeleteClusterOutput {
            crate::output::DeleteClusterOutput {
                cluster: self.cluster,
            }
        }
    }
}
impl DeleteClusterOutput {
    /// Creates a new builder-style object to manufacture [`DeleteClusterOutput`](crate::output::DeleteClusterOutput).
    pub fn builder() -> crate::output::delete_cluster_output::Builder {
        crate::output::delete_cluster_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct DeleteBackupOutput {
    /// <p>Information on the <code>Backup</code> object deleted.</p>
    #[doc(hidden)]
    pub backup: std::option::Option<crate::model::Backup>,
}
impl DeleteBackupOutput {
    /// <p>Information on the <code>Backup</code> object deleted.</p>
    pub fn backup(&self) -> std::option::Option<&crate::model::Backup> {
        self.backup.as_ref()
    }
}
/// See [`DeleteBackupOutput`](crate::output::DeleteBackupOutput).
pub mod delete_backup_output {

    /// A builder for [`DeleteBackupOutput`](crate::output::DeleteBackupOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) backup: std::option::Option<crate::model::Backup>,
    }
    impl Builder {
        /// <p>Information on the <code>Backup</code> object deleted.</p>
        pub fn backup(mut self, input: crate::model::Backup) -> Self {
            self.backup = Some(input);
            self
        }
        /// <p>Information on the <code>Backup</code> object deleted.</p>
        pub fn set_backup(mut self, input: std::option::Option<crate::model::Backup>) -> Self {
            self.backup = input;
            self
        }
        /// Consumes the builder and constructs a [`DeleteBackupOutput`](crate::output::DeleteBackupOutput).
        pub fn build(self) -> crate::output::DeleteBackupOutput {
            crate::output::DeleteBackupOutput {
                backup: self.backup,
            }
        }
    }
}
impl DeleteBackupOutput {
    /// Creates a new builder-style object to manufacture [`DeleteBackupOutput`](crate::output::DeleteBackupOutput).
    pub fn builder() -> crate::output::delete_backup_output::Builder {
        crate::output::delete_backup_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateHsmOutput {
    /// <p>Information about the HSM that was created.</p>
    #[doc(hidden)]
    pub hsm: std::option::Option<crate::model::Hsm>,
}
impl CreateHsmOutput {
    /// <p>Information about the HSM that was created.</p>
    pub fn hsm(&self) -> std::option::Option<&crate::model::Hsm> {
        self.hsm.as_ref()
    }
}
/// See [`CreateHsmOutput`](crate::output::CreateHsmOutput).
pub mod create_hsm_output {

    /// A builder for [`CreateHsmOutput`](crate::output::CreateHsmOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) hsm: std::option::Option<crate::model::Hsm>,
    }
    impl Builder {
        /// <p>Information about the HSM that was created.</p>
        pub fn hsm(mut self, input: crate::model::Hsm) -> Self {
            self.hsm = Some(input);
            self
        }
        /// <p>Information about the HSM that was created.</p>
        pub fn set_hsm(mut self, input: std::option::Option<crate::model::Hsm>) -> Self {
            self.hsm = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateHsmOutput`](crate::output::CreateHsmOutput).
        pub fn build(self) -> crate::output::CreateHsmOutput {
            crate::output::CreateHsmOutput { hsm: self.hsm }
        }
    }
}
impl CreateHsmOutput {
    /// Creates a new builder-style object to manufacture [`CreateHsmOutput`](crate::output::CreateHsmOutput).
    pub fn builder() -> crate::output::create_hsm_output::Builder {
        crate::output::create_hsm_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CreateClusterOutput {
    /// <p>Information about the cluster that was created.</p>
    #[doc(hidden)]
    pub cluster: std::option::Option<crate::model::Cluster>,
}
impl CreateClusterOutput {
    /// <p>Information about the cluster that was created.</p>
    pub fn cluster(&self) -> std::option::Option<&crate::model::Cluster> {
        self.cluster.as_ref()
    }
}
/// See [`CreateClusterOutput`](crate::output::CreateClusterOutput).
pub mod create_cluster_output {

    /// A builder for [`CreateClusterOutput`](crate::output::CreateClusterOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) cluster: std::option::Option<crate::model::Cluster>,
    }
    impl Builder {
        /// <p>Information about the cluster that was created.</p>
        pub fn cluster(mut self, input: crate::model::Cluster) -> Self {
            self.cluster = Some(input);
            self
        }
        /// <p>Information about the cluster that was created.</p>
        pub fn set_cluster(mut self, input: std::option::Option<crate::model::Cluster>) -> Self {
            self.cluster = input;
            self
        }
        /// Consumes the builder and constructs a [`CreateClusterOutput`](crate::output::CreateClusterOutput).
        pub fn build(self) -> crate::output::CreateClusterOutput {
            crate::output::CreateClusterOutput {
                cluster: self.cluster,
            }
        }
    }
}
impl CreateClusterOutput {
    /// Creates a new builder-style object to manufacture [`CreateClusterOutput`](crate::output::CreateClusterOutput).
    pub fn builder() -> crate::output::create_cluster_output::Builder {
        crate::output::create_cluster_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
pub struct CopyBackupToRegionOutput {
    /// <p>Information on the backup that will be copied to the destination region, including CreateTimestamp, SourceBackup, SourceCluster, and Source Region. CreateTimestamp of the destination backup will be the same as that of the source backup.</p>
    /// <p>You will need to use the <code>sourceBackupID</code> returned in this operation to use the <code>DescribeBackups</code> operation on the backup that will be copied to the destination region.</p>
    #[doc(hidden)]
    pub destination_backup: std::option::Option<crate::model::DestinationBackup>,
}
impl CopyBackupToRegionOutput {
    /// <p>Information on the backup that will be copied to the destination region, including CreateTimestamp, SourceBackup, SourceCluster, and Source Region. CreateTimestamp of the destination backup will be the same as that of the source backup.</p>
    /// <p>You will need to use the <code>sourceBackupID</code> returned in this operation to use the <code>DescribeBackups</code> operation on the backup that will be copied to the destination region.</p>
    pub fn destination_backup(&self) -> std::option::Option<&crate::model::DestinationBackup> {
        self.destination_backup.as_ref()
    }
}
/// See [`CopyBackupToRegionOutput`](crate::output::CopyBackupToRegionOutput).
pub mod copy_backup_to_region_output {

    /// A builder for [`CopyBackupToRegionOutput`](crate::output::CopyBackupToRegionOutput).
    #[derive(std::clone::Clone, std::cmp::PartialEq, std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) destination_backup: std::option::Option<crate::model::DestinationBackup>,
    }
    impl Builder {
        /// <p>Information on the backup that will be copied to the destination region, including CreateTimestamp, SourceBackup, SourceCluster, and Source Region. CreateTimestamp of the destination backup will be the same as that of the source backup.</p>
        /// <p>You will need to use the <code>sourceBackupID</code> returned in this operation to use the <code>DescribeBackups</code> operation on the backup that will be copied to the destination region.</p>
        pub fn destination_backup(mut self, input: crate::model::DestinationBackup) -> Self {
            self.destination_backup = Some(input);
            self
        }
        /// <p>Information on the backup that will be copied to the destination region, including CreateTimestamp, SourceBackup, SourceCluster, and Source Region. CreateTimestamp of the destination backup will be the same as that of the source backup.</p>
        /// <p>You will need to use the <code>sourceBackupID</code> returned in this operation to use the <code>DescribeBackups</code> operation on the backup that will be copied to the destination region.</p>
        pub fn set_destination_backup(
            mut self,
            input: std::option::Option<crate::model::DestinationBackup>,
        ) -> Self {
            self.destination_backup = input;
            self
        }
        /// Consumes the builder and constructs a [`CopyBackupToRegionOutput`](crate::output::CopyBackupToRegionOutput).
        pub fn build(self) -> crate::output::CopyBackupToRegionOutput {
            crate::output::CopyBackupToRegionOutput {
                destination_backup: self.destination_backup,
            }
        }
    }
}
impl CopyBackupToRegionOutput {
    /// Creates a new builder-style object to manufacture [`CopyBackupToRegionOutput`](crate::output::CopyBackupToRegionOutput).
    pub fn builder() -> crate::output::copy_backup_to_region_output::Builder {
        crate::output::copy_backup_to_region_output::Builder::default()
    }
}
