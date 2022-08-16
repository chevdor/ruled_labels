use super::{parser::Parser, rules::Rules};
use semver::Version;
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct Specs {
	pub name: String,
	pub description: String,
	pub version: Option<Version>,
	pub labels: Vec<Label>,
	pub parser: Parser,

	#[serde(flatten)]
	pub rules: Rules,
}

#[derive(Debug, Deserialize)]
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

		self.rules.rules.iter().for_each(|rule| {
			let _ = f.write_fmt(format_args!(" - {:#?}\n", rule));
		});

		Ok(())
	}
}

impl Specs {
	pub fn check_label(&self, label: &str, amongst: Vec<String>) -> Result<(), String> {
		log::debug!("Checking label {} amongst {:?}", label, amongst);

		self.rules.rules.iter().for_each(|rule| {
			if rule.disabled {
				log::warn!("Rule DISABLED: {}", rule.name);
			} else if let Some(spec) = &rule.rule {
				log::info!("spec: {:?}", spec);
			} else {
				log::warn!("No spec, skipping {}", rule.name);
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
}

#[cfg(test)]
mod test_specs {
	use std::fs;

	use super::*;
	const SPEC_FILE: &str = "specs.yaml";

	#[test]
	fn test_deserialize_spec() {
		let s = fs::read_to_string(SPEC_FILE).unwrap();
		let specs: Specs = serde_yaml::from_str(&s).unwrap();

		let rule = &specs.rules.rules[0];
		println!("rule = {:?}", rule);
		assert!(rule.rule.is_some());
	}
}
