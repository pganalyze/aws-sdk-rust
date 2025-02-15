// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssumeRoleForPodIdentity`](crate::operation::assume_role_for_pod_identity::builders::AssumeRoleForPodIdentityFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cluster_name(impl Into<String>)`](crate::operation::assume_role_for_pod_identity::builders::AssumeRoleForPodIdentityFluentBuilder::cluster_name) / [`set_cluster_name(Option<String>)`](crate::operation::assume_role_for_pod_identity::builders::AssumeRoleForPodIdentityFluentBuilder::set_cluster_name):<br>required: **true**<br><p>The name of the cluster for the request.</p><br>
    ///   - [`token(impl Into<String>)`](crate::operation::assume_role_for_pod_identity::builders::AssumeRoleForPodIdentityFluentBuilder::token) / [`set_token(Option<String>)`](crate::operation::assume_role_for_pod_identity::builders::AssumeRoleForPodIdentityFluentBuilder::set_token):<br>required: **true**<br><p>The token of the Kubernetes service account for the pod.</p><br>
    /// - On success, responds with [`AssumeRoleForPodIdentityOutput`](crate::operation::assume_role_for_pod_identity::AssumeRoleForPodIdentityOutput) with field(s):
    ///   - [`subject(Option<Subject>)`](crate::operation::assume_role_for_pod_identity::AssumeRoleForPodIdentityOutput::subject): <p>The name of the Kubernetes service account inside the cluster to associate the IAM credentials with.</p>
    ///   - [`audience(String)`](crate::operation::assume_role_for_pod_identity::AssumeRoleForPodIdentityOutput::audience): <p>The identity that is allowed to use the credentials. This value is always <code>pods.eks.amazonaws.com</code>.</p>
    ///   - [`pod_identity_association(Option<PodIdentityAssociation>)`](crate::operation::assume_role_for_pod_identity::AssumeRoleForPodIdentityOutput::pod_identity_association): <p>The Amazon Resource Name (ARN) and ID of the EKS Pod Identity association.</p>
    ///   - [`assumed_role_user(Option<AssumedRoleUser>)`](crate::operation::assume_role_for_pod_identity::AssumeRoleForPodIdentityOutput::assumed_role_user): <p>An object with the permanent IAM role identity and the temporary session name.</p>  <p>The ARN of the IAM role that the temporary credentials authenticate to.</p>  <p>The session name of the temporary session requested to STS. The value is a unique identifier that contains the role ID, a colon (<code>:</code>), and the role session name of the role that is being assumed. The role ID is generated by IAM when the role is created. The role session name part of the value follows this format: <code>eks-<i>clustername</i>-<i>podname</i>-<i>random UUID</i> </code> </p>
    ///   - [`credentials(Option<Credentials>)`](crate::operation::assume_role_for_pod_identity::AssumeRoleForPodIdentityOutput::credentials): <p>The <i>Amazon Web Services Signature Version 4</i> type of temporary credentials.</p>
    /// - On failure, responds with [`SdkError<AssumeRoleForPodIdentityError>`](crate::operation::assume_role_for_pod_identity::AssumeRoleForPodIdentityError)
    pub fn assume_role_for_pod_identity(&self) -> crate::operation::assume_role_for_pod_identity::builders::AssumeRoleForPodIdentityFluentBuilder {
        crate::operation::assume_role_for_pod_identity::builders::AssumeRoleForPodIdentityFluentBuilder::new(self.handle.clone())
    }
}
