// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(std::fmt::Debug)]
pub enum Error {
    /// <p>An internal service error occurred.</p>
    InternalServiceError(crate::error::InternalServiceError),
    /// <p>The request was not valid. Verify that your request was properly formatted, that the signature was generated with the correct credentials, and that you haven't exceeded any of the service limits for your account.</p>
    InvalidRequestException(crate::error::InvalidRequestException),
    /// <p>The specified pipeline has been deleted.</p>
    PipelineDeletedException(crate::error::PipelineDeletedException),
    /// <p>The specified pipeline was not found. Verify that you used the correct user and account identifiers.</p>
    PipelineNotFoundException(crate::error::PipelineNotFoundException),
    /// <p>The specified task was not found. </p>
    TaskNotFoundException(crate::error::TaskNotFoundException),
    ///
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    ///
    /// When logging an error from the SDK, it is recommended that you either wrap the error in
    /// [`DisplayErrorContext`](crate::types::DisplayErrorContext), use another
    /// error reporter library that visits the error's cause/source chain, or call
    /// [`Error::source`](std::error::Error::source) for more details about the underlying cause.
    ///
    Unhandled(crate::error::Unhandled),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalServiceError(inner) => inner.fmt(f),
            Error::InvalidRequestException(inner) => inner.fmt(f),
            Error::PipelineDeletedException(inner) => inner.fmt(f),
            Error::PipelineNotFoundException(inner) => inner.fmt(f),
            Error::TaskNotFoundException(inner) => inner.fmt(f),
            Error::Unhandled(inner) => inner.fmt(f),
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ActivatePipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ActivatePipelineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ActivatePipelineError> for Error {
    fn from(err: crate::error::ActivatePipelineError) -> Self {
        match err.kind {
            crate::error::ActivatePipelineErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::ActivatePipelineErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ActivatePipelineErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::ActivatePipelineErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::ActivatePipelineErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::AddTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::AddTagsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::AddTagsError> for Error {
    fn from(err: crate::error::AddTagsError) -> Self {
        match err.kind {
            crate::error::AddTagsErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::AddTagsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::AddTagsErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::AddTagsErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::AddTagsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::CreatePipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::CreatePipelineError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::CreatePipelineError> for Error {
    fn from(err: crate::error::CreatePipelineError) -> Self {
        match err.kind {
            crate::error::CreatePipelineErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::CreatePipelineErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::CreatePipelineErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeactivatePipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DeactivatePipelineError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeactivatePipelineError> for Error {
    fn from(err: crate::error::DeactivatePipelineError) -> Self {
        match err.kind {
            crate::error::DeactivatePipelineErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::DeactivatePipelineErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DeactivatePipelineErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::DeactivatePipelineErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::DeactivatePipelineErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DeletePipelineError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DeletePipelineError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DeletePipelineError> for Error {
    fn from(err: crate::error::DeletePipelineError) -> Self {
        match err.kind {
            crate::error::DeletePipelineErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::DeletePipelineErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DeletePipelineErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::DeletePipelineErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribeObjectsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::DescribeObjectsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribeObjectsError> for Error {
    fn from(err: crate::error::DescribeObjectsError) -> Self {
        match err.kind {
            crate::error::DescribeObjectsErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::DescribeObjectsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DescribeObjectsErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::DescribeObjectsErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::DescribeObjectsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::DescribePipelinesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::DescribePipelinesError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::DescribePipelinesError> for Error {
    fn from(err: crate::error::DescribePipelinesError) -> Self {
        match err.kind {
            crate::error::DescribePipelinesErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::DescribePipelinesErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::DescribePipelinesErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::DescribePipelinesErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::DescribePipelinesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::EvaluateExpressionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::EvaluateExpressionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::EvaluateExpressionError> for Error {
    fn from(err: crate::error::EvaluateExpressionError) -> Self {
        match err.kind {
            crate::error::EvaluateExpressionErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::EvaluateExpressionErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::EvaluateExpressionErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::EvaluateExpressionErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::EvaluateExpressionErrorKind::TaskNotFoundException(inner) => {
                Error::TaskNotFoundException(inner)
            }
            crate::error::EvaluateExpressionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::GetPipelineDefinitionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::GetPipelineDefinitionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::GetPipelineDefinitionError> for Error {
    fn from(err: crate::error::GetPipelineDefinitionError) -> Self {
        match err.kind {
            crate::error::GetPipelineDefinitionErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::GetPipelineDefinitionErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::GetPipelineDefinitionErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::GetPipelineDefinitionErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::GetPipelineDefinitionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ListPipelinesError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::ListPipelinesError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ListPipelinesError> for Error {
    fn from(err: crate::error::ListPipelinesError) -> Self {
        match err.kind {
            crate::error::ListPipelinesErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::ListPipelinesErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ListPipelinesErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PollForTaskError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::PollForTaskError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PollForTaskError> for Error {
    fn from(err: crate::error::PollForTaskError) -> Self {
        match err.kind {
            crate::error::PollForTaskErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::PollForTaskErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::PollForTaskErrorKind::TaskNotFoundException(inner) => {
                Error::TaskNotFoundException(inner)
            }
            crate::error::PollForTaskErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::PutPipelineDefinitionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::PutPipelineDefinitionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::PutPipelineDefinitionError> for Error {
    fn from(err: crate::error::PutPipelineDefinitionError) -> Self {
        match err.kind {
            crate::error::PutPipelineDefinitionErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::PutPipelineDefinitionErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::PutPipelineDefinitionErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::PutPipelineDefinitionErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::PutPipelineDefinitionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::QueryObjectsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::QueryObjectsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::QueryObjectsError> for Error {
    fn from(err: crate::error::QueryObjectsError) -> Self {
        match err.kind {
            crate::error::QueryObjectsErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::QueryObjectsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::QueryObjectsErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::QueryObjectsErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::QueryObjectsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::RemoveTagsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::RemoveTagsError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::RemoveTagsError> for Error {
    fn from(err: crate::error::RemoveTagsError) -> Self {
        match err.kind {
            crate::error::RemoveTagsErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::RemoveTagsErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::RemoveTagsErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::RemoveTagsErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::RemoveTagsErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ReportTaskProgressError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ReportTaskProgressError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ReportTaskProgressError> for Error {
    fn from(err: crate::error::ReportTaskProgressError) -> Self {
        match err.kind {
            crate::error::ReportTaskProgressErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::ReportTaskProgressErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ReportTaskProgressErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::ReportTaskProgressErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::ReportTaskProgressErrorKind::TaskNotFoundException(inner) => {
                Error::TaskNotFoundException(inner)
            }
            crate::error::ReportTaskProgressErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ReportTaskRunnerHeartbeatError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ReportTaskRunnerHeartbeatError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ReportTaskRunnerHeartbeatError> for Error {
    fn from(err: crate::error::ReportTaskRunnerHeartbeatError) -> Self {
        match err.kind {
            crate::error::ReportTaskRunnerHeartbeatErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::ReportTaskRunnerHeartbeatErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ReportTaskRunnerHeartbeatErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SetStatusError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SetStatusError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::SetStatusError> for Error {
    fn from(err: crate::error::SetStatusError) -> Self {
        match err.kind {
            crate::error::SetStatusErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::SetStatusErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::SetStatusErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::SetStatusErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::SetStatusErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::SetTaskStatusError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: aws_smithy_http::result::SdkError<crate::error::SetTaskStatusError, R>) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::SetTaskStatusError> for Error {
    fn from(err: crate::error::SetTaskStatusError) -> Self {
        match err.kind {
            crate::error::SetTaskStatusErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::SetTaskStatusErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::SetTaskStatusErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::SetTaskStatusErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::SetTaskStatusErrorKind::TaskNotFoundException(inner) => {
                Error::TaskNotFoundException(inner)
            }
            crate::error::SetTaskStatusErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl<R> From<aws_smithy_http::result::SdkError<crate::error::ValidatePipelineDefinitionError, R>>
    for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: aws_smithy_http::result::SdkError<crate::error::ValidatePipelineDefinitionError, R>,
    ) -> Self {
        match err {
            aws_smithy_http::result::SdkError::ServiceError(context) => {
                Self::from(context.into_err())
            }
            _ => Error::Unhandled(crate::error::Unhandled::new(err.into())),
        }
    }
}
impl From<crate::error::ValidatePipelineDefinitionError> for Error {
    fn from(err: crate::error::ValidatePipelineDefinitionError) -> Self {
        match err.kind {
            crate::error::ValidatePipelineDefinitionErrorKind::InternalServiceError(inner) => {
                Error::InternalServiceError(inner)
            }
            crate::error::ValidatePipelineDefinitionErrorKind::InvalidRequestException(inner) => {
                Error::InvalidRequestException(inner)
            }
            crate::error::ValidatePipelineDefinitionErrorKind::PipelineDeletedException(inner) => {
                Error::PipelineDeletedException(inner)
            }
            crate::error::ValidatePipelineDefinitionErrorKind::PipelineNotFoundException(inner) => {
                Error::PipelineNotFoundException(inner)
            }
            crate::error::ValidatePipelineDefinitionErrorKind::Unhandled(inner) => {
                Error::Unhandled(crate::error::Unhandled::new(inner.into()))
            }
        }
    }
}
impl std::error::Error for Error {}
