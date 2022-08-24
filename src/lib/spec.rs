use crate::lib::test_result::{ResultPrinter, TestResult};
use anyhow::{Context, Result};

pub const DEFAULT_SPEC_FILE: &str = "specs.yaml";
pub const TEST_SPEC_FILE: &str = "./specs.yaml";

use super::{
	label_match_set::LabelMatchSet,
	parsed_label::LabelId,
	// parser::Parser,
	rule::Rule,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, fmt::Display, fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Specs {
	pub name: String,
	pub description: String,
	pub version: Version,
	pub labels: HashSet<Label>,

	// #[serde(flatten)]
	pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Label {
	pub name: String,
	pub description: String,
	pub color: String,
}

impl Display for Specs {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("name: {}\n", self.name))?;
		f.write_fmt(format_args!("desc: {}\n", self.description))?;
		f.write_fmt(format_args!("labels: {:?}\n", self.labels.len()))?;

		f.write_str("Rules:\n")?;

		self.rules.iter().for_each(|rule| {
			let _ = f.write_fmt(format_args!(" - {}\n", rule));
		});

		Ok(())
	}
}

impl Specs {
	pub fn load(file_path: &str) -> Result<Self> {
		let s = fs::read_to_string(PathBuf::from(file_path))?;
		let res = serde_yaml::from_str::<Self>(&s)
			.with_context(|| format!("Failed deserializing specs from {}", file_path))?;

		// Ok(res.attach_ref())
		Ok(res)
	}

	pub fn load_default() -> Result<Self> {
		Self::load(DEFAULT_SPEC_FILE)
	}

	#[cfg(test)]
	pub fn load_test_default() -> Result<Self> {
		Self::load(TEST_SPEC_FILE)
	}

	/// This functions loops thru all rules and check the rule outcome.
	pub fn run_checks(&self, labels: &HashSet<LabelId>, run_skipped: bool) -> Vec<Option<bool>> {
		println!(
			"      Running checks on {:?} labels: {}",
			labels.len(),
			labels.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
		);
		const WIDTH: usize = 8;

		let res: Vec<Option<bool>> = self
			.rules
			.iter()
			.filter(|rule| !rule.disabled || run_skipped)
			.map(|rule| {
				let check_result = rule.check(labels, self);
				ResultPrinter::new(&rule.name, TestResult::from(check_result))
					.with_indent(8)
					.print();
				check_result
			})
			.collect();
		res
	}

	/// The passed argument contains a Vec of test status. This functions returns
	/// references to the faulty rules. This is used to show up more information to the user.
	pub fn find_faulty(&self, res: Vec<Option<bool>>) -> Vec<&Rule> {
		log::trace!("results: {:?}", res);

		let mut rule_iter = self.rules.iter();
		let result: Vec<&Rule> = res
			.iter()
			.filter_map(|r| {
				let rule = rule_iter.next().expect("We expect to have as many rules as results");
				if let Some(rr) = r {
					if !(*rr) {
						Some(rule)
					} else {
						None
					}
				} else {
					None
				}
			})
			.collect();

		result
	}

	/// In the yaml spec file, the user either explicitely lists some `LabelId` or provide
	/// a list of patterns. The list of patterns needs to be applied against the actual list
	/// of labels. We also need to consider the case when a label is unknown to our specs.
	/// Foo instance, if our local set contains A1 and B1 and we query passing A2, A2 needs to be
	/// added to the pre-filter set.
	pub fn generate_label_set(
		&self,
		set: &LabelMatchSet,
		extra: Option<&HashSet<LabelId>>,
	) -> HashSet<LabelId> {
		let mut list_from_spec: HashSet<LabelId> = self
			.labels
			.iter()
			.map(|label| LabelId::try_from(label.name.as_str()).unwrap())
			.collect::<HashSet<_>>();
		if let Some(ids) = extra {
			let label_ids: HashSet<LabelId> = HashSet::from_iter(ids.clone());
			list_from_spec.extend(label_ids.iter());
		}
		println!("list_from_spec.len = {:?}", list_from_spec.len());
		println!("list_from_spec = {:?}", list_from_spec);

		// we now need to filter the full list according to the `set` and retain only the matches
		list_from_spec
			.iter()
			.filter(|&label_id| set.matches(label_id).0)
			.copied()
			.collect()
	}
}

#[cfg(test)]
mod test_specs {
	use super::*;
	use crate::lib::{label_id_set::LabelIdSet, rule::*, token_rule::*};

	#[test]
	fn test_spec_serialize() {
		let label_set = LabelMatchSet::from_str("B1");
		let token_rule = TokenRuleRequire::One(label_set);
		let rs = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule = Rule {
			name: "Foo".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec: rs,
		};
		// let rules = Rules { rules: vec![rule] };
		let rules = vec![rule];

		let specs: Specs = Specs {
			name: "Foo".to_string(),
			description: "desc".to_string(),
			version: Version::new(0, 1, 0),
			labels: HashSet::new(),
			rules,
		};

		println!("{}", serde_yaml::to_string(&specs).unwrap());
	}

	#[test]
	fn test_spec_deserialize() {
		let specs = Specs::load_default().unwrap();

		specs.rules.iter().for_each(|rule| {
			println!("rule = {:?}", rule);
		});
	}

	#[test]
	fn test_spec_ser_then_de() {
		let label_set = LabelMatchSet::from_str("B1");
		let token_rule = TokenRuleRequire::One(label_set);
		let rs = RuleSpec { require: Some(token_rule), exclude: None, when: None };
		let rule = Rule {
			name: "Foo".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec: rs,
		};
		// let rules = Rules { rules: vec![rule] };
		let rules = vec![rule];

		let specs: Specs = Specs {
			name: "Foo".to_string(),
			description: "desc".to_string(),
			version: Version::new(0, 1, 0),
			labels: HashSet::new(),
			rules,
		};

		let s = format!("{}", serde_yaml::to_string(&specs).unwrap());
		println!("{}", s);
		let new_specs: Specs = serde_yaml::from_str(&s).unwrap();

		let rule = &new_specs.rules[0];
		println!("rule = {:?}", rule);
	}

	#[test]
	fn test_generate_label_set_none() {
		let specs = Specs::load_test_default().unwrap();
		let label_set = LabelMatchSet::from_str("A1,A2,B*");
		let set = specs.generate_label_set(&label_set, None);

		assert_eq!(LabelIdSet::from_str("A1,A2,B0,B1,B2"), set);
	}

	#[test]
	fn test_generate_label_set_some() {
		let specs = Specs::load_test_default().unwrap();
		let label_set = LabelMatchSet::from_str("A1,A2,B*,T*");
		let extra = LabelIdSet::from_str("T9");
		let set = specs.generate_label_set(&label_set, Some(&extra));

		assert_eq!(LabelIdSet::from_str("A1,A2,B0,B1,B2, T9"), set);
	}
}
