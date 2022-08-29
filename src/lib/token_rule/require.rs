use std::fmt::Display;

use crate::lib::{common::set_to_string, label_match_set::LabelMatchSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TokenRuleRequire {
	/// None from the set
	#[serde(rename = "none_of")]
	None(LabelMatchSet),

	/// A single one from the set
	#[serde(rename = "one_of")]
	One(LabelMatchSet),

	/// Any number from the set, at least one and
	/// up to the full set
	#[serde(rename = "some_of")]
	Some(LabelMatchSet),

	/// All of those in the set
	#[serde(rename = "all_of")]
	All(LabelMatchSet),
}

impl Display for TokenRuleRequire {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TokenRuleRequire::All(set) => f.write_fmt(format_args!(
				"you need to include all of the {} label(s)",
				set_to_string(set.iter())
			)),
			TokenRuleRequire::None(set) => f.write_fmt(format_args!(
				"you need to include none of the {} label(s)",
				set_to_string(set.iter())
			)),
			TokenRuleRequire::One(set) => f.write_fmt(format_args!(
				"you need to include one of the {} label(s)",
				set_to_string(set.iter())
			)),
			TokenRuleRequire::Some(set) => f.write_fmt(format_args!(
				"you need to include some of the {} label(s)",
				set_to_string(set.iter())
			)),
		}
	}
}
