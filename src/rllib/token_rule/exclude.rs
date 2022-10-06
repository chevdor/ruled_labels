use crate::rllib::{common::set_to_string, label_match_set::LabelMatchSet};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TokenRuleExclude {
	/// All of those in the set
	#[serde(rename = "all_of")]
	All(LabelMatchSet),
}

impl Display for TokenRuleExclude {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TokenRuleExclude::All(set) => f.write_fmt(format_args!(
				"you need to exclude all of the {} label(s)",
				set_to_string(set.iter())
			)),
		}
	}
}
