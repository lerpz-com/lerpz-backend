use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::extract::{Path, State};

use crate::{error::HandlerResult, middleware::AuthUser, AppState};

use super::CommentParams;

#[axum::debug_handler]
pub async fn handler(
    Path(params): Path<CommentParams>,
    AuthUser(user): AuthUser,
	State(state): State<AppState>,
) -> HandlerResult<impl IntoApiResponse> {
    sqlx::query!(
		"DELETE FROM comments WHERE user_id = $1 AND post_id = $2 AND id = $3",
		&user.id,
		&params.post_id,
		&params.comment_id,
	)
	.execute(&state.pg)
	.await?;

	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Delete a comment.").tag("posts")
}
