use std::fmt::Display;
use termion::color;

pub const WIDTH: usize = 7; // Length of SKIPPED
pub const DEFAULT_INDENT: usize = 0;

#[derive(Debug)]
pub struct ResultPrinter {
	pub title: String,
	pub result: TestResult,
	pub message_passed: Option<String>,
	pub message_failed: Option<String>,
	pub message_skipped: Option<String>,
	indent: usize,
}

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

	pub fn with_message_skipped(mut self, msg: &str) -> Self {
		self.message_skipped = Some(msg.to_string());
		self
	}

	pub fn with_indent(mut self, indent: usize) -> Self {
		self.indent = indent;
		self
	}

	pub fn print(&self) {
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

impl Display for TestResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TestResult::Skipped => f.write_fmt(format_args!(
				"{}{:<WIDTH$}{}",
				color::Fg(color::Cyan),
				"SKIPPED",
				color::Fg(color::Reset)
			)),
			TestResult::Passed => f.write_fmt(format_args!(
				"{}{:<WIDTH$}{}",
				color::Fg(color::Green),
				"PASSED",
				color::Fg(color::Reset)
			)),
			TestResult::Failed => f.write_fmt(format_args!(
				"{}{:<WIDTH$}{}",
				color::Fg(color::Red),
				"FAILED",
				color::Fg(color::Reset)
			)),
		}
	}
}

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
		match self.result {
			TestResult::Skipped => f.write_fmt(format_args!(
				"{:>indent$}{} {}{}{}",
				"",
				self.result,
				color::Fg(color::LightBlack),
				message,
				color::Fg(color::Reset)
			)),
			TestResult::Failed | TestResult::Passed =>
				f.write_fmt(format_args!("{:>indent$}{} {}", "", self.result, message)),
		}
	}
}
