use super::{label_match_set::LabelMatchSet, parsed_label::LabelId, token_rule::TokenRule};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use Iterator;

// fn default_priority() -> u8 {
// 	100_u8
// }

fn default_none() -> Option<String> {
	None
}

fn default_disabled() -> bool {
	false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
	pub name: String,

	#[serde(default = "default_none")]
	pub description: Option<String>,

	#[serde(default = "default_none")]
	pub id: Option<String>,

	#[serde(default = "default_disabled")]
	pub disabled: bool,

	// #[serde(default = "default_priority")]
	// pub priority: u8,
	pub spec: RuleSpec,
}

impl Display for Rule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.name))?;
		f.write_fmt(format_args!(
			"{}",
			if let Some(id) = &self.id { format!(" ({})", id) } else { "".to_string() }
		))?;
		f.write_fmt(format_args!("{}", if self.disabled { " DISABLED" } else { "" }))
	}
}

impl Rule {
	/// `label` cannot be contained in `label_set`.
	/// This is done by calling `self.exclude_all`
	pub fn require_none(&self, labels: &[LabelId], label_set: &LabelMatchSet) -> bool {
		// println!("require_none");
		self.exclude_all(labels, label_set)
	}

	// deprecated
	// fn concat_labels(label: &LabelId, labels: &[LabelId]) -> Vec<LabelId> {
	// 	let mut res = Vec::from(labels);
	// 	res.push(*label);
	// 	res
	// }

	/// Only one of the `label_set` should be part of the set `label` + `labels`
	pub fn require_one(&self, labels: &[LabelId], label_set: LabelMatchSet) -> bool {
		// println!("require_one");
		// let ids = Rule::concat_labels(labels);
		// println!("ids = {:?}", ids);
		// println!("labelset = {:?}", label_set);
		label_set.matches_one(labels)
	}

	pub fn require_some(&self, labels: &[LabelId], label_set: &LabelMatchSet) -> bool {
		// println!("require_some");
		label_set.matches_some(labels)
	}

	pub fn require_all(&self, _labels: &[LabelId], _label_set: &LabelMatchSet) -> bool {
		println!("require_all");
		todo!();
	}

	pub fn exclude_none(&self, _labels: &[LabelId], _label_set: &LabelMatchSet) -> bool {
		// println!("exclude_none");
		true
	}

	pub fn exclude_one(&self, _labels: &[LabelId], _label_set: &LabelMatchSet) -> bool {
		// println!("exclude_one");
		todo!();
	}

	pub fn exclude_some(&self, _labels: &[LabelId], _label_set: &LabelMatchSet) -> bool {
		// println!("exclude_some");
		todo!();
	}

	/// The passed `LabelId` should be neither be `_label` nor part of the `_label_set`.
	pub fn exclude_all(&self, labels: &[LabelId], label_set: &LabelMatchSet) -> bool {
		let match_some = label_set.matches_some(labels);
		!match_some
	}

	pub fn check(&self, labels: &[LabelId]) -> Option<bool> {
		log::debug!("Checking rule: {}", self);
		log::trace!("Labels: {:?}", labels);

		// TODO: impl the when filter

		if self.disabled {
			return None
		}

		if let Some(tr) = &self.spec.exclude {
			log::trace!("  Processing exclude rules");
			return Some(match tr {
				TokenRule::None(ls) => self.exclude_none(labels, ls),
				TokenRule::One(ls) => self.exclude_one(labels, ls),
				TokenRule::Some(ls) => self.exclude_some(labels, ls),
				TokenRule::All(ls) => self.exclude_all(labels, ls),
			})
		} else {
			log::trace!("  NO exclude rules")
		}

		if let Some(tr) = &self.spec.require {
			log::trace!("  Processing require rules");
			return Some(match tr {
				TokenRule::None(ls) => self.require_none(labels, ls),
				TokenRule::One(ls) => self.require_one(labels, ls.clone()),
				TokenRule::Some(ls) => self.require_some(labels, ls),
				TokenRule::All(ls) => self.require_all(labels, ls),
			})
		} else {
			log::trace!("  NO require rules")
		}

		None
	}

	#[deprecated(note = "This is to trash")]
	pub fn check_old(&self, label: &LabelId, labels: &[LabelId]) -> Option<bool> {
		log::debug!("  OLD Check‰ng rule: {}", self);
		log::trace!(
			"  for: {} labels: {}",
			label,
			labels.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(", ")
		);

		// TODO: impl the when filter

		if self.disabled {
			return None
		};

		if let Some(tr) = &self.spec.exclude {
			log::trace!("  Processing exclude rules");
			return Some(match tr {
				TokenRule::None(ls) => self.exclude_none(labels, ls),
				TokenRule::One(ls) => self.exclude_one(labels, ls),
				TokenRule::Some(ls) => self.exclude_some(labels, ls),
				TokenRule::All(ls) => self.exclude_all(labels, ls),
			})
		} else {
			log::trace!("  NO exclude rules")
		}

		if let Some(tr) = &self.spec.require {
			log::trace!("  Processing require rules");
			return Some(match tr {
				TokenRule::None(ls) => self.require_none(labels, ls),
				TokenRule::One(ls) => self.require_one(labels, ls.clone()),
				TokenRule::Some(ls) => self.require_some(labels, ls),
				TokenRule::All(ls) => self.require_all(labels, ls),
			})
		} else {
			log::trace!("  NO require rules")
		}

		None
	}
}

// pub type LabelSet = String;

// impl From<LabelMatch> for LabelSet {
// 	fn from(lm: LabelMatch) -> Self {
// 		Self(vec![lm])
// 	}
// }

// {
// 	// Item(String),
// 	// Regexp(RegexPattern),
// }

// impl From<&str> for LabelSet {
// 	fn from(s: &str) -> Self {
// 		if s.contains('*') {
// 			LabelSet::Regexp(s.into())
// 		} else {
// 			LabelSet::Item(s.into())
// 		}
// 	}
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum RuleType {
// 	#[serde(rename = "require")]
// 	Require(TokenRule),
// 	#[serde(rename = "exclude")]
// 	Exclude(TokenRule),
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleSpec {
	// pub when: Option<LabelSet>,

	// following does not work
	// #[serde(flatten)]
	// pub rule_type: RuleType,
	pub require: Option<TokenRule>,
	pub exclude: Option<TokenRule>,
}

// #[derive(Debug, Serialize, Deserialize, Default)]
// pub struct Token {
// 	one_of: Option<TokenRule>,
// 	some_of: Option<TokenRule>,
// 	none_of: Option<TokenRule>,
// }

// pub type LabelSetNone =  LabelSet;
// pub type LabelSetOne =  LabelSet;
// pub type LabelSetSome =  LabelSet;
// pub type LabelSetAll =  LabelSet;

#[cfg(test)]
impl Default for RuleSpec {
	fn default() -> Self {
		Self { require: None, exclude: None }
	}
}

#[cfg(test)]
impl Default for Rule {
	fn default() -> Self {
		let spec = RuleSpec::default();
		Self {
			name: "Rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			// priority: 100,
			spec,
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
	// use super::*;

	// #[test]
	// fn test_from() {
	// 	assert_eq!(LabelSet::Regexp("B*".into()), LabelSet::from("B*"));
	// 	assert_eq!(LabelSet::Item("B1".into()), LabelSet::from("B1"));
	// }
}

#[cfg(test)]
mod test_label_set {
	use crate::lib::{label_match_set::LabelMatchSet, parsed_label::LabelId};

	#[test]
	fn test_from_single() {
		let set = LabelMatchSet::from("B1");
		assert_eq!(1, set.len());
	}

	#[test]
	fn test_from_multiple() {
		let set = LabelMatchSet::from("B1, C*");
		assert_eq!(2, set.len());
	}

	#[test]
	fn test_matches() {
		let set = LabelMatchSet::from("A1,A2,B*");
		assert_eq!(3, set.len());
		let res = set.matches(&LabelId::from("A1"));
		assert!(res.0);
		assert!(res.1.is_some());
		assert_eq!(1, res.1.unwrap().len());
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

#[cfg(test)]
mod test_rule {
	use crate::lib::{label_match::LabelMatch, label_match_set::LabelMatchSet};

	use super::*;

	#[test]
	fn test_token_rule_deserialize() {
		let yaml = r#"!one_of
- B1
- B2
"#;
		println!("== yaml:\n{}", yaml);
		let rs: TokenRule = serde_yaml::from_str(&yaml).unwrap();
		println!("rs = {:?}", rs);
	}

	#[test]
	fn test_token_rule_serialize() {
		let label_match = LabelMatch::from("B1");
		let label_set = LabelMatchSet::from(vec![label_match]);
		let rs: TokenRule = TokenRule::One(label_set);
		println!("{}", serde_yaml::to_string(&rs).unwrap());
	}

	// #[test]
	// fn test_serialize_rule_type() {
	// 	let rs: RuleType = RuleType::Require(Token { one_of: Some()});

	// 		println!("{}", serde_yaml::to_string(&rs).unwrap());
	// }

	#[test]
	fn test_rule_spec_serialize() {
		let label_match = LabelMatch::from("B1");
		let label_set = LabelMatchSet::from(vec![label_match]);

		let token_rule = TokenRule::One(label_set);
		let rs: RuleSpec = RuleSpec { require: Some(token_rule), exclude: None };

		println!("{}", serde_yaml::to_string(&rs).unwrap());
	}

	#[test]
	fn test_rule_serialize() {
		let label_match = LabelMatch::from("B1");
		let label_set = LabelMatchSet::from(vec![label_match]);
		let token_rule = TokenRule::One(label_set);
		let rs: RuleSpec = RuleSpec { require: Some(token_rule), exclude: None };
		let rule: Rule = Rule {
			name: "Foo".to_string(),
			description: None,
			spec: rs,
			id: None,
			disabled: false,
		};

		println!("{}", serde_yaml::to_string(&rule).unwrap());
	}

	// 	#[test]
	// 	fn test_deserialize_rule_spec() {
	// 		let yaml = r#"require: !one_of
	// - B*
	// "#;
	// 		println!("== yaml:\n{}", yaml);
	// 		let rs: RuleSpec = serde_yaml::from_str(&yaml).unwrap();
	// 		println!("rs = {:?}", rs);
	// 	}

	#[test]
	fn test_rule_deserialize() {
		let yaml = r#"name: Foo
id: foo
disabled: false
spec:
  require: !one_of
  - B1
"#;
		println!("== yaml:\n{}", yaml);
		let rule: Rule = serde_yaml::from_str(&yaml).unwrap();
		println!("rule = {:?}", rule);
	}

	#[test]
	fn magic_test() {
		let label_match = LabelMatch::from("foo");
		let label_set = LabelMatchSet::from(vec![label_match]);
		let token_rule = TokenRule::One(label_set);
		let rs = RuleSpec { require: Some(token_rule), exclude: None };

		let s = format!("{}", serde_yaml::to_string(&rs).unwrap());
		println!("{}", s);

		let new_rs: RuleSpec = serde_yaml::from_str(&s).unwrap();
		println!("{:#?}", new_rs);
	}

	#[test]
	fn test_rule_check_require_none_of_true() {
		let token_rule = TokenRule::None(LabelMatchSet::from(vec![
			LabelMatch::from("B0"),
			LabelMatch::from("B1"),
		]));
		let spec = RuleSpec { require: Some(token_rule), exclude: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&vec![LabelId::from("T0"), LabelId::from("T1"), LabelId::from("T2")]);

		assert_eq!(Some(true), res);
	}

	#[test]
	fn test_rule_check_require_none_of_false() {
		let token_rule = TokenRule::None(LabelMatchSet::from(vec![
			LabelMatch::from("B0"),
			LabelMatch::from("B1"),
		]));
		let spec = RuleSpec { require: Some(token_rule), exclude: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&vec![LabelId::from("B0"), LabelId::from("B1"), LabelId::from("B2")]);

		assert_eq!(Some(false), res);
	}

	#[test]
	fn test_rule_check_require_one_of_true() {
		let token_rule = TokenRule::One(LabelMatchSet::from(vec![
			LabelMatch::from("B0"),
			LabelMatch::from("B1"),
		]));
		let spec = RuleSpec { require: Some(token_rule), exclude: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&vec![LabelId::from("B0")]);

		assert_eq!(Some(true), res);
	}

	#[test]
	fn test_rule_check_require_one_of_false() {
		let token_rule = TokenRule::One(LabelMatchSet::from(vec![
			LabelMatch::from("B0"),
			LabelMatch::from("B1"),
		]));
		let spec = RuleSpec { require: Some(token_rule), exclude: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&vec![LabelId::from("B0"), LabelId::from("B1"), LabelId::from("B2")]);

		assert_eq!(Some(false), res);
	}

	#[test]
	fn test_rule_check_require_some_of() {
		let token_rule = TokenRule::Some(LabelMatchSet::from(vec![
			LabelMatch::from("B0"),
			LabelMatch::from("B1"),
		]));
		let spec = RuleSpec { require: Some(token_rule), exclude: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&vec![LabelId::from("B0"), LabelId::from("B1"), LabelId::from("B2")]);

		assert_eq!(Some(true), res);
	}

	#[test]
	fn test_rule_check_exclude_all_of_false() {
		let token_rule = TokenRule::All(LabelMatchSet::from(vec![
			LabelMatch::from("B0"),
			LabelMatch::from("B1"),
		]));
		let spec = RuleSpec { exclude: Some(token_rule), require: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&vec![LabelId::from("B0"), LabelId::from("B1"), LabelId::from("B2")]);

		assert_eq!(Some(false), res);
	}
}
