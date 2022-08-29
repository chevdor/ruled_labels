//! `ruled-labels` is a cli helping with Github labels verifications based on a simple rule engine.
//! The rules are defined using a yaml file. `ruled-labels` allows running a single check but also
//! running a set of test cases to validate label set against your rules and ensuring your rules
//! meet all your expectations.

mod lib;
mod opts;

use crate::lib::{
	parsed_label::LabelId,
	rule::Rule,
	specs::Specs,
	test_result::{ResultPrinter, TestResult},
	tests::Tests,
};
use clap::{crate_name, crate_version, StructOpt};
use env_logger::Env;
use opts::*;
use std::{collections::HashSet, env, error::Error};

/// This is the entry point of the `ruled-labels` cli.
fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::info!("Running {} v{}", crate_name!(), crate_version!());
	let opts: Opts = Opts::parse();

	match opts.subcmd {
		SubCommand::List(cmd_opts) => {
			log::debug!("list: {:#?}", cmd_opts);
			let specs = Specs::load(&cmd_opts.spec_file)?;
			println!("{}", specs);
			Ok(())
		},

		SubCommand::Lint(cmd_opts) => {
			log::debug!("lint: {:#?}", cmd_opts);
			let specs: Result<Specs, _> = Specs::load(&cmd_opts.spec_file);
			let result = specs.is_ok();
			ResultPrinter::new("Lint Result", TestResult::from(result))
				.with_message_passed(&format!("The file {} looks OK", cmd_opts.spec_file))
				.with_message_failed(&format!("The file {} contains errors", cmd_opts.spec_file))
				.with_color(!opts.no_color)
				.print();

			if result {
				std::process::exit(0)
			} else {
				std::process::exit(1)
			}
		},

		SubCommand::Check(cmd_opts) => {
			log::debug!("check: {:#?}", cmd_opts);
			let specs: Specs = Specs::load(&cmd_opts.spec_file)?;

			let label_ids: HashSet<LabelId> =
				cmd_opts.labels.iter().map(|s| LabelId::from(s.as_ref())).collect();
			let res = specs.run_checks(&label_ids, true, !opts.no_color, opts.verbose);
			let aggregated_result = res.iter().fold(true, |acc, x| match x {
				Some(v) => acc && *v,
				None => acc,
			});

			if cmd_opts.faulty {
				let faulty_rules: Vec<&Rule> = specs.find_faulty(res);
				// println!("faulty_rules = {:?}", faulty_rules);
				if !faulty_rules.is_empty() {
					println!("faulty_rules:");
					faulty_rules.iter().for_each(|rule| println!("{:#?}", rule));
				}
			}

			if opts.verbose {
				let title = format!(
					"{} v{} for labels {}",
					specs.name,
					specs.version,
					label_ids.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(", ")
				);
				ResultPrinter::new(&title, TestResult::from(aggregated_result))
					.with_color(!opts.no_color)
					.print();
			}

			if aggregated_result {
				std::process::exit(0)
			} else {
				std::process::exit(1)
			}
		},

		SubCommand::Test(cmd_opts) => {
			log::debug!("test: {:#?}", cmd_opts);
			let tests = Tests::load(&cmd_opts.test_specs)?;

			let spec_file = if let Some(spec_file) = cmd_opts.spec_file {
				spec_file
			} else {
				tests.spec_file.clone()
			};
			log::debug!("spec_file: {}", spec_file.display());
			let specs = Specs::load(&spec_file.display().to_string())?;

			println!("Tests specs: {}", &cmd_opts.test_specs);
			println!("Specs file : {}", &spec_file.display());

			// TODO: The following is unaccurate as it does not consider that some tests are
			// included/excluded by `only` and `all` println!(
			// 	"Running {:?} test cases on your {:?} rules",
			// 	tests.specs.specs.len(),
			// 	specs.rules.len()
			// );

			tests.run(specs, cmd_opts.only, cmd_opts.all, !opts.no_color, opts.verbose);
			Ok(())
		},
	}
}
