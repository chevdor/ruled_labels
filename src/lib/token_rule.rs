use super::label_set::LabelSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// #[serde(tag = "type")]
// #[serde(tag = "type", content = "list")]
pub enum TokenRule {
	/// None from the set
	#[serde(rename = "none_of")]
	None(LabelSet),

	/// A single one from the set
	#[serde(rename = "one_of")]
	One(LabelSet),

	/// Any number from the set, at least one and
	/// up to the full set
	#[serde(rename = "some_of")]
	Some(LabelSet),

	/// All of those in the set
	#[serde(rename = "all_of")]
	All(LabelSet),
}
