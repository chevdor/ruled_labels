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
		log::info!("Running tests: {}", self.name);
		log::info!("Using specs: {}", specs.name);
		if let Some(version) = &specs.version {
			log::info!("Using specs version: {}", version.to_string());
		}

		// Iterate thru all the test specs
		let overall_result = self
			.specs
			.specs
			.iter()
			.map(|test_spec| {
				log::info!("Running test: {}", test_spec.name);
				println!("\n▶️ Running test: {}", test_spec.name);
				let labels: Vec<LabelId> =
					test_spec.labels.clone().iter().map(|s| LabelId::from(s.as_ref())).collect();
				println!("Checking following labels:");
				labels.iter().for_each(|label| {
					println!(" - {}", label);
				});

				let results = specs.run_checks(&labels);

				let mut results_iter = results.iter();
				let aggregated_result = specs
					.rules
					.iter()
					.map(|rule| {
						let res = *results_iter.next().expect("We have as many results as rules");
						println!(
							"{} {}",
							if let Some(r) = res {
								if r {
									"✅"
								} else {
									"❌"
								}
							} else {
								"❔"
							},
							rule
						);
						res
					})
					.fold(true, |acc, x| match x {
						Some(v) => acc && v,
						None => acc,
					});
				println!("aggregated_result = {:?}", aggregated_result);
				aggregated_result
			})
			.all(|x| x);
		println!("overall_result = {:?}", overall_result);
		todo!()
	}
}
