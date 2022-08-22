use super::parsed_label::LabelId;

/// A convenience struct to create a `Vec<LabelId>` from a comma
/// spearated string.
pub struct LabelIdSet;

impl LabelIdSet {
	pub fn from_str(s: &str) -> Vec<LabelId> {
		s.split(',')
			.map(|s| {
				let cleaned: String = s.chars().filter(|c| !c.is_whitespace()).collect();
				LabelId::from(cleaned.as_str())
			})
			.collect()
	}
}

#[cfg(test)]
mod test_labels_set {
	use super::*;

	#[test]
	fn test_from_str() {
		assert_eq!(1, LabelIdSet::from_str("B1").len());
		assert_eq!(2, LabelIdSet::from_str("B1,B2").len());
		assert_eq!(4, LabelIdSet::from_str("B1, B2 ,B3 ,B4").len());
	}
}
