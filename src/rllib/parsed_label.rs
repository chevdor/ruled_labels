//! [ParsedLabel] and [LabelId]

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type CodeNumber = u8;

/// The [LabelId] is the initial letter + number from a Label.
/// For instance, the [LabelId] for `B0-silent` is `B0`.
///
/// WARNING: Do not confuse [LabelId] with [LabelMatch](super::label_match::LabelMatch).
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LabelId {
	pub letter: char,
	pub number: CodeNumber,
}

impl PartialEq<LabelId> for &str {
	fn eq(&self, l: &LabelId) -> bool {
		let letter = match self.chars().next() {
			Some(l) => l,
			_ => return false,
		};
		let number = match self.chars().next() {
			Some(l) => match l.to_string().parse::<CodeNumber>() {
				Ok(n) => n,
				_ => return false,
			},
			None => return false,
		};
		l.letter == letter && l.number == number
	}
}

impl PartialEq<str> for LabelId {
	fn eq(&self, s: &str) -> bool {
		let letter = match s.chars().next() {
			Some(l) => l,
			_ => return false,
		};
		let number = match s.chars().next() {
			Some(l) => match l.to_string().parse::<CodeNumber>() {
				Ok(n) => n,
				_ => return false,
			},
			None => return false,
		};
		self.letter == letter && self.number == number
	}
}

impl From<&str> for LabelId {
	fn from(s: &str) -> Self {
		// TODO: switch to removes_matches once https://github.com/rust-lang/rust/issues/72826 is resolved
		let sanitized_str = String::from(s).replace('\"', "");

		LabelId::from_str(&sanitized_str).expect("String should be a valid LabelId")
	}
}

// error[E0119]: conflicting implementations of trait `std::convert::TryFrom<&str>` for type
// `lib::parsed_label::LabelId` impl TryFrom<&str> for LabelId {
// 	type Error = String;
// 	fn try_from(s: &str) -> Result<Self, Self::Error> {
// 		LabelId::from_str(s)
// 	}
// }

impl LabelId {
	pub fn new(letter: char, number: CodeNumber) -> Self {
		Self { letter, number }
	}

	/// This version, using a 1 digit code, is faster as it does not use
	/// a regexp. See [Self::from_str] for the new version based on a regexp.
	pub fn from_str_fast(s: &str) -> Result<Self, String> {
		let sanitized_str = String::from(s).replace('\"', "");
		let mut chars = sanitized_str.chars();
		let first = chars.next();
		let second = chars.next();

		if first.is_none() || second.is_none() {
			return Err(format!("Err 001: Invalid label: {s} ({first:?}{second:?})"))
		}

		let first = first.expect("Cannot fail").to_ascii_uppercase();
		let second = second.expect("Cannot fail");

		if !(first.is_alphabetic() && second.is_numeric()) {
			return Err(format!("Err 002: Invalid label: {s} ({first}{second})"))
		}
		let second = second.to_string().parse::<CodeNumber>().expect("Cannot fail");
		Ok(LabelId::new(first, second))
	}

	/// Unlike [Self::from_str_fast], this function uses a regexp and allows supporting
	/// 2 digits codes.
	pub fn from_str(s: &str) -> Result<Self, String> {
		let sanitized_str = String::from(s).replace('\"', "").to_uppercase();

		let re = Regex::new(r"^([A-Z]{1})(\d+).*$").unwrap();
		let caps = re.captures(&sanitized_str);

		if caps.is_none() {
			return Err(format!("Err 002: Invalid label, no regexp match: {s}"))
		}

		let caps = caps.unwrap();

		let first = caps.get(1).map(|m| m.as_str().chars().next().unwrap());
		let second = caps.get(2).map(|m| m.as_str());

		if first.is_none() || second.is_none() {
			return Err(format!("Err 001: Invalid label: {s} ({first:?}{second:?})"))
		}

		let first = first.expect("Cannot fail").to_ascii_uppercase();
		let second = second.expect("Cannot fail");

		let second = second.to_string().parse::<CodeNumber>().expect("Cannot fail");
		Ok(LabelId::new(first, second))
	}
}

impl Display for LabelId {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}{}", self.letter, self.number))
	}
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParsedLabel {
	pub id: LabelId,
	pub description: Option<String>,
}

impl AsRef<ParsedLabel> for ParsedLabel {
	fn as_ref(&self) -> &ParsedLabel {
		self
	}
}

impl TryFrom<&str> for ParsedLabel {
	type Error = String;

	fn try_from(s: &str) -> Result<Self, Self::Error> {
		let id = LabelId::from_str(s)?;
		let description = s.to_string().drain(0..2).as_str().to_string();
		let description = if description.is_empty() { None } else { Some(description) };
		Ok(Self { id, description })
	}
}

impl From<String> for ParsedLabel {
	fn from(s: String) -> Self {
		println!("s = {s:?}");
		let id = LabelId::from_str(&s).unwrap();
		let mut s = s;
		let description = s.drain(0..2).as_str().to_string();
		let description = if description.is_empty() { None } else { Some(description) };
		Self { id, description }
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
	fn test_parsed_label_str_fancy_ok() {
		const INPUTS: &'static [&'static str] = &["B0-Foo 🧸", "\"b0-silent\""];

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

#[cfg(test)]
mod test_label_id {
	use super::*;

	#[test]
	fn test_label_id_ok() {
		const INPUTS: &'static [&'static str] =
			&["B0-Silent", "B1-silent", "X9-foobar", "X9 -foobar", "X9 - foobar", "B0"];

		INPUTS.iter().for_each(|&case| {
			let id = LabelId::from_str(case);
			println!("{:?}", id);
			assert!(id.is_ok());
		});
	}

	#[test]
	fn test_label_id_ok_2digits() {
		const INPUTS: &'static [&'static str] =
			&["B10-Silent", "B11-silent", "X09-foobar", "Z99 -foobar"];

		INPUTS.iter().for_each(|&case| {
			let id = LabelId::from_str(case);
			println!("{:?}", id);
			assert!(id.is_ok());
			assert!(id.unwrap().number >= 9);
		});
	}

	#[test]
	fn test_label_id_err() {
		const INPUTS: &'static [&'static str] = &["BB-Silent", "B-silent", "99-foobar"];

		INPUTS.iter().for_each(|&case| {
			let id = LabelId::from_str(case);
			println!("{:?}", id);
			assert!(id.is_err());
		});
	}

	#[test]
	fn test_label_id_cmp() {
		assert_eq!("B0", LabelId::from_str("B0").unwrap().to_string());
	}

	#[test]
	fn test_misc() {
		assert_eq!("B0", LabelId::from_str("B0 - foo").unwrap().to_string());
		assert_eq!("B0", LabelId::from_str("B0-foo").unwrap().to_string());
		assert_eq!("B0", LabelId::from_str("B0 -foo").unwrap().to_string());
		assert_eq!("B0", LabelId::from_str("B0- foo").unwrap().to_string());
		assert_eq!("B0", LabelId::from_str("B0   -    foo").unwrap().to_string());
		assert_eq!("B0", LabelId::from_str("B0 - foo bar").unwrap().to_string());
		assert_eq!("B0", LabelId::from_str("B0 - foo bar 😊😊😊").unwrap().to_string());
	}

	#[test]
	fn test_from_str() {
		let id = LabelId::from_str("B1").unwrap();
		assert_eq!('B', id.letter);
		assert_eq!(1, id.number);
	}

	#[test]
	fn test_b42() {
		assert_eq!("B42", LabelId::from_str("B42").unwrap().to_string());
		assert_eq!("B42", LabelId::from_str("B0042").unwrap().to_string());
		assert_eq!("B42", LabelId::from_str("b0042").unwrap().to_string());
	}

	#[test]
	fn test_b255() {
		assert_eq!("B255", LabelId::from_str("B255").unwrap().to_string());
	}

	#[test]
	#[should_panic]
	fn test_b256() {
		let _ = LabelId::from_str("B256");
	}
}
