use super::common::RegexPattern;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Rules {
	pub rules: Vec<Rule>,
}

#[derive(Debug, Deserialize)]
pub enum LabelSetSpec {
	List(Vec<String>),
	Regexp(RegexPattern),
}

fn default_priority() -> u8 {
	100_u8
}
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

	#[serde(default = "default_priority")]
	pub priority: u8,

	pub specs: Option<RuleSpec>,
}

#[derive(Debug, Deserialize)]
pub struct RuleSpec {
	pub when: LabelSetSpec,
	pub rule_type: RuleType,
}

#[derive(Debug, Deserialize)]
pub enum RuleType {
	Require(LabelSetSpec),
	Exclude(LabelSetSpec),
}

#[derive(Debug, Deserialize)]
pub enum TokenRule {
	None(LabelSetSpec),
	One(LabelSetSpec),
	Some(LabelSetSpec),
}

impl Default for Rule {
	fn default() -> Self {
		Self { name: "Rule".to_string(), id: None, disabled: false, priority: 100, specs: None }
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
		assert_eq!(100, rule.priority);
	}
}
