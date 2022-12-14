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
	mod lint {
		use assert_cmd::Command;

		#[test]
		fn it_lints_good_file() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("lint").arg("./tests/specs_ok.yaml").assert();
			assert.success().code(0);
		}
		#[test]
		fn it_lints_bad_file() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("lint").arg("./tests/specs_err.yaml").assert();
			assert.failure().code(1);
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
		fn it_calls_check_and_pass() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd
				.arg("check")
				.arg("./tests/specs_ok.yaml")
				.arg("-l")
				.arg("B1")
				.arg("X1")
				.arg("X2")
				.arg("X3")
				.arg("P2")
				.assert();
			assert.success().code(0);
		}

		#[test]
		fn it_calls_check_and_pass_with_comma_no_spaces() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd
				.arg("check")
				.arg("./tests/specs_ok.yaml")
				.arg("-l")
				.arg("B1,X1,X2,X3,P2")
				.assert();
			assert.success().code(0);
		}

		#[test]
		fn it_calls_check_and_fails() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd
				.arg("check")
				.arg("./tests/specs_ok.yaml")
				.arg("-l")
				.arg("B1")
				.arg("X1")
				.arg("X2")
				.arg("X3")
				.arg("P1")
				.assert();
			assert.failure().code(1);
		}

		#[test]
		fn it_calls_check_and_fail_1() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd
				.arg("check")
				.arg("./tests/specs_ok.yaml")
				.arg("-l")
				.arg("B1")
				.arg("X1")
				.arg("X2")
				.assert();
			assert.failure().code(1);
		}

		#[test]
		fn it_calls_check_and_fail_2() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd
				.arg("check")
				.arg("./tests/specs_ok.yaml")
				.arg("-l")
				.arg("B1")
				.arg("X1")
				.arg("X2")
				.arg("X2")
				.assert();
			assert.failure().code(1);
		}

		#[test]
		fn it_calls_check_and_fail_3() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("check").arg("./tests/specs_ok.yaml").arg("-l").arg("B0").assert();
			assert.failure().code(1);
		}
	}

	#[cfg(test)]
	mod test {
		use assert_cmd::Command;

		#[test]
		fn it_tests_good_all() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd
				.arg("test")
				.arg("./tests/tests_pass.yaml")
				.arg("-s")
				.arg("./tests/specs_ok.yaml")
				.arg("--all")
				.assert();
			assert.success().code(0);
		}

		#[test]
		fn it_tests_good_only() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd
				.arg("test")
				.arg("./tests/tests_pass.yaml")
				.arg("-s")
				.arg("./tests/specs_ok.yaml")
				.arg("--only")
				.assert();
			assert.success().code(0);
		}

		#[test]
		fn it_tests_bad() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd
				.arg("test")
				.arg("./tests/tests_fail.yaml")
				.arg("-s")
				.arg("./tests/specs_ok.yaml")
				.assert();
			assert.failure().code(1);
		}

		#[test]
		fn it_passes_when_no_labels_required() {
			let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
			let assert = cmd.arg("check").arg("./tests/specs_mini.yaml").arg("--no-label").assert();
			assert.success().code(0);
		}
	}
}
