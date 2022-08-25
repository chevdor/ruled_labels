#![allow(dead_code)]

pub mod common;
pub mod label_id_set;
pub mod label_match;
pub mod label_match_set;
pub mod parsed_label;
pub mod parser;
pub mod rule;
pub mod rules;
pub mod spec;
pub mod test_result;
pub mod tests;
pub mod token_rule;

pub fn set_to_string<T: IntoIterator<Item = I>, I: ToString>(c: T) -> String {
	c.into_iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", ")
}
