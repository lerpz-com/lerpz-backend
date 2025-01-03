//! All endpoints related to posts.

pub mod comments;
pub mod create;
pub mod delete;
pub mod edit;
pub mod list;

use axum::{
	routing::{delete, get, patch, post},
	Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

pub use create::create;
pub use delete::destroy;
pub use edit::edit;
pub use list::list;

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/create", post(create))
		.route("/delete/:id", delete(destroy))
		.route("/edit/:id", patch(edit))
		.route("/posts", get(list))
        .nest("/comments", comments::routes())
}

/// Request body for the create/edit posts endpoint.
#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct PostRequest {
	title: String,
	body: String,
}

/// Parameters to identify a post.
#[derive(Debug, Deserialize, Serialize)]
pub struct PostParams {
	post_id: Uuid,
}
