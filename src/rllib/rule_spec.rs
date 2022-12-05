use super::{
	common::capitalize, exclude::TokenRuleExclude, require::TokenRuleRequire, when::TokenRuleWhen,
};
use serde::{Deserialize, Serialize};

/// The [RuleSpec] describes:
/// - **when** the rule should be applied
/// - what [LabelMatch](super::label_match::LabelMatch) are **require**d
/// - what [LabelMatch](super::label_match::LabelMatch) are **exclude**d
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RuleSpec {
	pub when: Option<TokenRuleWhen>,
	pub require: Option<TokenRuleRequire>,
	pub exclude: Option<TokenRuleExclude>,
}

impl RuleSpec {
	#[cfg(test)]
	pub fn new(
		when: Option<TokenRuleWhen>,
		require: Option<TokenRuleRequire>,
		exclude: Option<TokenRuleExclude>,
	) -> Self {
		Self { when, require, exclude }
	}

	/// This function converts a [RuleSpec] into a user tip that
	/// can be shown if a rule check fails. The goal is **not** to
	/// tell the user what is wrong but tell the user how to fix it.
	pub fn to_user_tip(&self) -> String {
		let when = if let Some(w) = &self.when { format!("{w}, ") } else { String::new() };
		let require = if let Some(r) = &self.require { format!("{r}") } else { String::new() };

		let and = if self.require.is_some() {
			if self.exclude.is_some() {
				" and "
			} else {
				""
			}
		} else {
			""
		};

		let exclude = if let Some(e) = &self.exclude { format!("{e}") } else { String::new() };

		capitalize(&format!("{when}{require}{and}{exclude}"))
	}
}

#[cfg(test)]
impl Default for RuleSpec {
	fn default() -> Self {
		Self { require: None, exclude: None, when: None }
	}
}

#[cfg(test)]
mod test_rule_spec {
	use super::*;
	use crate::rllib::label_match_set::LabelMatchSet;

	#[test]
	fn test_rule_spec_serialize() {
		let label_set = LabelMatchSet::from_str("B1");

		let token_rule = TokenRuleRequire::One(label_set);
		let rs: RuleSpec = RuleSpec { require: Some(token_rule), exclude: None, when: None };

		println!("{}", serde_yaml::to_string(&rs).unwrap());
	}

	#[test]
	fn test_deserialize_rule_spec() {
		let yaml = "require: !one_of [ B* ]";
		println!("== yaml:\n{}", yaml);
		let rs: RuleSpec = serde_yaml::from_str(&yaml).unwrap();
		println!("rs = {:?}", rs);
	}

	#[test]
	fn test_rule_spec_to_tip() {
		// let specs = &Specs::load_test_default().unwrap();
		let when = TokenRuleWhen::All(LabelMatchSet::from_str("B*"));
		let require = TokenRuleRequire::Some(LabelMatchSet::from_str("A*"));
		let exlude = TokenRuleExclude::All(LabelMatchSet::from_str("B*"));

		let tip = RuleSpec::new(None, None, None).to_user_tip();
		println!("tip: {}", tip);

		let tip = RuleSpec::new(None, Some(require.clone()), None).to_user_tip();
		println!("tip: {}", tip);

		let tip = RuleSpec::new(None, None, Some(exlude.clone())).to_user_tip();
		println!("tip: {}", tip);

		let tip = RuleSpec::new(None, Some(require.clone()), Some(exlude.clone())).to_user_tip();
		println!("tip: {}", tip);

		let tip = RuleSpec::new(Some(when.clone()), Some(require.clone()), None).to_user_tip();
		println!("tip: {}", tip);

		let tip = RuleSpec::new(None, None, Some(exlude.clone())).to_user_tip();
		println!("tip: {}", tip);

		let tip = RuleSpec::new(Some(when.clone()), Some(require.clone()), Some(exlude.clone()))
			.to_user_tip();
		println!("tip: {}", tip);
	}
}
