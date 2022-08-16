mod lib;
mod opts;

use clap::{crate_name, crate_version, StructOpt};
use env_logger::Env;
use lib::*;
use opts::*;
use std::{env, error::Error, fs};
// use termion::{color, style};

fn main() -> Result<(), Box<dyn Error>> {
	env_logger::Builder::from_env(Env::default().default_filter_or("none")).init();
	log::info!("Running {} v{}", crate_name!(), crate_version!());

	let opts: Opts = Opts::parse();
	let pat = env::var("GITHUB_TOKEN").unwrap_or_default();
	log::debug!("PAT: {}", if !pat.is_empty() { "SET" } else { "NOT SET " });

	match opts.subcmd {
		SubCommand::List(cmd_opts) => {
			log::debug!("list: {:#?}", cmd_opts);
			let s = fs::read_to_string(cmd_opts.spec_file)?;
			let specs: spec::Specs = serde_yaml::from_str(&s)?;
			// println!("specs = {:#?}", specs);

			println!("{}", specs);
			Ok(())
		},
		// SubCommand::Lint(cmd_opts) => {
		// 	log::debug!("lint: {:#?}", cmd_opts);

		// 	Ok(())
		// },
		SubCommand::Check(cmd_opts) => {
			log::debug!("check: {:#?}", cmd_opts);
			let s = fs::read_to_string(cmd_opts.spec_file)?;
			let specs: spec::Specs = serde_yaml::from_str(&s)?;

			let _ = specs.check_labels(cmd_opts.labels);
			// println!("{}", specs);
			Ok(())
		},
		// SubCommand::Test(cmd_opts) => {
		// 	log::debug!("test: {:#?}", cmd_opts);

		// 	Ok(())
		// },
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
