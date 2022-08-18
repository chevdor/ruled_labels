use super::{
	parsed_label::LabelId,
	parser::Parser,
	rule::{LabelSet, Rule},
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
	pub parser: Parser,

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
		f.write_fmt(format_args!("parser id: {}\n", self.parser.id))?;

		f.write_str("Rules:\n")?;

		self.rules.iter().for_each(|rule| {
			let _ = f.write_fmt(format_args!(" - {:#?}\n", rule));
		});

		Ok(())
	}
}

impl Specs {
	pub fn check_label(&self, label: &str, amongst: Vec<String>) -> Result<(), String> {
		log::debug!("Checking label {} amongst {:?}", label, amongst);

		self.rules.iter().for_each(|rule| {
			if rule.disabled {
				log::warn!("Rule DISABLED: {}", rule.name);
			} else {
				log::info!("spec: {:?}", rule.spec);
			}
		});

		Ok(())
	}

	pub fn check_labels(&self, labels: Vec<String>) -> Result<(), String> {
		labels.iter().for_each(|label| {
			let mut others = labels.clone();
			others.retain(|x| x != label);
			let _ = self.check_label(label, others);
		});
		Ok(())
	}

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
	use crate::lib::rule::*;
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
			parser: Parser::default(),
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
			parser: Parser::default(),
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
