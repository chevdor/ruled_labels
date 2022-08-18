use serde::{Deserialize, Serialize};

pub type CodeNumber = u8;

/// The `LabelId` is the initial letter + number from a Label.
/// For instance, the `LabelId` for `B0-silent` is `B0`.
#[derive(Debug, Serialize, Deserialize)]
pub struct LabelId {
	pub letter: char,
	pub number: CodeNumber,
}

impl LabelId {
	pub fn new(letter: char, number: CodeNumber) -> Self {
		Self { letter, number }
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedLabel {
	pub id: LabelId,
	pub description: Option<String>,
}

impl TryFrom<&str> for ParsedLabel {
	type Error = String;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let first = s.chars().next();
		let second = s.chars().next();

		if first.is_none() || second.is_none() {
			return Err(format!("Invalid label: {}", s))
		}

		let first = first.expect("Cannot fail").to_ascii_uppercase();
		let second = second.expect("Cannot fail");

		if !(first.is_alphabetic() && second.is_numeric()) {
			return Err(format!("Invalid label: {}", s))
		}
		let second = second.to_string().parse::<CodeNumber>().expect("Cannot fail");

		let id = LabelId::new(first, second);
		let description = s.to_string().drain(0..2).as_str().to_string();
		let description = if description.is_empty() { None } else { Some(description) };
		Ok(Self { id, description })
	}
}

#[cfg(test)]
mod test_parsed_label {
	use super::*;

	#[test]
	fn test_parsed_label_from_str_ok() {
		const INPUTS: &'static [&'static str] =
			&["B0-Silent", "b0-silent", "Z9-foobar", "B0silent", "B00-Silent"];

		INPUTS.iter().for_each(|&case| {
			let label = ParsedLabel::try_from(case);
			println!("{:?}", label);
			assert!(label.is_ok());
			let label = label.unwrap();
			assert!(label.id.letter.is_uppercase());
			assert!((0..=9).contains(&label.id.number));
		});
	}

	#[test]
	fn test_parsed_label_from_str_error() {
		const INPUTS: &'static [&'static str] = &["BB-Silent", "B-silent", "99-foobar"];
		INPUTS.iter().for_each(|&case| {
			let label = ParsedLabel::try_from(case);
			println!("{:?}", label);
			assert!(label.is_err());
		});
	}
}
