//! [LabelMatchSet] imoplementation.

use super::{label_match::LabelMatch, parsed_label::LabelId, specs::Specs};
use crate::lib::common::set_to_string;
use serde::{Deserialize, Serialize};
use std::{
	collections::{hash_set::Iter, HashSet},
	fmt::Display,
};

/// A [HashSet] of [LabelMatch]. It allows describing a list of
/// [LabelId] or patterns that will expand in such a list.
/// ## example:
/// ```
/// let lms = LabelMatchSet::from_str("B1, X*");
/// ```
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct LabelMatchSet(HashSet<LabelMatch>);

impl LabelMatchSet {
	#[cfg(test)]
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

	pub fn iter(&self) -> Iter<LabelMatch> {
		self.0.iter()
	}

	#[cfg(test)]
	fn from_vec(label_matches: HashSet<LabelMatch>) -> Self {
		Self(label_matches)
	}

	/// Check whether the passed `LabelId` matches at least one
	/// item in the `LabelSet`. If it matches it returns a tupple
	/// made of the matching status as boolean as well as the list of
	/// matching patterns.
	pub fn matches_label(&self, id: &LabelId) -> (bool, Option<Vec<&LabelMatch>>) {
		let matches: Vec<&LabelMatch> = self.0.iter().filter(|pat| pat.matches(id)).collect();
		let status = !matches.is_empty();
		let matches = if !matches.is_empty() { Some(matches) } else { None };
		(status, matches)
	}

	/// Returns true if one of the passed `LabelId` matches items in the set.
	pub fn matches_none(&self, _labels: &HashSet<LabelId>, _specs: &Specs) -> bool {
		// let augmented_label_set = specs.generate_label_set(self, None);
		// println!("augmented_label_set = {:?}", augmented_label_set);

		unimplemented!()
	}

	/// Returns true if one of the passed `LabelId` matches items in the set.
	pub fn matches_one(&self, labels: &HashSet<LabelId>, specs: &Specs) -> bool {
		let ref_set = specs.generate_reference_set(self, Some(labels));
		let hits = labels.iter().filter(|&label| ref_set.contains(label));
		hits.count() == 1
	}

	/// Returns true if one of the passed `LabelId` matches items in the set.
	pub fn matches_some(&self, labels: &HashSet<LabelId>, specs: &Specs) -> bool {
		let ref_set = specs.generate_reference_set(self, Some(labels));
		let hits = labels.iter().filter(|&label| ref_set.contains(label));
		hits.count() >= 1
	}

	/// Returns true if ALL of the passed `LabelId` matches the items in the set.
	/// This requires an intermediate step to expand the LabelMatchSet into an actual list
	/// according to both the input labels and the specs
	pub fn matches_all(&self, labels: &HashSet<LabelId>, specs: &Specs) -> bool {
		log::trace!("matches_all");

		let ref_set: HashSet<LabelId> =
			specs.generate_reference_set(self, Some(labels)).into_iter().collect();

		log::debug!("MatchSet     : {:?}", self);
		log::debug!("new ref_set  : {:>3?} => {}", ref_set.len(), set_to_string(&ref_set));
		log::debug!("labels       : {:>3?} => {}", labels.len(), set_to_string(labels));

		// We now iterate the ref_set to ensure that each of the items in the set
		// is indeed present in the `labels`.
		ref_set.iter().map(|l| labels.contains(l)).all(|r| r)

		// self.0.iter().map(|match_set| {
		// 	let labels_under_test = match_set.filter(labels);
		// 	println!("labels_under_test = {:?}", set_to_string(labels_under_test));
		// 	let res = match_set.matches_all(labels_under_test);
		// 	println!("res = {:?}", res);
		// 	res
		// }).all(|e| e)
	}

	#[cfg(test)]
	pub fn len(&self) -> usize {
		self.0.len()
	}
}

impl Display for LabelMatchSet {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", set_to_string(&self.0)))
	}
}

// impl IntoIterator for LabelMatchSet {
//     type Item = LabelMatch;
//     type IntoIter = LabelMatchIntoIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.iter()
//     }

// fn next(&mut self) -> Option<Self::Item> {
// 	let mut iter = self.0.iter();
// 	println!("next...");
// 	match iter.next() {
// 		None => None,
// 		Some(x) => Some(x.clone()),
// 	}
// }
// }

// pub struct LabelMatchIntoIterator {
//     label_match: LabelMatch,
//     index: usize,
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
		assert_eq!(2, set.len());
		assert!(set.0.contains(&LabelMatch::from("B1")));
	}

	#[test]
	fn test_label_set_from_str_multiple_spaces() {
		let set = LabelMatchSet::from_str(" B1,  B2");
		assert_eq!(2, set.len());
		assert!(set.0.contains(&LabelMatch::from("B1")));
		assert!(set.0.contains(&LabelMatch::from("B2")));
	}

	#[test]
	fn test_matches() {
		assert!(LabelMatchSet::default().matches_label(&LabelId::from("B1")).0);
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
