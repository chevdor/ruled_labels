use super::common::RegexPattern;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Parser {
	pub id: RegexPattern,
	pub description: RegexPattern,
}

#[derive(Debug, Deserialize)]
pub struct Token {
	pub name: String,
	pub regexp: RegexPattern,
}

impl Default for Parser {
	fn default() -> Self {
		Self { id: "^(\\w\\d).*$".to_string(), description: "^\\w\\d-(.*?)$".to_string() }
	}
}
