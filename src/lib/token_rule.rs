use super::label_set::LabelMatchSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// #[serde(tag = "type")]
// #[serde(tag = "type", content = "list")]
pub enum TokenRule {
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
