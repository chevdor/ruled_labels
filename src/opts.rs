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

	// #[clap(version = crate_version!(), author = crate_authors!())]
	// Lint(LintOpts),
	#[clap(version = crate_version!(), author = crate_authors!())]
	Check(CheckOpts),
	// #[clap(version = crate_version!(), author = crate_authors!())]
	// Test(TestOpts),
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
	/// The repo string for now in the form owner/repo such as chevdor/foobar
	#[clap(required = true, index = 1)]
	pub repository: String,

	/// The output filename
	#[clap(short, long, alias("out"), parse(from_os_str))]
	pub output: Option<PathBuf>,
}

/// Check label set against the rules
#[derive(Debug, Parser)]
pub struct CheckOpts {
	#[clap(index = 1, default_value = "specs.yaml")]
	pub spec_file: String,

	/// The repo string for now in the form owner/repo such as chevdor/foobar
	#[clap(long, short, required = true, multiple = true)]
	pub labels: Vec<String>,
}

/// Run tests using rules and a test set
#[derive(Debug, Parser)]
pub struct TestOpts {
	/// The repo string for now in the form owner/repo such as chevdor/foobar
	#[clap(required = true, index = 1)]
	pub repository: String,

	/// The output filename
	#[clap(short, long, alias("out"), parse(from_os_str))]
	pub output: Option<PathBuf>,
}
