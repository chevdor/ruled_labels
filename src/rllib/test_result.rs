//! Definitions of [ResultPrinter] and [TestResult].

use std::fmt::Display;
use termion::color;

pub const WIDTH: usize = 7; // Length of SKIPPED
pub const DEFAULT_INDENT: usize = 0;

/// The [ResultPrinter] helps printing those `PASS foo` and `FAIL bar`
/// with or without color.
#[derive(Debug)]
pub struct ResultPrinter {
	pub title: String,
	pub result: TestResult,
	pub message_passed: Option<String>,
	pub message_failed: Option<String>,
	pub message_skipped: Option<String>,
	color: bool,
	indent: usize,
}

/// The variants for a [TestResult].
#[derive(Clone, Debug)]
pub enum TestResult {
	Skipped,
	Passed,
	Failed,
}

impl ResultPrinter {
	pub fn new(title: &str, result: TestResult) -> Self {
		Self {
			title: title.to_string(),
			result,
			message_passed: None,
			message_failed: None,
			message_skipped: None,
			indent: DEFAULT_INDENT,
			color: true,
		}
	}

	pub fn with_message_passed(mut self, msg: &str) -> Self {
		self.message_passed = Some(msg.to_string());
		self
	}

	pub fn with_message_failed(mut self, msg: &str) -> Self {
		self.message_failed = Some(msg.to_string());
		self
	}

	// pub fn with_message_skipped(mut self, msg: &str) -> Self {
	// 	self.message_skipped = Some(msg.to_string());
	// 	self
	// }

	pub fn with_indent(mut self, indent: usize) -> Self {
		self.indent = indent;
		self
	}

	pub fn with_color(mut self, color: bool) -> Self {
		self.color = color;
		self
	}

	pub fn print(&self) {
		log::debug!("Printing with color = {:?}", self.color);
		println!("{}", self);
	}
}

impl From<bool> for TestResult {
	fn from(b: bool) -> Self {
		if b {
			TestResult::Passed
		} else {
			TestResult::Failed
		}
	}
}

impl From<Option<bool>> for TestResult {
	fn from(bool_maybe: Option<bool>) -> Self {
		match bool_maybe {
			Some(b) => TestResult::from(b),
			None => TestResult::Skipped,
		}
	}
}

// impl Display for TestResult {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			TestResult::Skipped => f.write_fmt(format_args!(
// 				"{}{:<WIDTH$}{}",
// 				color::Fg(color::Cyan),
// 				"SKIPPED",
// 				color::Fg(color::Reset)
// 			)),
// 			TestResult::Passed => f.write_fmt(format_args!(
// 				"{}{:<WIDTH$}{}",
// 				color::Fg(color::Green),
// 				"PASSED",
// 				color::Fg(color::Reset)
// 			)),
// 			TestResult::Failed => f.write_fmt(format_args!(
// 				"{}{:<WIDTH$}{}",
// 				color::Fg(color::Red),
// 				"FAILED",
// 				color::Fg(color::Reset)
// 			)),
// 		}
// 	}
// }

impl Display for ResultPrinter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let message = match self.result {
			TestResult::Skipped =>
				if let Some(m) = &self.message_skipped {
					m
				} else {
					&self.title
				},
			TestResult::Passed =>
				if let Some(m) = &self.message_passed {
					m
				} else {
					&self.title
				},
			TestResult::Failed =>
				if let Some(m) = &self.message_failed {
					m
				} else {
					&self.title
				},
		};

		let indent = self.indent;

		let output = match self.result {
			TestResult::Skipped => format!(
				"{}{:<WIDTH$}{}",
				if self.color { color::Fg(color::Cyan).to_string() } else { "".into() },
				"SKIPPED",
				if self.color { color::Fg(color::Reset).to_string() } else { "".into() }
			),
			TestResult::Passed => format!(
				"{}{:<WIDTH$}{}",
				if self.color { color::Fg(color::Green).to_string() } else { "".into() },
				"PASSED",
				if self.color { color::Fg(color::Reset).to_string() } else { "".into() },
			),
			TestResult::Failed => format!(
				"{}{:<WIDTH$}{}",
				if self.color { color::Fg(color::Red).to_string() } else { "".into() },
				"FAILED",
				if self.color { color::Fg(color::Reset).to_string() } else { "".into() }
			),
		};

		match self.result {
			TestResult::Skipped => f.write_fmt(format_args!(
				"{:>indent$}{} {}{}{}",
				"",
				output,
				if self.color { color::Fg(color::LightBlack).to_string() } else { "".into() },
				message,
				if self.color { color::Fg(color::Reset).to_string() } else { "".into() }
			)),
			TestResult::Failed | TestResult::Passed =>
				f.write_fmt(format_args!("{:>indent$}{} {}", "", output, message)),
		}
	}
}
