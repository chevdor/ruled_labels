VERSION := `toml get Cargo.toml package.version | jq -r`
CLI_NAME := `toml get Cargo.toml package.name | jq -r`
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
  #!/usr/bin/env bash
  unset RUST_LOG
  cargo run -q -- --help --no-color > doc/usage/help.adoc
  cargo run -q -- lint --no-color > doc/usage/lint.adoc
  cargo run -q -- list --no-color > doc/usage/list.adoc
  cargo run -q -- check --dev -l B0 A1 --no-color > doc/usage/check.adoc || true
  cargo run -q -- test --all --no-color > doc/usage/test.adoc || true

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

# Run Rustfmt
fmt:
  cargo +nightly fmt

# Run Clippy
clippy:
  cargo +nightly clippy

check: fmt clippy

# Open the slides in a browser
slides:
  #!/usr/bin/env bash
  just -d doc/slides --justfile doc/slides/justfile present

# Build and tag the docker images
docker_build:
  docker build -t {{ CLI_NAME }} -t chevdor/{{ CLI_NAME }} -t chevdor/{{ CLI_NAME }}:{{ VERSION }} .
  docker images | grep {{ CLI_NAME }}

# Push the docker image
docker_push:
  docker push chevdor/{{ CLI_NAME }}
  docker push chevdor/{{ CLI_NAME }}:{{ VERSION }}

git_tag:
  git tag v{{ VERSION }} -f
  git push origin v{{ VERSION }} -f
