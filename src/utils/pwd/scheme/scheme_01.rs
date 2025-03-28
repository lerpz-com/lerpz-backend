//! Scheme 0 implemented using Argon2.

use std::sync::LazyLock;

use argon2::{
	password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
	PasswordVerifier, Version,
};

use crate::config::CONFIG;

use super::{
	error::{Error, Result},
	Scheme,
};

static ARGON2: LazyLock<Argon2<'static>> = LazyLock::new(|| {
	Argon2::new_with_secret(
		CONFIG.PWD_SECRET.as_bytes(),
		Algorithm::Argon2id,
		Version::V0x13,
		Params::default(),
	)
	.unwrap()
});

pub struct Scheme01;

impl Scheme for Scheme01 {
	fn hash(&self, pwd: &str, salt: &str) -> Result<String> {
		let salt = SaltString::encode_b64(salt.as_bytes()).map_err(Error::PwdHash)?;

		let pwd = ARGON2
			.hash_password(pwd.as_bytes(), &salt)
			.map_err(Error::PwdHash)?
			.to_string();

		Ok(pwd)
	}

	fn validate(&self, pwd_hash: &str, pwd_ref: &str, _: Option<&str>) -> Result<bool> {
		let pwd_hash_parsed = PasswordHash::new(pwd_hash).map_err(Error::PwdHash)?;

		let pwd_ref_bytes = pwd_ref.as_bytes();

		Ok(ARGON2
			.verify_password(pwd_ref_bytes, &pwd_hash_parsed)
			.is_ok())
	}
}
