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
  cargo run -q -- --help | sed -r 's/\x1B\[(;?[0-9]{1,3})+[mGK]//g' > doc/usage/help.adoc
  cargo run -q -- lint | sed -r 's/\x1B\[(;?[0-9]{1,3})+[mGK]//g' > doc/usage/lint.adoc
  cargo run -q -- list | sed -r 's/\x1B\[(;?[0-9]{1,3})+[mGK]//g' > doc/usage/list.adoc
  cargo run -q -- check -l B0 A1 | sed -r 's/\x1B\[(;?[0-9]{1,3})+[mGK]//g' > doc/usage/check.adoc || true
  cargo run -q -- test | sed -r 's/\x1B\[(;?[0-9]{1,3})+[mGK]//g' > doc/usage/test.adoc || true

slides:
  #!/usr/bin/env bash
  cd doc/slides
  just _build

doc: _usage md slides

# Run the specs thru the tera template
gen_doc:
  #!/usr/bin/env bash
  FILE=specs
  tera $FILE.yaml -t templates/template.md.tera > $FILE.md
