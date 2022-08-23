use super::label_match_set::LabelMatchSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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

// impl RefSpecs for TokenRuleRequire {
// 	fn attach_ref(mut self) -> Self {
// 		match self {
// 			TokenRuleRequire::None(n) => n.attach_ref(),
// 			TokenRuleRequire::One(o) => todo!(),
// 			TokenRuleRequire::Some(s) => todo!(),
// 			TokenRuleRequire::All(a) => todo!(),
// 		}
// 		// .iter().for_each(|rule| rule.attach_ref(&self));
// 		self
// 	}
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum TokenRuleExclude {
	// /// None from the set
	// #[serde(rename = "none_of")]
	// None(LabelMatchSet),

	// /// A single one from the set
	// #[serde(rename = "one_of")]
	// One(LabelMatchSet),

	// /// Any number from the set, at least one and
	// /// up to the full set
	// #[serde(rename = "some_of")]
	// Some(LabelMatchSet),
	/// All of those in the set
	#[serde(rename = "all_of")]
	All(LabelMatchSet),
}

#[derive(Debug, Serialize, Deserialize)]
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
