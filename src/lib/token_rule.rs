use super::label_match_set::LabelMatchSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum TokenRuleExclude {
	/// All of those in the set
	#[serde(rename = "all_of")]
	All(LabelMatchSet),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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
