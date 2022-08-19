use super::{
	label_set::LabelSet,
	parsed_label::LabelId,
	// parser::Parser,
	rule::Rule,
};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Serialize, Deserialize)]
pub struct Specs {
	pub name: String,
	pub description: String,
	pub version: Option<Version>,
	pub labels: Vec<Label>,

	// The following ends up not being used
	// pub parser: Parser,

	// #[serde(flatten)]
	pub rules: Vec<Rule>,
}

#[derive(Debug, Serialize, Deserialize)]
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
		// f.write_fmt(format_args!("parser id: {}\n", self.parser.id))?;

		f.write_str("Rules:\n")?;

		self.rules.iter().for_each(|rule| {
			let _ = f.write_fmt(format_args!(" - {}\n", rule));
		});

		Ok(())
	}
}

impl Specs {
	/// This functions loops thru all rules and check the rule outcome.
	pub fn run_checks(&self, labels: &[LabelId]) -> Vec<Option<bool>> {
		log::debug!("Checking {:?} labels", labels.len());
		let res: Vec<Option<bool>> = self.rules.iter().map(|rule| rule.check(labels)).collect();
		res
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
	pub fn generate_label_set(&self, set: LabelSet, extra: Option<Vec<LabelId>>) -> Vec<LabelId> {
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
	use crate::lib::{label_match::LabelMatch, rule::*, token_rule::TokenRule};
	use std::fs;

	use super::*;
	const SPEC_FILE: &str = "specs.yaml";

	#[test]
	fn test_spec_serialize() {
		let label_match = LabelMatch::from("B1");
		let label_set = LabelSet::from(vec![label_match]);
		let token_rule = TokenRule::One(label_set);
		let rs = RuleSpec { require: Some(token_rule), exclude: None };
		let rule = Rule { name: "Foo".to_string(), id: None, disabled: false, spec: rs };
		// let rules = Rules { rules: vec![rule] };
		let rules = vec![rule];

		let specs: Specs = Specs {
			name: "Foo".to_string(),
			description: "desc".to_string(),
			version: None,
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
		let label_match = LabelMatch::from("B1");
		let label_set = LabelSet::from(vec![label_match]);
		let token_rule = TokenRule::One(label_set);
		let rs = RuleSpec { require: Some(token_rule), exclude: None };
		let rule = Rule { name: "Foo".to_string(), id: None, disabled: false, spec: rs };
		// let rules = Rules { rules: vec![rule] };
		let rules = vec![rule];

		let specs: Specs = Specs {
			name: "Foo".to_string(),
			description: "desc".to_string(),
			version: None,
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
	fn test_generate_labet_set_none() {
		let s = fs::read_to_string(SPEC_FILE).unwrap();
		let specs: Specs = serde_yaml::from_str(&s).unwrap();
		let label_set = LabelSet::from("A1,A2,B*");
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
	fn test_generate_labet_set_some() {
		let s = fs::read_to_string(SPEC_FILE).unwrap();
		let specs: Specs = serde_yaml::from_str(&s).unwrap();
		let label_set = LabelSet::from("A1,A2,B*,T*");
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
