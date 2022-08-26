//! A `Vec<Rule>`. See [Rule].

use super::rule::Rule;
use serde::{Deserialize, Serialize};

/// Hold a a vector of [Rule]
#[derive(Debug, Serialize, Deserialize)]
pub struct Rules {
	pub rules: Vec<Rule>,
}
