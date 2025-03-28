//! Functions related to password hashing and verification.

/// Errors that can occur when working with passwords.
mod error;
/// Parts needed for hashing and validating passwords.
mod parts;
/// Schemas for hashing and validating passwords.
mod scheme;

use std::{str::FromStr, sync::LazyLock};

pub use error::{Error, Result};
pub use parts::{HashParts, PwdParts};
pub use scheme::{get_scheme, Scheme};

use regex::Regex;
use validator::ValidationError;

/// The default scheme used for hashing passwords.
pub static DEFAULT_SCHEME: &str = "01";

/// The default validation regex for passwords.
static VALIDATE_PWD_REGEX: LazyLock<(Regex, Regex, Regex, Regex)> = LazyLock::new(|| {
	(
		Regex::new(r"[a-z]").unwrap(),
		Regex::new(r"[A-Z]").unwrap(),
		Regex::new(r"[0-9]").unwrap(),
		Regex::new(r"[!@#$%&*?]").unwrap(),
	)
});

/// Validates a password against the default requirements.
///
/// The default requirements are:
/// - At least one lowercase letter
/// - At least one uppercase letter
/// - At least one number
/// - At least one symbol
///
/// If the password does not meet these requirements, a validation error is
/// returned. The error will have the code `requirements` and a list of errors
/// with the key `errors`.
pub fn validate_pwd_requirements(pwd: &str) -> std::result::Result<(), validator::ValidationError> {
	let (lowercase, uppercase, number, symbol) = VALIDATE_PWD_REGEX.clone();

	let requirements = [
		(lowercase, "missing_lowercase"),
		(uppercase, "missing_uppercase"),
		(number, "missing_number"),
		(symbol, "missing_symbol"),
	];

	let errors: Vec<&'static str> = requirements
		.into_iter()
		.filter_map(|(regex, msg)| {
			if !regex.is_match(pwd) {
				Some(msg)
			} else {
				None
			}
		})
		.collect();

	if errors.is_empty() {
		Ok(())
	} else {
		let mut err = ValidationError::new("requirements");
		err.add_param("errors".into(), &errors);
		Err(err)
	}
}

/// Hashes a password using the latest scheme.
///
/// # Safety
///
/// This uses the latest scheme, because we use the [`PwdParts::new`] method.
pub async fn hash_pwd(pwd: impl Into<String>, salt: impl Into<String>) -> Result<String> {
	unsafe { hash_pwd_parts(PwdParts::new(pwd.into(), salt.into())).await }
}

/// Hashes a password using custom [`PwdParts`].
///
/// This function can create a password using an old scheme. This is why it is
/// unsafe to call. You can use this function together with the
/// [`PwdParts::new`] method to create a password using the latest scheme.
///
/// # Safety
///
/// Use the [`PwdParts::new`] method to create a password using the latest
pub async unsafe fn hash_pwd_parts(pwd_parts: PwdParts) -> Result<String> {
	let scheme = get_scheme(&pwd_parts.scheme)?;
	tokio::task::spawn_blocking(move || {
		scheme
			.hash(&pwd_parts.pwd, &pwd_parts.salt)
			.map(|hash| format!("#{}#{}", pwd_parts.scheme, hash))
			.map_err(Error::SchemeError)
	})
	.await
	.map_err(|_| Error::FailSpawnBlockForHash)
	.and_then(|res| res)
}

/// Validates a hash against a password and salt.
///
/// The hash needs to be parseable to [`HashParts`]. See [`HashParts::from_str`]
/// to see how the format works.
pub async fn validate_pwd(
	pwd_hash: &str,
	pwd_ref: impl Into<String>,
	pwd_salt: Option<impl Into<String>>,
) -> Result<bool> {
	let pwd_hash = HashParts::from_str(pwd_hash)?;
	let pwd_ref = pwd_ref.into();
	let pwd_salt = pwd_salt.map(|v| v.into());
	unsafe { validate_pwd_parts(pwd_hash, pwd_ref, pwd_salt).await }
}

/// Validate a password using [`HashParts`] and a password reference.
///
/// This function validates a password hash against a password and optional
/// salt. This uses the [`HashParts`] to decide wich scheme to use for
/// validating. You can use the [`HashParts::from_str`] to create the
/// [`HashParts`] needed for validating the password scheme. If you do not use
/// the correct scheme, this function will error.
///
/// # Safety
///
/// Make sure you use [`HashParts::from_str`] to get the scheme or be certain
/// that the scheme given is the same as what was used to create the password
/// hash.
pub async unsafe fn validate_pwd_parts(
	hash_parts: impl Into<HashParts>,
	pwd_ref: impl Into<String>,
	pwd_salt: Option<impl Into<String>>,
) -> Result<bool> {
	let hash_parts = hash_parts.into();
	let pwd_ref = pwd_ref.into();
	let pwd_salt = pwd_salt.map(|v| v.into());

	let scheme = get_scheme(&hash_parts.scheme)?;
	tokio::task::spawn_blocking(move || {
		scheme
			.validate(&hash_parts.hash, &pwd_ref, pwd_salt.as_deref())
			.map_err(Error::SchemeError)
	})
	.await
	.map_err(|_| Error::FailSpawnBlockForValidate)
	.and_then(|res| res)
}

#[cfg(test)]
mod tests {
	use super::{hash_pwd, validate_pwd};

	#[tokio::test]
	async fn test_password_hashing_and_validate() {
		dotenvy::from_filename(".env.test").unwrap();

		let salt = uuid::Uuid::new_v4().to_string();
		let hash = hash_pwd("password".to_string(), salt.clone())
			.await
			.unwrap();

		assert!(!validate_pwd(&hash, "drowssap", Some(&salt)).await.unwrap());
		assert!(validate_pwd(&hash, "password", Some(&salt)).await.unwrap());
	}
}
