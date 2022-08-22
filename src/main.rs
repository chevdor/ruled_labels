mod lib;
mod opts;

use clap::{crate_name, crate_version, StructOpt};
use env_logger::Env;
use opts::*;
use std::{env, error::Error, fs};

use crate::lib::{parsed_label::LabelId, spec::Specs, tests::Tests};
// use termion::{color, style};

fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::info!("Running {} v{}", crate_name!(), crate_version!());
	let opts: Opts = Opts::parse();

	match opts.subcmd {
		SubCommand::List(cmd_opts) => {
			log::debug!("list: {:#?}", cmd_opts);
			let spec_str = fs::read_to_string(cmd_opts.spec_file)?;
			let specs: Specs = serde_yaml::from_str(&spec_str)?;
			// println!("specs = {:#?}", specs);

			println!("{}", specs);
			Ok(())
		},
		SubCommand::Lint(cmd_opts) => {
			log::debug!("lint: {:#?}", cmd_opts);

			let spec_str = fs::read_to_string(&cmd_opts.spec_file)?;
			let specs: Result<Specs, _> = serde_yaml::from_str(&spec_str);

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
			let spec_str = fs::read_to_string(cmd_opts.spec_file)?;
			let specs: Specs = serde_yaml::from_str(&spec_str)?;

			let label_ids: Vec<LabelId> =
				cmd_opts.labels.iter().map(|s| LabelId::from(s.as_ref())).collect();
			let _ = specs.run_checks(&label_ids);
			// println!("{}", specs);
			Ok(())
		},
		SubCommand::Test(cmd_opts) => {
			log::debug!("test: {:#?}", cmd_opts);

			let spec_str = fs::read_to_string(cmd_opts.test_specs)?;
			let tests: Tests = serde_yaml::from_str(&spec_str)?;

			let spec_file = if let Some(spec_file) = cmd_opts.spec_file {
				spec_file
			} else {
				tests.spec_file.clone()
			};
			log::debug!("spec_file: {}", spec_file.display());

			let spec_str = fs::read_to_string(spec_file)?;
			let specs: Specs = serde_yaml::from_str(&spec_str)?;
			// println!("tests = {:#?}", &tests);
			// println!("specs = {:#?}", &specs);
			tests.run(specs);
			Ok(())
		},
		// SubCommand::Get(get_opts) => {
		// 	log::debug!("get: {:#?}", get_opts);
		// 	let input_repo = Repo::from_str(&get_opts.repository).unwrap();
		// 	let github = Github::new(String::from("glabel"), Credentials::Token(pat))?;

		// 	log::debug!("connected");

		// 	// TODO: there is the same block below, factorize that away !
		// 	let labels = github
		// 		.repo(input_repo.owner, input_repo.repository)
		// 		.labels()
		// 		.iter()
		// 		.map(|label| Label::from(label.unwrap()))
		// 		.collect::<Vec<_>>()
		// 		.await;

		// 	if let Some(file) = get_opts.output {
		// 		// let description = format!("Import from {}", get_opts.repository);
		// 		// let set = Set::new(&get_opts.repository, Some(description), labels);
		// 		let yml = serde_yaml::to_string(&set).unwrap();
		// 		// let mut file = File::create(file)?;
		// 		// file.write_all(yml.as_bytes())?;
		// 	} else {
		// 		labels.iter().for_each(|label| {
		// 			println!(
		// 				" - {: <24}\t[{}]: {}",
		// 				label.name,
		// 				label.color,
		// 				label.description.as_ref().unwrap_or(&String::from("n/a"))
		// 			);
		// 		})
		// 	}
		// }
	}
}
