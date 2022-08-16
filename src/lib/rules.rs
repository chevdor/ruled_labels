use super::rule::Rule;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rules {
	pub rules: Vec<Rule>,
}
