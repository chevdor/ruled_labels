use clap::{crate_authors, crate_version, Parser, Subcommand};
use std::path::PathBuf;

/// This utility allows checking labels based on rules
#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	// /// Output as json
	// #[clap(short, long, global = true)]
	// pub json: bool,
	#[clap(subcommand)]
	pub subcmd: SubCommand,
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	List(ListOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Lint(LintOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Check(CheckOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Test(TestOpts),
}

/// List all the rules
#[derive(Debug, Parser)]
pub struct ListOpts {
	/// The yaml spec file to be used.
	#[clap(index = 1, default_value = "specs.yaml")]
	pub spec_file: String,
}

/// Lint the rules
#[derive(Debug, Parser)]
pub struct LintOpts {
	/// Spec file
	#[clap(index = 1, default_value = "specs.yaml")]
	pub spec_file: String,
}

/// Check label set against the rules
#[derive(Debug, Parser)]
pub struct CheckOpts {
	/// Spec file
	#[clap(index = 1, default_value = "specs.yaml")]
	pub spec_file: String,

	/// The list of labels
	#[clap(long, short, required = true, multiple = true)]
	pub labels: Vec<String>,
}

/// Run tests using rules and a test set
#[derive(Debug, Parser)]
pub struct TestOpts {
	/// The yaml test file
	#[clap(index = 1, default_value = "tests.yaml")]
	pub test_specs: String,

	/// The spec is usually defined in the test file but you may override it
	#[clap(long, short)]
	pub spec_file: Option<PathBuf>,
}
