use super::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rules<'a> {
	pub rules: Vec<Rule<'a>>,
}
