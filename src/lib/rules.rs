use super::rule::Rule;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rules {
	pub rules: Vec<Rule>,
}
