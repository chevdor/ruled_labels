use crate::lib::test_result::{ResultPrinter, TestResult};
use anyhow::{Context, Result};

pub const DEFAULT_SPEC_FILE: &str = "specs.yaml";

// pub trait RefSpecs {
// 		/// After deserializing, we need to pass a ref to our specs down the stuct tree
// 	fn attach_ref(self) -> Self;
// }

use super::{
	label_match_set::LabelMatchSet,
	parsed_label::LabelId,
	// parser::Parser,
	rule::Rule,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Specs<'a> {
	pub name: String,
	pub description: String,
	pub version: Version,
	pub labels: Vec<Label>,

	// The following ends up not being used
	// pub parser: Parser,

	// #[serde(flatten)]
	pub rules: Vec<Rule<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
	pub name: String,
	pub description: String,
	pub color: String,
}

impl<'a> Display for Specs<'a> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_fmt(format_args!("name: {}\n", self.name))?;
		f.write_fmt(format_args!("desc: {}\n", self.description))?;
		f.write_fmt(format_args!("labels: {:?}\n", self.labels.len()))?;
		// f.write_fmt(format_args!("parser id: {}\n", self.parser.id))?;

		f.write_str("Rules:\n")?;

		self.rules.iter().for_each(|rule| {
			let _ = f.write_fmt(format_args!(" - {}\n", rule));
		});

		Ok(())
	}
}

impl<'a> Specs<'a> {
	pub fn load(file_path: &str) -> Result<Self> {
		let s = fs::read_to_string(PathBuf::from(file_path))?;
		let res = serde_yaml::from_str::<Self>(&s)
			.with_context(|| format!("Failed deserializing specs from {}", file_path))?;

		Ok(res.attach_ref())
	}

	pub fn load_default() -> Result<Self> {
		Self::load(DEFAULT_SPEC_FILE)
	}

	/// Our sub types need to access a ref of the specs
	fn attach_ref(self) -> Self {
		self.rules.iter().for_each(|rule| rule.attach_ref(&self));
		self
	}

	/// This functions loops thru all rules and check the rule outcome.
	pub fn run_checks(&self, labels: &[LabelId], run_skipped: bool) -> Vec<Option<bool>> {
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
				let check_result = rule.check(labels);
				ResultPrinter::new(&rule.name, TestResult::from(check_result))
					.with_indent(8)
					.print();
				// let status = if let Some(r) = check_result {
				// 	if r {
				// 		format!(
				// 			"{}{:>WIDTH$}{}",
				// 			color::Fg(color::Green),
				// 			"PASS",
				// 			color::Fg(color::Reset)
				// 		)
				// 	} else {
				// 		format!(
				// 			"{}{:>WIDTH$}{}",
				// 			color::Fg(color::Red),
				// 			"FAIL",
				// 			color::Fg(color::Reset)
				// 		)
				// 	}
				// } else {
				// 	format!(
				// 		"{}{:>WIDTH$}{}",
				// 		color::Fg(color::Cyan),
				// 		"SKIPPED",
				// 		color::Fg(color::Reset)
				// 	)
				// };

				// println!("{} {}", status, rule.name);
				check_result
			})
			.collect();
		// println!("{}", color::Fg(color::Reset));
		res
	}

	/// The passed argument contains a Vec of test status. This functions returns
	/// references to the faulty rules. This is used to show up more information to the user.
	pub fn find_faulty(&self, res: Vec<Option<bool>>) -> Vec<&Rule> {
		log::trace!("results: {:?}", res);

		let mut rule_iter = self.rules.iter();
		let result: Vec<&Rule> = res
			.iter()
			// .map(|r| {
			// 	let rule = rule_iter.next().expect("We expect to have as many rules as results");
			// 	if let Some(rr) = r {
			// 		if !(*rr) {
			// 			Some(rule)
			// 		} else {
			// 			None
			// 		}
			// 	} else {
			// 		None
			// 	}
			// })
			// .flatten()
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
			// .filter(|elem| elem.is_some())
			// .map(|elem| elem.unwrap())
			.collect();

		result
	}

	/// This functions loops thru all non disabled rules and check the rule outcome.
	/// DEPRECATED
	// pub fn check_label(&self, label: &LabelId, against: &Vec<LabelId>) -> Result<(), String> {
	// 	log::debug!("  Checking label {} against {:?}", label, against);

	// 	self.rules.iter().filter(|r| !r.disabled).for_each(|rule| {
	// 		// log::debug!("spec: {:?}", rule.spec);
	// 		let res = rule.check_old(label, against);
	// 		log::debug!("    rule check = {:?}", res);
	// 	});

	// 	Ok(())
	// }

	/// Loop thru all labels and check against others
	/// DEPRECATED
	// pub fn check_labels(&self, labels: &[LabelId]) -> Result<(), String> {
	// 	labels.iter().for_each(|label| {
	// 		let mut others: Vec<LabelId> = Vec::from(labels);
	// 		others.retain(|x| x != label);

	// 		let _ = self.check_label(label, &others);
	// 	});
	// 	Ok(())
	// }

	/// In the yaml spec file, the user either explicitely lists some `LabelId` or provide
	/// a list of patterns. The list of patterns needs to be applied against the actual list
	/// of labels. We also need to consider the case when a label is unknown to our specs.
	/// Fro instance, if our local set contains A1 and B1 and we query about A2, A2 needs to be
	/// added to the pre-filter set.
	pub fn generate_label_set(
		&self,
		set: LabelMatchSet,
		extra: Option<Vec<LabelId>>,
	) -> Vec<LabelId> {
		let mut list_from_spec: Vec<LabelId> = self
			.labels
			.iter()
			.map(|label| LabelId::try_from(label.name.as_str()).unwrap())
			.collect();
		if let Some(mut x) = extra {
			list_from_spec.append(&mut x);
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
	use crate::lib::{rule::*, token_rule::*};
	use std::fs;

	use super::*;
	const SPEC_FILE: &str = "specs.yaml";

	#[test]
	fn test_spec_serialize() {
		let label_set = LabelMatchSet::from_str("B1");
		let token_rule = TokenRuleRequire::One(label_set);
		let rs = RuleSpec { require: Some(token_rule), exclude: None };
		let rule = Rule {
			name: "Foo".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec: rs,
			specs_ref: None,
		};
		// let rules = Rules { rules: vec![rule] };
		let rules = vec![rule];

		let specs: Specs = Specs {
			name: "Foo".to_string(),
			description: "desc".to_string(),
			version: Version::new(0, 1, 0),
			labels: vec![],
			// parser: Parser::default(),
			rules,
		};

		println!("{}", serde_yaml::to_string(&specs).unwrap());
	}

	#[test]
	fn test_spec_deserialize() {
		let s = fs::read_to_string(SPEC_FILE).unwrap();
		let specs: Specs = serde_yaml::from_str(&s).unwrap();

		specs.rules.iter().for_each(|rule| {
			println!("rule = {:?}", rule);
		});
	}

	#[test]
	fn test_spec_ser_then_de() {
		let label_set = LabelMatchSet::from_str("B1");
		let token_rule = TokenRuleRequire::One(label_set);
		let rs = RuleSpec { require: Some(token_rule), exclude: None };
		let rule = Rule {
			name: "Foo".to_string(),
			description: None,
			id: None,
			disabled: false,
			spec: rs,
			specs_ref: None,
		};
		// let rules = Rules { rules: vec![rule] };
		let rules = vec![rule];

		let specs: Specs = Specs {
			name: "Foo".to_string(),
			description: "desc".to_string(),
			version: Version::new(0, 1, 0),
			labels: vec![],
			// parser: Parser::default(),
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
		let s = fs::read_to_string(SPEC_FILE).unwrap();
		let specs: Specs = serde_yaml::from_str(&s).unwrap();
		let label_set = LabelMatchSet::from_str("A1,A2,B*");
		let set = specs.generate_label_set(label_set, None);
		// let target =LabelSet::from("A1", "A2", "B0", "B1", "B2")

		assert_eq!(
			vec![
				LabelId::try_from("A1").unwrap(),
				LabelId::try_from("A2").unwrap(),
				LabelId::try_from("B0").unwrap(),
				LabelId::try_from("B1").unwrap(),
				LabelId::try_from("B2").unwrap(),
			],
			set
		);
	}

	#[test]
	fn test_generate_label_set_some() {
		let s = fs::read_to_string(SPEC_FILE).unwrap();
		let specs: Specs = serde_yaml::from_str(&s).unwrap();
		let label_set = LabelMatchSet::from_str("A1,A2,B*,T*");
		let extra = Some(vec![LabelId::try_from("T9").unwrap()]);
		let set = specs.generate_label_set(label_set, extra);

		assert_eq!(
			vec![
				LabelId::try_from("A1").unwrap(),
				LabelId::try_from("A2").unwrap(),
				LabelId::try_from("B0").unwrap(),
				LabelId::try_from("B1").unwrap(),
				LabelId::try_from("B2").unwrap(),
				LabelId::try_from("T9").unwrap(),
			],
			set
		);
	}
}
