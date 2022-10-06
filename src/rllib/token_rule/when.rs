use std::fmt::Display;

use crate::rllib::{common::set_to_string, label_match_set::LabelMatchSet};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum TokenRuleWhen {
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

impl Display for TokenRuleWhen {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TokenRuleWhen::All(set) => f.write_fmt(format_args!(
				"since you have all of the {} label(s)",
				set_to_string(set.iter())
			)),
			TokenRuleWhen::None(set) => f.write_fmt(format_args!(
				"since you none of {} label(s)",
				set_to_string(set.iter())
			)),
			TokenRuleWhen::One(set) => f.write_fmt(format_args!(
				"since you have one of the {} label(s)",
				set_to_string(set.iter())
			)),
			TokenRuleWhen::Some(set) => f.write_fmt(format_args!(
				"since you have some of the {} label(s)",
				set_to_string(set.iter())
			)),
		}
	}
}
