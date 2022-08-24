mod lib;
mod opts;

use crate::lib::{
	parsed_label::LabelId,
	rule::Rule,
	spec::Specs,
	test_result::{ResultPrinter, TestResult},
	tests::Tests,
};
use clap::{crate_name, crate_version, StructOpt};
use env_logger::Env;
use opts::*;
use std::{collections::HashSet, env, error::Error};

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

			match specs {
				Ok(_) => {
					println!("✅ The file {} looks OK", cmd_opts.spec_file);
					std::process::exit(0)
				},
				Err(e) => {
					println!("❌ The file {} contains errors", cmd_opts.spec_file);
					eprintln!("{:?}", e);
					std::process::exit(1)
				},
			}
		},

		SubCommand::Check(cmd_opts) => {
			log::debug!("check: {:#?}", cmd_opts);
			let specs: Specs = Specs::load(&cmd_opts.spec_file)?;

			let label_ids: HashSet<LabelId> =
				cmd_opts.labels.iter().map(|s| LabelId::from(s.as_ref())).collect();
			let res = specs.run_checks(&label_ids, true);
			let aggregated_result = res.iter().fold(true, |acc, x| match x {
				Some(v) => acc && *v,
				None => acc,
			});

			let faulty_rules: Vec<&Rule> = specs.find_faulty(res);
			// println!("faulty_rules = {:?}", faulty_rules);
			if !faulty_rules.is_empty() {
				println!("faulty_rules:");
				faulty_rules.iter().for_each(|rule| println!("{:#?}", rule));
			}

			let title = format!(
				"{} v{} for labels {}",
				specs.name,
				specs.version,
				label_ids.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(", ")
			);
			ResultPrinter::new(&title, TestResult::from(aggregated_result)).print();

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

			// TODO: The following is erroneous as it does not consider `only` and `all`
			// println!(
			// 	"Running {:?} test cases on your {:?} rules",
			// 	tests.specs.specs.len(),
			// 	specs.rules.len()
			// );

			tests.run(specs, cmd_opts.only, cmd_opts.all);
			Ok(())
		},
	}
}
