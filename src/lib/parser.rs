use super::common::RegexPattern;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Parser {
	pub id: RegexPattern,
	pub description: RegexPattern,
}

#[derive(Debug, Deserialize)]
pub struct Token {
	pub name: String,
	pub regexp: RegexPattern,
}
