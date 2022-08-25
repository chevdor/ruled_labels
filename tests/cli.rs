#[cfg(test)]
mod cli_tests {

	#[cfg(test)]
	mod help {
		use assert_cmd::Command;

		#[test]
		fn it_shows_help() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("--help").assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod list {
		use assert_cmd::Command;

		#[test]
		fn it_calls_list() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("list").assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod check {
		use assert_cmd::Command;

		#[test]
		fn it_calls_check() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("check").arg("-l").arg("B1").arg("B2").assert();
			assert.success().code(0);
		}
	}

	#[cfg(test)]
	mod test {
		use assert_cmd::Command;

		#[test]
		#[ignore = "todo"]
		fn it_calls_test() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("test").assert();
			assert.success().code(0);
		}
	}
}
