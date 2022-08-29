//! Definition of common types and helper functions.

/// This helper function helps converting an interrable object of item convertible to
/// strings, as comma separated string, making debugging easier.
/// ## example:
/// ```
/// let data = vec!["Hello", "World"];
/// println!("{}", set_to_string(data));
/// // Output: "Hello, World"
/// ```
pub fn set_to_string<T: IntoIterator<Item = I>, I: ToString>(c: T) -> String {
	c.into_iter().map(|e| e.to_string()).collect::<Vec<String>>().join(", ")
}

pub fn capitalize(s: &str) -> String {
	let mut c = s.chars();
	match c.next() {
		None => String::new(),
		Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
	}
}
