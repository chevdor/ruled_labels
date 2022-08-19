use super::{label_match::LabelMatch, parsed_label::LabelId};
use serde::{Deserialize, Serialize};

/// A Vec of `LabelMatch`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LabelSet(Vec<LabelMatch>);

impl LabelSet {
	/// Check whether the passed `LabelId` matches at least one
	/// item in the `LabelSet`. If it matches it returns a tupple
	/// made of the matching status as boolean as well as the list of
	/// matching patterns.
	pub fn matches(&self, id: &LabelId) -> (bool, Option<Vec<&LabelMatch>>) {
		let matches: Vec<&LabelMatch> = self.0.iter().filter(|pat| pat.matches(id)).collect();
		let status = !matches.is_empty();
		let matches = if !matches.is_empty() { Some(matches) } else { None };
		(status, matches)
	}

	/// Returns true if one of the passed `LabelId` matches items in the set.
	pub fn matches_one(&self, ids: Vec<&LabelId>) -> bool {
		let hits = ids.iter().find(|&&id| self.matches(id).0);
		hits.is_some()
	}

	/// Returns true if ALL of the passed `LabelId` matches the items in the set.
	pub fn matches_all(&self, ids: Vec<&LabelId>) -> bool {
		let hits = ids.iter().filter(|&&id| self.matches(id).0);
		hits.count() == ids.len()
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}
}

// impl Iterator for LabelSet {
// 	type Item = LabelMatch;

// 	fn next(&mut self) -> Option<Self::Item> {
// 		let mut iter = self.0.iter();
// 		println!("next...");
// 		match iter.next() {
// 			None => None,
// 			Some(x) => Some(x.clone()),
// 		}
// 	}
// }

impl From<Vec<LabelMatch>> for LabelSet {
	fn from(lm: Vec<LabelMatch>) -> Self {
		Self(lm)
	}
}

/// Conversion from a comma, separated list of `LabelMatch` such as
/// "A1,B2,C*".
impl From<&str> for LabelSet {
	fn from(s: &str) -> Self {
		let res: Vec<LabelMatch> = s
			.split(',')
			.map(|s| {
				// println!("s = {:?}", s);
				let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
				LabelMatch::from(cleaned.as_str())
			})
			.collect();
		LabelSet::from(res)
	}
}

#[cfg(test)]
impl Default for LabelSet {
	fn default() -> Self {
		Self(vec![LabelMatch::from("B1"), LabelMatch::from("B2")])
	}
}

#[cfg(test)]
mod test_label_set {
	use super::*;

	#[test]
	fn test_label_set_from_str_single() {
		let set = LabelSet::from("B1");
		let first = set.0.first().unwrap();
		assert_eq!(1, set.len());
		assert_eq!(&LabelMatch::from("B1"), first);
	}

	#[test]
	fn test_label_set_from_str_multiple() {
		let set = LabelSet::from("B1,B2");
		let first = set.0.first().unwrap();
		assert_eq!(2, set.len());
		assert_eq!(&LabelMatch::from("B1"), first);
	}

	#[test]
	fn test_label_set_from_str_multiple_spaces() {
		let set = LabelSet::from(" B1,  B2");
		let first = set.0.first().unwrap();
		let second = set.0.iter().nth(1).unwrap();
		assert_eq!(2, set.len());
		assert_eq!(&LabelMatch::from("B1"), first);
		assert_eq!(&LabelMatch::from("B2"), second);
	}

	#[test]
	fn test_matches() {
		assert!(LabelSet::default().matches(&LabelId::from("B1")).0);
	}

	#[test]
	fn test_matches_one() {
		assert!(LabelSet::default().matches_one(vec![&LabelId::from("B1")]));
	}

	#[test]
	fn test_matches_all() {
		let b1 = LabelId::from("B1");
		let b2 = LabelId::from("B2");
		let ids = vec![&b1, &b2];
		assert!(LabelSet::default().matches_all(ids));
	}
}
