use crate::lib::parsed_label::LabelId;

use super::spec::Specs;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Tests {
	pub name: String,
	pub spec_file: PathBuf,

	#[serde(flatten)]
	pub specs: TestSpecs,
}

#[derive(Debug, Deserialize)]
pub struct TestSpecs {
	pub specs: Vec<TestSpec>,
}

#[derive(Debug, Deserialize)]
pub struct TestSpec {
	pub name: String,
	pub labels: Vec<String>,
	pub expected: u8,
}

impl Tests {
	pub fn run(&self, specs: Specs) {
		log::debug!("Running tests: {}", self.name);
		log::debug!("Using specs: {}", specs.name);
		if let Some(version) = &specs.version {
			log::debug!("Using specs version: {}", version.to_string());
		}

		self.specs.specs.iter().for_each(|spec| {
			log::debug!("Running test {}", spec.name);
			let labels: Vec<LabelId> = spec
				.labels
				.clone()
				.iter()
				.map(|s| LabelId::try_from(s.as_ref()).expect("Can parse label"))
				.collect();
			let _res = specs.check_labels(&labels);
		});

		todo!()
	}
}
