//! Definitions of [Tests], [TestSpec] and [TestSpecs].

use super::{rule_filter::RuleFilter, specs::Specs};
use crate::rllib::{
	parsed_label::LabelId,
	test_result::{ResultPrinter, TestResult},
};
use anyhow::{Context, Result};
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashSet, fs, path::PathBuf};

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
	pub description: Option<String>,
	pub labels: Vec<String>,
	pub skip: Option<bool>,
	pub filter: Option<RuleFilter>,
	pub only: Option<bool>,
	pub expected: bool,
}

impl Tests {
	pub fn load(file_path: &PathBuf) -> Result<Self> {
		let s = fs::read_to_string(PathBuf::from(file_path))?;
		serde_yaml::from_str::<Self>(&s)
			.with_context(|| format!("Failed deserializing tests from {}", file_path.display()))
	}

	/// This is our test runner. It reads tests from a yaml file and apply the rules
	/// from another (overridable) yaml file. The tests specification contain the expectations
	/// for each test.
	// unnecessary_fold if required in our case
	#[allow(clippy::unnecessary_fold)]
	pub fn run(
		&self,
		specs: Specs,
		only: bool,
		all: bool,
		color: bool,
		dev: bool,
		filter: &Option<Regex>,
	) {
		let mut test_index = 0;

		// TODO: use `only` and `all` filters to count the effective number of tests
		let tests_count = self.specs.specs.len();

		log::info!("Running tests: {}", self.name);
		log::info!("Found {:?} tests", tests_count);
		log::info!("Using specs: {}", specs.name);
		log::info!("Using specs version: {}", specs.version.to_string());
		log::debug!("Only: {:?}", only);
		log::debug!("All : {:?}", all);
		log::debug!("Color : {:?}", color);

		// Iterate thru all the test specs
		let overall_result = self
			.specs
			.specs
			.iter()
			// if the --only flag was passeded, we consider only the `only` tests
			.filter(|spec| {
				if only {
					if let Some(o) = spec.only {
						o
					} else {
						false
					}
				} else {
					true
				}
			})
			// if --all was passed, we run all tests, including skipped ones
			.filter(|spec| {
				if all {
					true
				} else {
					match spec.skip {
						None => true,
						Some(skip) => !skip,
					}
				}
			})
			.filter(|spec| if let Some(f) = filter { f.is_match(&spec.name) } else { true })
			.map(|test_spec| {
				test_index += 1;
				// TODO: Bring back the `test_count` once fixed and considers `all` and
				// `only` println!(
				// 	"\n    ▶️ Running test {:>2?}/{:<2?}: {}",
				// 	test_index, tests_count, test_spec.name
				// );
				println!("\n    ▶️ Running test {:>2?}: {}", test_index, test_spec.name);
				if dev {
					println!(
						"      Expected to {}",
						match test_spec.expected {
							true => "PASS",
							false => "FAIL",
						}
					);
				}
				let labels: HashSet<LabelId> =
					test_spec.labels.clone().iter().map(|s| LabelId::from(s.as_ref())).collect();

				let results = specs.run_checks(&labels, true, color, dev, None, &test_spec.filter);

				let aggregated_result = results.iter().fold(true, |acc, x| match x {
					Some(v) => acc && *v,
					None => acc,
				});
				log::debug!("aggregated result for the test: {:?}", aggregated_result);
				log::debug!("expected   result for the test: {:?}", test_spec.expected);

				ResultPrinter::new(
					&test_spec.name,
					TestResult::from(test_spec.expected == aggregated_result),
				)
				.with_indent(4)
				.with_color(color)
				.print();
				test_spec.expected == aggregated_result
			})
			.fold(true, |acc, x| acc && x);

		let result = TestResult::from(overall_result);
		ResultPrinter::new("OVERALL", result.clone())
			.with_message_passed("All expectations are OK")
			.with_message_failed("Some expectations were not OK")
			.with_color(color)
			.print();

		match result {
			TestResult::Passed => std::process::exit(0),
			_ => std::process::exit(1),
		}
	}
}
