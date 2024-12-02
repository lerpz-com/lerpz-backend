use axum::{
	extract::{Path, State},
	response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{error::HandlerResult, middleware::AuthUser, routes::v1::POSTS_TAG};

#[derive(Debug, Deserialize, Serialize)]
pub struct Params {
	post_id: Uuid,
}

#[utoipa::path(
	delete,
	path = "/api/v1/posts/{post_id}/delete",
	responses(
        (status = 200, description = "Successfully deleted post"),
    ),
    params(
        ("post_id" = Uuid, Path, description = "The UUID of the post")
    ),
    tag = POSTS_TAG
)]
pub async fn destroy(
	Path(params): Path<Params>,
	State(pool): State<PgPool>,
	AuthUser(user): AuthUser,
) -> HandlerResult<impl IntoResponse> {
	sqlx::query_as!(
		models::Post,
		"DELETE FROM posts WHERE user_id = $1 AND id = $2",
		&user.id,
		&params.post_id
	)
	.execute(&pool)
	.await?;

	Ok(())
}
