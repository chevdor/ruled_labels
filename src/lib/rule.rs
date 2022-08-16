use super::common::RegexPattern;
use serde::Deserialize;

// fn default_priority() -> u8 {
// 	100_u8
// }

fn default_id() -> Option<String> {
	None
}

fn default_disabled() -> bool {
	false
}

#[derive(Debug, Deserialize)]
pub struct Rule {
	pub name: String,

	#[serde(default = "default_id")]
	pub id: Option<String>,

	#[serde(default = "default_disabled")]
	pub disabled: bool,

	// #[serde(default = "default_priority")]
	// pub priority: u8,
	pub rule: Option<RuleSpec>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum LabelSetSpec {
	Item(String),
	Regexp(RegexPattern),
}

impl From<&str> for LabelSetSpec {
	fn from(s: &str) -> Self {
		if s.contains('*') {
			LabelSetSpec::Regexp(s.into())
		} else {
			LabelSetSpec::Item(s.into())
		}
	}
}

#[derive(Debug, Deserialize)]
pub enum RuleType {
	Require(TokenRule),
	Exclude(TokenRule),
}

#[derive(Debug, Deserialize)]
pub struct RuleSpec {
	//TODO
	// pub when: Option<LabelSetSpec>,
	pub rule_type: Option<RuleType>,
}

#[derive(Debug, Deserialize)]
pub enum TokenRule {
	#[serde(alias = "none_of")]
	None(LabelSetSpec),

	#[serde(alias = "one_of")]
	One(LabelSetSpec),

	#[serde(alias = "some_of")]
	Some(LabelSetSpec),
}

impl Default for Rule {
	fn default() -> Self {
		Self {
			name: "Rule".to_string(),
			id: None,
			disabled: false,
			// priority: 100,
			rule: None,
		}
	}
}

#[cfg(test)]
mod test_rules {
	use super::*;

	#[test]
	fn test_rule_default() {
		let rule = Rule::default();
		assert_eq!(None, rule.id);
		assert_eq!(false, rule.disabled);
		// assert_eq!(100, rule.priority);
	}
}

#[cfg(test)]
mod test_label_set_spec {
	use super::*;

	#[test]
	fn test_from() {
		assert_eq!(LabelSetSpec::Regexp("B*".into()), LabelSetSpec::from("B*"));
		assert_eq!(LabelSetSpec::Item("B1".into()), LabelSetSpec::from("B1"));
	}
}

// #[cfg(test)]
// mod test_token_rule {
//     use super::*;

//     #[test]
//     fn test_from_str() {
//         TokenRule::from("_")
//     }
// }
