//! This module defines all the claps (cli) options and flags.

use clap::{crate_authors, crate_version, Parser, Subcommand};
use regex::Regex;
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

	/// Output without any coloring, this is useful
	/// for documentation and CI system where the color code
	/// pollute the output.
	#[clap(long, global = true)]
	pub no_color: bool,

	/// The output is more developer oriented
	#[clap(short, long, global = true)]
	pub dev: bool,
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

	/// Show details about the rules of the faulty tests
	#[clap(long)]
	pub faulty: bool,
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

	/// Only run the tests marked with only = true
	#[clap(long, conflicts_with = "all")]
	pub only: bool,

	/// Run ALL tests, even those marked as skip
	#[clap(long)]
	pub all: bool,

	/// By passing an optional filter, you can limit which tests will run
	#[clap(short, long)]
	pub filter: Option<Regex>,
}
