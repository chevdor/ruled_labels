use super::{label_match::LabelMatch, parsed_label::LabelId, spec::Specs};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// A Vec of `LabelMatch`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LabelMatchSet(HashSet<LabelMatch>);

impl LabelMatchSet {
	pub fn from_str(s: &str) -> Self {
		let res: HashSet<LabelMatch> = s
			.split(',')
			.map(|s| {
				let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
				LabelMatch::from(cleaned.as_str())
			})
			.collect();
		LabelMatchSet::from_vec(res)
	}

	fn from_vec(label_matches: HashSet<LabelMatch>) -> Self {
		Self(label_matches)
	}

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

	// /// Returns true if one of the passed `LabelId` matches items in the set.
	// pub fn matches_none(&self, ids: &HashSet<LabelId>, specs: &Specs) -> bool {
	// 	let augmented_label_set = specs.generate_label_set(self, None);
	// 	println!("augmented_label_set = {:?}", augmented_label_set);

	// 	todo!()
	// }

	/// Returns true if one of the passed `LabelId` matches items in the set.
	pub fn matches_one(&self, ids: &HashSet<LabelId>, specs: &Specs) -> bool {
		let hits = ids.iter().filter(|&id| self.matches(id).0);
		hits.count() == 1
	}

	/// Returns true if one of the passed `LabelId` matches items in the set.
	pub fn matches_some(&self, ids: &HashSet<LabelId>, specs: &Specs) -> bool {
		let hits = ids.iter().filter(|&id| self.matches(id).0);
		hits.count() >= 1
	}

	/// Returns true if ALL of the passed `LabelId` matches the items in the set.
	pub fn matches_all(&self, ids: &HashSet<LabelId>, specs: &Specs) -> bool {
		println!("matches_all");
		let hits = ids.iter().filter(|&&id| self.matches(&id).0);
		hits.count() == self.0.len()
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

// impl From<Vec<LabelMatch>> for LabelMatchSet {
// 	fn from(lm: Vec<LabelMatch>) -> Self {
// 		Self(lm)
// 	}
// }

// /// Conversion from a comma, separated list of `LabelMatch` such as
// /// "A1,B2,C*".
// impl From<&str> for LabelMatchSet {
// 	fn from(s: &str) -> Self {
// 		let res: Vec<LabelMatch> = s
// 			.split(',')
// 			.map(|s| {
// 				// println!("s = {:?}", s);
// 				let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
// 				LabelMatch::from(cleaned.as_str())
// 			})
// 			.collect();
// 		LabelMatchSet::from_str(res)
// 	}
// }

#[cfg(test)]
impl Default for LabelMatchSet {
	fn default() -> Self {
		Self(HashSet::from([LabelMatch::from("B1"), LabelMatch::from("B2")]))
	}
}

#[cfg(test)]
mod test_label_set {
	use crate::lib::label_id_set::LabelIdSet;

	use super::*;

	#[test]
	fn test_label_set_from_str_single() {
		let set = LabelMatchSet::from_str("B1");
		let first = set.0.iter().next().unwrap();
		assert_eq!(1, set.len());
		assert_eq!(&LabelMatch::from("B1"), first);
	}

	#[test]
	fn test_label_set_from_str_multiple() {
		let set = LabelMatchSet::from_str("B1,B2");
		let first = set.0.iter().next().unwrap();
		assert_eq!(2, set.len());
		assert_eq!(&LabelMatch::from("B1"), first);
	}

	#[test]
	fn test_label_set_from_str_multiple_spaces() {
		let set = LabelMatchSet::from_str(" B1,  B2");
		let first = set.0.iter().next().unwrap();
		let second = set.0.iter().nth(1).unwrap();
		assert_eq!(2, set.len());
		assert_eq!(&LabelMatch::from("B1"), first);
		assert_eq!(&LabelMatch::from("B2"), second);
	}

	#[test]
	fn test_matches() {
		assert!(LabelMatchSet::default().matches(&LabelId::from("B1")).0);
	}

	#[test]
	fn test_matches_one() {
		let specs_ref = &Specs::load_default().unwrap();
		assert!(
			LabelMatchSet::default().matches_one(&HashSet::from([LabelId::from("B1")]), specs_ref)
		);
	}

	#[test]
	fn test_matches_all() {
		let specs_ref = &Specs::load_default().unwrap();
		assert!(LabelMatchSet::default().matches_all(&LabelIdSet::from_str("B1,B2"), specs_ref));
		assert!(LabelMatchSet::default().matches_all(&LabelIdSet::from_str("B1,B2,B3"), specs_ref));
		assert!(LabelMatchSet::default().matches_all(&LabelIdSet::from_str("X0,B1,B2"), specs_ref));
		assert!(!LabelMatchSet::default().matches_all(&LabelIdSet::from_str("B1"), specs_ref));
		assert!(!LabelMatchSet::default().matches_all(&LabelIdSet::from_str("X0,B1"), specs_ref));
	}
}
