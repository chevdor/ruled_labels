VERSION := `toml get Cargo.toml package.version | jq -r`
export TAG:=`toml get Cargo.toml "package.version" | jq -r .`

# List available commands
_default:
  just --choose --chooser "fzf +s -x --tac --cycle"

# Fetch labels of the glabel repo
fetch:
	glabel get chevdor/glabel -o glabel.yaml

# Run cargo tests
test:
  cargo nextest run

# Run the cli to test
run_tests:
  cargo run -- test

# Generate the readme as .md
md:
  #!/usr/bin/env bash
  asciidoctor -b docbook -a leveloffset=+1 -o - README_src.adoc | pandoc   --markdown-headings=atx --wrap=preserve -t markdown_strict -f docbook - > README.md

_usage:
  cargo run -q -- --help --no-color > doc/usage/help.adoc
  cargo run -q -- lint --no-color > doc/usage/lint.adoc
  cargo run -q -- list --no-color > doc/usage/list.adoc
  cargo run -q -- check -l B0 A1 --no-color > doc/usage/check.adoc || true
  cargo run -q -- test --no-color > doc/usage/test.adoc || true

_build_slides:
  #!/usr/bin/env bash
  cd doc/slides
  just _build

_rust_doc:
  cargo doc --no-deps

doc: _usage md _build_slides _rust_doc

# Run the specs thru the tera template
gen_doc:
  #!/usr/bin/env bash
  FILE=specs
  tera $FILE.yaml -t templates/template.md.tera > $FILE.md

# Open the slides in a browser
slides:
  #!/usr/bin/env bash
  just -d doc/slides --justfile doc/slides/justfile present
