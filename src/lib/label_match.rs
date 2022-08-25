use super::parsed_label::LabelId;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
}

impl Display for LabelMatch {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.0))
	}
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
