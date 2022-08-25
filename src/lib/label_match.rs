use super::parsed_label::LabelId;
use serde::{Deserialize, Serialize};

/// An type to describe one or a set of Labels
/// either specifying it or providing a regexp matching several
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct LabelMatch(String);

impl LabelMatch {
	/// Returns true if the passed `LabelId` matches our pattern
	pub fn matches(&self, id: &LabelId) -> bool {
		let pattern = &self.0;
		if pattern.contains('*') {
			match pattern.chars().next() {
				Some(pat) => pat == id.letter,
				_ => false,
			}
		} else {
			pattern == &id.to_string()
		}
	}

	/// Returns true if ONE of the passed `LabelId`s match our pattern
	pub fn matches_one(&self, _ids: &[LabelId]) -> bool {
		todo!()
	}

	/// Returns true if ALL of the passed `LabelId`s match our pattern
	pub fn matches_all(&self, _ids: &[LabelId]) -> bool {
		todo!()
	}

	// /// When we test labels such as X0, B1, B2, B3 against the LabelMatch B*
	// /// we only want to consider the labels matching the patter.
	// /// In other words, in our example, X0 is totally irrelevant to know
	// /// if X0, B1, B2, B3 matches the pattern B*. This function filters the non relevant labels.
	// pub fn filter(labels: &HashSet<LabelId>) -> &HashSet<LabelId> {

	// }
}

impl From<&str> for LabelMatch {
	fn from(s: &str) -> Self {
		Self(s.to_string())
	}
}

#[cfg(test)]
mod test_label_match {
	use super::*;

	#[test]
	fn test_1() {
		let m1 = LabelMatch::from("B1");
		m1.matches(&LabelId::from("B1"));
	}
}
