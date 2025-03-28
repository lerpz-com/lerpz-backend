//! All endpoints related to posts.

pub mod comments;
pub mod create;
pub mod delete;
pub mod edit;
pub mod list;

use aide::axum::{
	routing::{get_with, put_with},
	ApiRouter,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::AppState;

pub fn routes(state: AppState) -> ApiRouter {
	ApiRouter::new()
		.api_route(
    		"/",
            get_with(list::handler, list::docs).post_with(create::handler, create::docs)
		)
		.api_route(
			"/{post_id}",
			put_with(edit::handler, edit::docs).delete_with(delete::handler, delete::docs),
		)
		.nest_api_service("/{post_id}/comments", comments::routes(state.clone()))
		.with_state(state)
}

/// Request body for the create/edit posts endpoint.
#[derive(Debug, Serialize, Deserialize, JsonSchema, Validate)]
pub struct PostRequest {
	#[validate(length(min = 3, max = 64))]
	title: String,
	#[validate(length(min = 3, max = 1024))]
	body: String,
}

/// Parameters to identify a post.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct PostParams {
	post_id: Uuid,
}
