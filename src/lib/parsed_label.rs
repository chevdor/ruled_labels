use serde::Deserialize;

pub type CodeNumber = u8;

#[derive(Debug, Deserialize)]
pub struct LabelId {
	pub letter: char,
	pub number: CodeNumber,
}

#[derive(Debug, Deserialize)]
pub struct ParsedLabel {
	pub id: LabelId,
	pub description: Option<String>,
}
