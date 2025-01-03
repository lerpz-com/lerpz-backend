mod auth;
mod posts;
mod health;

#[cfg(debug_assertions)]
mod failure;

use auth as Auth;
use axum::{routing::get, Router};

use crate::routes::v1::health as Health;
use crate::routes::v1::posts as Posts;

use sqlx::PgPool;
use utoipa::OpenApi;

const AUTH_TAG: &str = "Authentication API";
const POSTS_TAG: &str = "Post API";
const HEALTH_TAG: &str = "Health API";

pub fn routes() -> Router<PgPool> {
	let router = Router::new()
		.route("/health", get(health::health))
		.nest("/auth", Auth::routes())
		.nest("/post", Posts::routes());

	#[cfg(debug_assertions)]
	let router = router.route("/failure", get(failure::failure));

	router
}

#[derive(OpenApi)]
#[openapi(
    paths(
        Auth::login::login,
        Auth::register::register,
        Auth::refresh::refresh,
        Posts::create::create,
        Posts::delete::destroy,
        Posts::edit::edit,
        Posts::list::list,
        Posts::comments::create::create,
        Posts::comments::delete::destroy,
        Posts::comments::edit::edit,
        Posts::comments::list::list,
        Health::health,
    ),
    components(schemas(
        Auth::LoginRequest,
        Auth::RegisterRequest,
        Auth::TokenResponse,
    )),
    tags(
        (name = AUTH_TAG, description = "Endpoints for user authentication."),
        (name = POSTS_TAG, description = "Endpoints for handling posts."),
        (name = HEALTH_TAG, description = "Endpoints for checking application health."),
    )
)]
pub struct ApiDoc;
