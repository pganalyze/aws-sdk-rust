// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteBucketWebsite`](crate::operation::delete_bucket_website::builders::DeleteBucketWebsiteFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bucket(impl Into<String>)`](crate::operation::delete_bucket_website::builders::DeleteBucketWebsiteFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::delete_bucket_website::builders::DeleteBucketWebsiteFluentBuilder::set_bucket):<br>required: **true**<br><p>The bucket name for which you want to remove the website configuration. </p><br>
    ///   - [`expected_bucket_owner(impl Into<String>)`](crate::operation::delete_bucket_website::builders::DeleteBucketWebsiteFluentBuilder::expected_bucket_owner) / [`set_expected_bucket_owner(Option<String>)`](crate::operation::delete_bucket_website::builders::DeleteBucketWebsiteFluentBuilder::set_expected_bucket_owner):<br>required: **false**<br><p>The account ID of the expected bucket owner. If the account ID that you provide does not match the actual owner of the bucket, the request fails with the HTTP status code <code>403 Forbidden</code> (access denied).</p><br>
    /// - On success, responds with [`DeleteBucketWebsiteOutput`](crate::operation::delete_bucket_website::DeleteBucketWebsiteOutput)
    /// - On failure, responds with [`SdkError<DeleteBucketWebsiteError>`](crate::operation::delete_bucket_website::DeleteBucketWebsiteError)
    pub fn delete_bucket_website(&self) -> crate::operation::delete_bucket_website::builders::DeleteBucketWebsiteFluentBuilder {
        crate::operation::delete_bucket_website::builders::DeleteBucketWebsiteFluentBuilder::new(self.handle.clone())
    }
}
