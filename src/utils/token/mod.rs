//! Functions related to token generation and verification.

mod claims;
mod error;
mod keys;

pub use error::{Error, Result};

pub use claims::{TokenClaims, TokenUser};
use jsonwebtoken::{decode, encode, TokenData};
use rand::{distr::Alphanumeric, Rng};

pub fn generate_access_token(user: impl Into<TokenUser>) -> Result<String> {
	encode(
		&jsonwebtoken::Header {
			alg: jsonwebtoken::Algorithm::EdDSA,
			..Default::default()
		},
		&TokenClaims::new(user),
		keys::jwt_encode_key(),
	)
	.map_err(Error::TokenError)
}

pub fn decode_access_token(token: &str) -> Result<TokenData<TokenClaims>> {
	decode(
		token,
		keys::jwt_decode_key(),
		&jsonwebtoken::Validation::default(),
	)
	.map_err(Error::TokenError)
}

pub fn generate_refresh_token() -> String {
	let rng = rand::rng();
	rng.sample_iter(&Alphanumeric)
		.take(32)
		.map(char::from)
		.collect()
}
