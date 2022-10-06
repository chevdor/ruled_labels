use super::rule::RuleId;
use serde::Deserialize;

/// A [RuleFilter] allows a test to specify the list of rules that should be ran
#[derive(Debug, Deserialize)]
pub struct RuleFilter {
	pub id: Vec<RuleId>,
}
