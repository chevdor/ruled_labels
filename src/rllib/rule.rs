//! Definition of the [Rule] structure. A [Rule] defines the requirements for a set of [LabelId].

use super::{
	label_match_set::LabelMatchSet, parsed_label::LabelId, rule_spec::RuleSpec, specs::Specs,
};
use crate::rllib::{
	common::set_to_string, exclude::TokenRuleExclude, require::TokenRuleRequire,
	when::TokenRuleWhen,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Display};
use Iterator;

// fn default_priority() -> u8 {
// 	100_u8
// }

fn default_none<T>() -> Option<T> {
	None
}

fn default_false() -> bool {
	false
}

pub type Tag = String;
pub type RuleId = String;

/// This struct defines a [Rule]. It contains mainly some meta information
/// as well as the [RuleSpec] which describe the specs of the [Rule].
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Rule {
	pub name: String,

	#[serde(default = "default_none")]
	pub description: Option<String>,

	#[serde(default = "default_none")]
	pub id: Option<RuleId>,

	#[serde(default = "default_false")]
	pub disabled: bool,

	#[serde(default = "default_none")]
	pub tags: Option<Vec<Tag>>,

	// #[serde(default = "default_priority")]
	// pub priority: u8,
	pub spec: RuleSpec,
}

impl Display for Rule {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("{}", self.name))?;
		f.write_fmt(format_args!(
			"{}",
			if let Some(id) = &self.id { format!(" ({id})") } else { "".to_string() }
		))?;
		f.write_fmt(format_args!("{}", if self.disabled { " DISABLED" } else { "" }))
	}
}

impl Rule {
	/// Create a new named empty rule with no specs.
	#[cfg(test)]
	pub fn new(name: &str, spec: RuleSpec) -> Self {
		Self {
			name: name.to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		}
	}

	/// `label` cannot be contained in `label_set`.
	/// This is done by calling `self.exclude_all`
	pub fn require_none(
		&self,
		labels: &HashSet<LabelId>,
		label_set: &LabelMatchSet,
		specs: &Specs,
	) -> bool {
		// println!("require_none");
		self.exclude_all(labels, label_set, specs)
	}

	// deprecated
	// fn concat_labels(label: &LabelId, labels: &HashSet<LabelId>) -> Vec<LabelId> {
	// 	let mut res = Vec::from(labels);
	// 	res.push(*label);
	// 	res
	// }

	/// Only one of the `label_set` should be part of the set `label` + `labels`
	pub fn require_one(
		&self,
		labels: &HashSet<LabelId>,
		label_set: LabelMatchSet,
		specs: &Specs,
	) -> bool {
		// println!("require_one");
		// let ids = Rule::concat_labels(labels);
		// println!("ids = {:?}", ids);
		// println!("labelset = {:?}", label_set);
		label_set.matches_one(labels, specs)
	}

	pub fn require_some(
		&self,
		labels: &HashSet<LabelId>,
		label_match_set: &LabelMatchSet,
		specs: &Specs,
	) -> bool {
		// println!("require_some");
		label_match_set.matches_some(labels, specs)
	}

	/// All the labels from the `LabelMatchSet` must be present
	/// - labels: The set of LabelId to check
	/// - label_match_set: The specs of the labels that should match, this can contain wildcards and
	///   it is NOT expanded
	/// - specs: reference to the full specs
	pub fn require_all(
		&self,
		labels: &HashSet<LabelId>,
		label_match_set: &LabelMatchSet,
		specs: &Specs,
	) -> bool {
		// println!("require_all");
		label_match_set.matches_all(labels, specs)
	}

	/// The passed `LabelId` should be neither be `_label` nor part of the `_label_set`.
	pub fn exclude_all(
		&self,
		labels: &HashSet<LabelId>,
		label_match_set: &LabelMatchSet,
		specs: &Specs,
	) -> bool {
		let lset = specs.generate_reference_set(label_match_set, Some(labels));
		log::debug!("lset = {:?}", lset);
		let match_some = label_match_set.matches_some(labels, specs);
		!match_some
	}

	pub fn check(&self, labels: &HashSet<LabelId>, specs: &Specs) -> Option<bool> {
		log::debug!("⚙️ Checking rule: {}", self);
		log::trace!(
			"Labels: {}",
			labels.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
		);

		log::debug!("when = {:?}", self.spec.when);
		if let Some(when) = &self.spec.when {
			match when {
				TokenRuleWhen::None(when) =>
					if when.matches_none(labels, specs) {
						log::debug!(
							"when_none {} is satisfied with {}",
							when,
							set_to_string(labels)
						);
					} else {
						return None
					},
				TokenRuleWhen::One(when) =>
					if when.matches_one(labels, specs) {
						log::debug!(
							"when_one {} is satisfied with {}",
							when,
							set_to_string(labels)
						);
					} else {
						return None
					},
				TokenRuleWhen::Some(when) =>
					if when.matches_some(labels, specs) {
						log::debug!(
							"when_some {} is satisfied with {}",
							when,
							set_to_string(labels)
						);
					} else {
						return None
					},
				TokenRuleWhen::All(when) =>
					if when.matches_all(labels, specs) {
						log::debug!(
							"when_all {} is satisfied with {}",
							when,
							set_to_string(labels)
						);
					} else {
						return None
					},
			}
		}

		if self.disabled {
			return None
		}

		let exclude_result = if let Some(tr) = &self.spec.exclude {
			log::trace!("  Processing exclude rules");
			Some(match tr {
				TokenRuleExclude::All(ls) => self.exclude_all(labels, ls, specs),
			})
		} else {
			log::trace!("  NO exclude rules");
			None
		};

		let require_result = if let Some(tr) = &self.spec.require {
			log::trace!("  Processing require rules");
			Some(match tr {
				TokenRuleRequire::None(ls) => self.require_none(labels, ls, specs),
				TokenRuleRequire::One(ls) => self.require_one(labels, ls.clone(), specs),
				TokenRuleRequire::Some(ls) => self.require_some(labels, ls, specs),
				TokenRuleRequire::All(ls) => self.require_all(labels, ls, specs),
			})
		} else {
			log::trace!("  NO require rules");
			None
		};

		match (exclude_result, require_result) {
			(None, None) => None,
			(a @ Some(_), None) => a,
			(None, b @ Some(_)) => b,
			(Some(a), Some(b)) => Some(a && b),
		}
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
			tags: None,
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
mod test_label_set {
	use crate::rllib::{label_match_set::LabelMatchSet, parsed_label::LabelId};

	#[test]
	fn test_from_single() {
		let set = LabelMatchSet::from_str("B1");
		assert_eq!(1, set.len());
	}

	#[test]
	fn test_from_multiple() {
		let set = LabelMatchSet::from_str("B1, C*");
		assert_eq!(2, set.len());
	}

	#[test]
	fn test_matches() {
		let set = LabelMatchSet::from_str("A1,A2,B*");
		assert_eq!(3, set.len());
		let res = set.matches_label(&LabelId::from("A1"));
		assert!(res.0);
		assert!(res.1.is_some());
		assert_eq!(1, res.1.unwrap().len());
	}
}

#[cfg(test)]
mod test_rule {
	#![allow(non_snake_case)]
	use super::*;
	use crate::rllib::{
		exclude::TokenRuleExclude, label_id_set::LabelIdSet, label_match_set::LabelMatchSet,
		require::TokenRuleRequire, specs::*, when::TokenRuleWhen,
	};

	#[test]
	fn test_token_rule_deserialize() {
		let yaml = "!one_of\n- B1\n- B2\n";
		println!("== yaml:\n{}", yaml);
		let rs: TokenRuleRequire = serde_yaml::from_str(&yaml).unwrap();
		println!("rs = {:?}", rs);
	}

	#[test]
	fn test_token_rule_serialize() {
		let label_set = LabelMatchSet::from_str("B1");
		let rs: TokenRuleRequire = TokenRuleRequire::One(label_set);
		println!("{}", serde_yaml::to_string(&rs).unwrap());
	}

	#[test]
	fn test_rule_serialize() {
		let label_set = LabelMatchSet::from_str("B1");
		let token_rule = TokenRuleRequire::One(label_set);
		let rs: RuleSpec = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule: Rule = Rule {
			name: "Foo".to_string(),
			description: None,
			spec: rs,
			id: None,
			disabled: false,
			tags: None,
			//
		};

		println!("{}", serde_yaml::to_string(&rule).unwrap());
	}

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
		let label_set = LabelMatchSet::from_str("foo");
		let token_rule = TokenRuleRequire::One(label_set);
		let rs = RuleSpec { require: Some(token_rule), exclude: None, when: None };

		let s = format!("{}", serde_yaml::to_string(&rs).unwrap());
		println!("{}", s);

		let new_rs: RuleSpec = serde_yaml::from_str(&s).unwrap();
		println!("{:#?}", new_rs);
	}

	#[test]
	fn test_rule_check_require_none_of_true() {
		let specs = &Specs::load_test_default().unwrap();
		let token_rule =
			TokenRuleRequire::None(LabelMatchSet::from(LabelMatchSet::from_str("B0, B1")));
		let spec = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&LabelIdSet::from_str("T0,T1,T2"), specs);

		assert_eq!(Some(true), res);
	}

	#[test]
	fn test_rule_check_require_none_of_false() {
		let specs = &Specs::load_test_default().unwrap();
		let token_rule = TokenRuleRequire::None(LabelMatchSet::from_str("B0, B1"));
		let spec = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&LabelIdSet::from_str("B0, B1, B2"), specs);

		assert_eq!(Some(false), res);
	}

	#[test]
	fn test_rule_check_require_one_of_true() {
		let specs = &Specs::load_test_default().unwrap();
		let token_rule = TokenRuleRequire::One(LabelMatchSet::from_str("B0, B1"));
		let spec = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&LabelIdSet::from_str("B0"), specs);

		assert_eq!(Some(true), res);
	}

	#[test]
	fn test_rule_check_require_one_of_false() {
		let specs = &Specs::load_test_default().unwrap();
		let token_rule = TokenRuleRequire::One(LabelMatchSet::from_str("B0,B1"));
		let spec = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&LabelIdSet::from_str("B0,B1,B2"), specs);

		assert_eq!(Some(false), res);
	}

	#[test]
	fn test_rule_check_require_some_of() {
		let specs = &Specs::load_test_default().unwrap();
		let token_rule = TokenRuleRequire::Some(LabelMatchSet::from_str("B0,B1"));
		let spec = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
			//
		};

		// println!("rule = {:?}", rule);
		let res = rule.check(&LabelIdSet::from_str("B0,B1,B2"), specs);

		assert_eq!(Some(true), res);
	}

	#[test]
	fn test_rule_check_require_all_of() {
		let specs = &Specs::load_test_default().unwrap();
		let token_rule = TokenRuleRequire::All(LabelMatchSet::from_str("X1,X2,X3"));
		let spec = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule = Rule::new("test rule", spec);

		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("X1,X2,X3"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0,X1,X2,X3"), specs));
		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("X1,X2"), specs));
	}

	#[test]
	fn test_rule_check_require_all_of_with_star() {
		let specs = &Specs::load_test_default().unwrap();
		let token_rule = TokenRuleRequire::All(LabelMatchSet::from_str("X*"));
		let spec = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule = Rule::new("test rule", spec);
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("X1,X2,X3"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0,X1,X2,X3"), specs));
		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("X1,X1,X3"), specs));
		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("X1,X2"), specs));
	}

	#[test]
	fn test_rule_check_exclude_all_of_false() {
		let specs = &Specs::load_test_default().unwrap();
		let token_rule = TokenRuleExclude::All(LabelMatchSet::from_str("B0, B1"));
		let spec = RuleSpec { exclude: Some(token_rule), require: None, when: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		let res = rule.check(&LabelIdSet::from_str("B0, B1, B2"), specs);

		assert_eq!(Some(false), res);
	}

	#[test]
	fn test_rule_check_when_B1_require_A1() {
		let specs = &Specs::load_test_default().unwrap();
		let when_one_b1 = TokenRuleWhen::One(LabelMatchSet::from_str("B1"));
		let require_one_a1 = TokenRuleRequire::One(LabelMatchSet::from_str("A1"));
		let spec =
			RuleSpec { when: Some(when_one_b1), require: Some(require_one_a1), exclude: None };
		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("B0, B1, B2"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B1, A1"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B1, A1, A2, B2"), specs));
		assert_eq!(None, rule.check(&LabelIdSet::from_str("A1, A2, B2"), specs));
	}

	#[test]
	fn test_rule_check_when_all_of_B_require_one_A() {
		let specs = &Specs::load_test_default().unwrap();
		let when_all_b = TokenRuleWhen::All(LabelMatchSet::from_str("B*"));
		let require_one_a = TokenRuleRequire::One(LabelMatchSet::from_str("A*"));
		let spec = RuleSpec { when: Some(when_all_b), require: Some(require_one_a), exclude: None };

		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("B0, B1, B2"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, B1, B2, A1"), specs));
		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("B0, B1, B2, A1, A2"), specs));
		assert_eq!(None, rule.check(&LabelIdSet::from_str("B0, B1"), specs));
	}

	#[test]
	fn test_rule_check_when_all_of_B_require_some_As() {
		let specs = &Specs::load_test_default().unwrap();
		let when_all_b = TokenRuleWhen::All(LabelMatchSet::from_str("B*"));
		let require_some_a = TokenRuleRequire::Some(LabelMatchSet::from_str("A*"));
		let spec =
			RuleSpec { when: Some(when_all_b), require: Some(require_some_a), exclude: None };

		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("B0, B1, B2"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, B1, B2, A1"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, B1, B2, A1, A2"), specs));
		assert_eq!(None, rule.check(&LabelIdSet::from_str("B0, B1"), specs));
	}

	#[test]
	fn test_rule_check_when_one_of_B_require_A1() {
		let specs = &Specs::load_test_default().unwrap();
		let when_one_b = TokenRuleWhen::One(LabelMatchSet::from_str("B*"));
		let require_one_a1 = TokenRuleRequire::One(LabelMatchSet::from_str("A1"));
		let spec =
			RuleSpec { when: Some(when_one_b), require: Some(require_one_a1), exclude: None };

		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, A1"), specs));
		assert_eq!(None, rule.check(&LabelIdSet::from_str("B0, B1, B2, A1"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B1, A1, A2"), specs));
		assert_eq!(None, rule.check(&LabelIdSet::from_str("X1"), specs));
	}

	#[test]
	fn test_rule_check_when_one_of_B_require_some_As() {
		let specs = &Specs::load_test_default().unwrap();
		let when_one_b = TokenRuleWhen::One(LabelMatchSet::from_str("B*"));
		let require_some_a = TokenRuleRequire::Some(LabelMatchSet::from_str("A*"));
		let spec =
			RuleSpec { when: Some(when_one_b), require: Some(require_some_a), exclude: None };

		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, A1"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, A1, A2"), specs));
		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("B0, X1"), specs));
		assert_eq!(None, rule.check(&LabelIdSet::from_str("X1"), specs));
	}

	#[test]
	fn test_rule_check_when_some_of_B_require_some_A() {
		let specs = &Specs::load_test_default().unwrap();
		let when_some_b = TokenRuleWhen::Some(LabelMatchSet::from_str("B*"));
		let require_some_a = TokenRuleRequire::Some(LabelMatchSet::from_str("A*"));
		let spec =
			RuleSpec { when: Some(when_some_b), require: Some(require_some_a), exclude: None };

		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("B0, T8"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, A1"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B1, A2"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, B1, A2"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, B1, A1, A2"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, B1, A1, A2, T9"), specs));
		assert_eq!(None, rule.check(&LabelIdSet::from_str("X1"), specs));
	}

	#[test]
	fn test_rule_check_when_some_of_B_require_some_A_and_exclude_all_X() {
		let specs = &Specs::load_test_default().unwrap();
		let when_some_b = TokenRuleWhen::Some(LabelMatchSet::from_str("B*"));
		let require_some_a = TokenRuleRequire::Some(LabelMatchSet::from_str("A*"));
		let exclude_all_x = TokenRuleExclude::All(LabelMatchSet::from_str("X*"));

		let spec = RuleSpec {
			when: Some(when_some_b),
			require: Some(require_some_a),
			exclude: Some(exclude_all_x),
		};

		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, A1"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B1, A2"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, B1, A2"), specs));
		assert_eq!(Some(true), rule.check(&LabelIdSet::from_str("B0, B1, A1, A2"), specs));
		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("B0, B1, A1, A2, T9, X1"), specs));
		assert_eq!(None, rule.check(&LabelIdSet::from_str("X1"), specs));
	}

	#[test]
	fn test_rule_check_require_one_p_and_no_x() {
		let mut specs = Specs::load_test_default().unwrap();
		let require_one_p = TokenRuleRequire::One(LabelMatchSet::from_str("P*"));
		let exclude_all_x = TokenRuleExclude::All(LabelMatchSet::from_str("X*"));

		let spec =
			RuleSpec { when: None, require: Some(require_one_p), exclude: Some(exclude_all_x) };

		let rule = Rule {
			name: "test rule".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec,
			tags: None,
		};

		specs.rules = vec![rule.clone()];

		assert_eq!(Some(false), rule.check(&LabelIdSet::from_str("A1, B1"), &specs));
	}
}
