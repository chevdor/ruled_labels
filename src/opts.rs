//! This module defines all the claps (cli) options and flags.

use clap::{crate_authors, crate_version, Parser, Subcommand};
use regex::Regex;
use std::path::PathBuf;

use crate::rllib::{parsed_label::ParsedLabel, rule::Tag};

/// This utility allows checking labels based on rules
#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
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

// /// We need this helper to go from a list of labels in the form of
// /// `A1,B1` to a Vec of Labels. Clap can handle `A1, B1` but not `A1,B1` as it considers
// /// `A1,B1` as one label which is then parsed as `A1`, dropping `B1` which is unintended.
// fn parsed_labels_from_string(s: &str) -> Result<Vec<ParsedLabel>> {
// 	let re: Vec<ParsedLabel> = String::from(s)
// 		.split(",")
// 		.into_iter()
// 		.map(|s| ParsedLabel::try_from(s))
// 		.filter(|i| i.is_ok())
// 		.map(|i| i.unwrap())
// 		.collect();

// 	if re.len() > 0 {
// 		Ok(vec![])
// 	} else {
// 		bail!("Cound not find any label in {}", s)
// 	}
// }

/// Check label set against the rules
#[derive(Debug, Parser)]
pub struct CheckOpts {
	/// Spec file
	#[clap(index = 1, default_value = "specs.yaml")]
	pub spec_file: String,

	/// The list of labels. You may pass then as `-l A1,B1` or `-l A1 -l B1`.
	///
	/// NOTE: The following calls are NOT valid: `-l A1, B1` or `-l A1 B1`
	#[clap(long, short, required = true, num_args=1.., value_delimiter = ',')]
	pub labels: Vec<ParsedLabel>,

	/// Show details about the rules of the faulty tests
	#[clap(long)]
	pub faulty: bool,

	/// If you pass optional tags here, only the checks containing
	/// **all** those tags will run
	#[clap(short, long, num_args=0..)]
	pub tags: Option<Vec<Tag>>,
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

	/// By passing an optional filter, you can limit which tests will run.
	/// You can pass any valid regexp.
	#[clap(short, long)]
	pub filter: Option<Regex>,
}
