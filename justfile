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

# Run the specs thru the tera template
gen_doc:
  #!/usr/bin/env bash
  FILE=specs
  tera $FILE.yaml -t templates/template.md.tera > $FILE.md
