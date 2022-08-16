use super::{parser::Parser, rules::Rules};
use serde::Deserialize;
use std::fmt::Display;

#[derive(Debug, Deserialize)]
pub struct Specs {
	pub name: String,
	pub description: String,
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
	pub fn check_label(&self, label: &str) -> Result<(), String> {
		log::debug!("Checking label: {}", label);
		Ok(())
	}

	pub fn check_labels(&self, labels: Vec<String>) -> Result<(), String> {
		labels.iter().for_each(|label| {
			let _ = self.check_label(label);
		});
		Ok(())
	}
}
