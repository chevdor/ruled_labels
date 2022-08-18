// use super::common::RegexPattern;
use serde::{Deserialize, Serialize};

// fn default_priority() -> u8 {
// 	100_u8
// }

fn default_id() -> Option<String> {
	None
}

fn default_disabled() -> bool {
	false
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
	pub name: String,

	#[serde(default = "default_id")]
	pub id: Option<String>,

	#[serde(default = "default_disabled")]
	pub disabled: bool,

	// #[serde(default = "default_priority")]
	// pub priority: u8,
	pub spec: RuleSpec,
}

// pub type LabelSet = String;

/// An type to describe one or a set of Labels
/// either specifying it or providing a regexp matching several
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LabelMatch(String);

/// A Vec of `LabelMatch`
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LabelSet(Vec<LabelMatch>);

impl From<&str> for LabelMatch {
	fn from(s: &str) -> Self {
		Self(s.to_string())
	}
}

impl From<LabelMatch> for LabelSet {
	fn from(lm: LabelMatch) -> Self {
		Self(vec![lm])
	}
}

impl From<&str> for LabelSet {
	fn from(s: &str) -> Self {
		LabelSet::from(LabelMatch::from(s))
	}
}

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

#[derive(Debug, Serialize, Deserialize)]
// #[serde(untagged)]
// #[serde(tag = "type")]
// #[serde(tag = "type", content = "list")]
pub enum TokenRule {
	#[serde(rename = "none_of")]
	None(LabelSet),

	#[serde(rename = "one_of")]
	One(LabelSet),

	#[serde(rename = "some_of")]
	Some(LabelSet),

	#[serde(rename = "all_of")]
	All(LabelSet),
}

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

// #[cfg(test)]
// mod test_token_rule {
//     use super::*;

//     #[test]
//     fn test_from_str() {
//         TokenRule::from("_")
//     }
// }

#[cfg(test)]
mod test_rule_deserialize {
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
		let label_set = LabelSet { 0: vec![label_match] };
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
		let label_match = LabelMatch("B1".to_string());
		let label_set = LabelSet { 0: vec![label_match] };

		let token_rule = TokenRule::One(label_set);
		let rs: RuleSpec = RuleSpec { require: Some(token_rule), exclude: None };

		println!("{}", serde_yaml::to_string(&rs).unwrap());
	}

	#[test]
	fn test_rule_serialize() {
		let label_match = LabelMatch("B1".to_string());
		let label_set = LabelSet { 0: vec![label_match] };
		let token_rule = TokenRule::One(label_set);
		let rs: RuleSpec = RuleSpec { require: Some(token_rule), exclude: None };
		let rule: Rule = Rule { name: "Foo".to_string(), spec: rs, id: None, disabled: false };

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
		let label_set = LabelSet::from(label_match);
		let token_rule = TokenRule::One(label_set);
		let rs = RuleSpec { require: Some(token_rule), exclude: None };

		let s = format!("{}", serde_yaml::to_string(&rs).unwrap());
		println!("{}", s);

		let new_rs: RuleSpec = serde_yaml::from_str(&s).unwrap();
		println!("{:#?}", new_rs);
	}
}
