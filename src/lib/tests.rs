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
	pub expected: bool,
}

impl Tests {
	/// This is our test runner. It reads tests from a yaml file and apply the rules
	/// from another (overridable) yaml file. The tests specification contain the expectations
	/// for each test.
	pub fn run(&self, specs: Specs) {
		let mut test_index = 0;
		let tests_count = self.specs.specs.len();

		log::info!("Running tests: {}", self.name);
		log::info!("Found {:?} tests", tests_count);
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
				test_index += 1;
				// log::info!("Running test: {}", test_spec.name);
				println!(
					"\n ▶️  Running test {:>2?}/{:<2?}: {}",
					test_index, tests_count, test_spec.name
				);
				let labels: Vec<LabelId> =
					test_spec.labels.clone().iter().map(|s| LabelId::from(s.as_ref())).collect();
				println!(
					"  ℹ️  Checking following labels: {}",
					labels.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(", ")
				);

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
									"   ✅"
								} else {
									"   ❌"
								}
							} else {
								"   ⏸️ "
							},
							rule
						);
						res
					})
					.fold(true, |acc, x| match x {
						Some(v) => acc && v,
						None => acc,
					});
				log::debug!("aggregated result for the test: {:?}", aggregated_result);
				log::debug!("expected   result for the test: {:?}", test_spec.expected);
				if test_spec.expected == aggregated_result {
					println!("  ✅ Test PASS");
				} else {
					println!("  ❌ Test FAIL");
				}

				test_spec.expected == aggregated_result
			})
			.all(|x| x);
		if overall_result {
			println!("✅ All tests PASS");
			std::process::exit(0)
		} else {
			println!("❌ Some tests FAIL");
			std::process::exit(1)
		}
	}
}
