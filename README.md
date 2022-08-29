# Ruled Labels

![badge](https://github.com/chevdor/ruled_labels/actions/workflows/quick-check.yml/badge.svg?branch=master)

## Intro

`ruled-labels` is compatible with the `glabel` yaml format and allow specifying additionnal rules regarding the labels. Those rules are described in a yaml file.

`ruled-labels` can handle:

-   the list of labels and their properties

-   the label rules

-   a description of test cases

## Install

There is currenly no binary package so you need to use `cargo` to build and install `ruled-labels` on your system.

    cargo install --locked --git https://github.com/chevdor/ruled_labels

## Docker

If you prefer not having to install Rust & Cargo and have Docker installed, you may prefer to run a dockerized version of `ruled-labels`. The next chapters explain how to proceed.

### Run

Docker commands can end up quite lenghty so you may like to set an alias:

        alias ruled-labels='docker run --rm -it ruled-labels'

After setting this alias, you may use `ruled-labels` by simply invoking the `ruled-labels` command:

        ruled-labels --version

If you prefer a shorter a command, you may set an alias for `rl` instead of `ruled-labels`.

This is out of the scope of this documentation but note that you cannot just invoke `ruled-labels` check and expect it to work on your local `specs.yaml`. For that to work, you need to mount your `specs.yaml` into the container. That looks like this:

        docker run --rm -it -v $PWD/specs.yaml:/usr/local/bin/specs.yaml <literal>ruled-labels</literal> list

### Build

You can pull the docker image from `chevdor`/`ruled-labels` or build you own:

        docker build -t ruled-labels .

## Features

-   rule based engine

-   rules defined as yaml

-   `lint`: the lint command helps you validate your yaml files

-   `list`: show a summary of your rules

-   `test`: You can define a set of tests scenarii to check against your specs to ensure you did not leave anything behind

-   `check`: CI can call this command to check a set of labels against your specs & rules

# Usage

## Help

    ruled-labels 0.1.0

    This utility allows checking labels based on rules

    USAGE:
        ruled-labels [OPTIONS] <SUBCOMMAND>

    OPTIONS:
        -h, --help        Print help information
            --no-color    Output without any coloring, this is useful for documentation and CI system
                          where the color code pollute the output
        -V, --version     Print version information

    SUBCOMMANDS:
        check    Check label set against the rules
        help     Print this message or the help of the given subcommand(s)
        lint     Lint the rules
        list     List all the rules
        test     Run tests using rules and a test set

## Lint

    PASSED  The file specs.yaml looks OK

## List

    name: chevdor/glabel
    desc: Import from chevdor/glabel
    labels: 14
    Rules:
     - Some topics (X labels)
     - Exactly one visibility label (b_rules)
     - Note Worthy need one Prio label (b_need_p)
     - Note Worthy implies no J label (b_excludes_j)
     - Exclude all Ds
     - Require all of J
     - Require 1 P and no X

## Test

    Tests specs: tests.yaml
    Specs file : specs.yaml

        ▶️ Running test  1: Fail
          Expected to FAIL
          Running checks on 3 labels: B1, X1, B0
            PASSED  Some topics (X labels)
            FAILED  Exactly one visibility label
            SKIPPED Note Worthy need one Prio label
            PASSED  Note Worthy implies no J label
            PASSED  Exclude all Ds
            FAILED  Require all of J
            FAILED  Require 1 P and no X
        PASSED  Fail

        ▶️ Running test  2: Pass
          Expected to PASS
          Running checks on 4 labels: B0, X2, X1, X3
            PASSED  Some topics (X labels)
            PASSED  Exactly one visibility label
            FAILED  Note Worthy need one Prio label
            PASSED  Note Worthy implies no J label
            PASSED  Exclude all Ds
            FAILED  Require all of J
            FAILED  Require 1 P and no X
        FAILED  Pass

        ▶️ Running test  3: Missing topics
          Expected to FAIL
          Running checks on 1 labels: B0
            FAILED  Some topics (X labels)
            PASSED  Exactly one visibility label
            FAILED  Note Worthy need one Prio label
            PASSED  Note Worthy implies no J label
            PASSED  Exclude all Ds
            FAILED  Require all of J
            FAILED  Require 1 P and no X
        PASSED  Missing topics

        ▶️ Running test  4: Fail
          Expected to FAIL
          Running checks on 2 labels: B1, A1
            FAILED  Some topics (X labels)
            PASSED  Exactly one visibility label
            FAILED  Note Worthy need one Prio label
            PASSED  Note Worthy implies no J label
            PASSED  Exclude all Ds
            FAILED  Require all of J
            FAILED  Require 1 P and no X
        PASSED  Fail
    FAILED  Some expectations were not OK

## Check

          Running checks on 2 labels: B0, A1
            FAILED  Some topics (X labels)
            PASSED  Exactly one visibility label
            FAILED  Note Worthy need one Prio label
            PASSED  Note Worthy implies no J label
            PASSED  Exclude all Ds
            FAILED  Require all of J
            FAILED  Require 1 P and no X
    FAILED  chevdor/glabel v0.1.0 for labels B0, A1

## Vscode yaml

Add to the [yaml pluggin](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml) (by RedHat) config:

    "yaml.customTags": [
        "!none_of sequence",
        "!one_of sequence",
        "!some_of sequence",
        "!all_of sequence",
    ]

## Yaml specs

## Rules

    ---
    # tag::header[]
    name: chevdor/glabel
    version: 0.1.0
    description: Import from chevdor/glabel
    # end::header[]

    # tag::labels[]
    labels:
      - name: A1-foo
        description: Foo
        color: d73a4a
      - name: A2-bar
        description: Bar
        color: d73a4a
    # end::labels[]
      - name: X1-frontend
        description: Frontend
        color: d73a4a
      - name: X2-backend
        description: Frontend
        color: d73a4a
      - name: X3-documentation
        description: Improvements or additions to documentation
        color: 0075ca
      - name: P1-low_prio
        description: Low prio
        color: ffffff
      - name: P2-low_prio
        description: Low prio
        color: ffffff
      - name: P3-low_prio
        description: Low prio
        color: ffffff
      - name: B0-silent
        description: silent
        color: ffffff
      - name: B1-note_worthy
        description: note worthy
        color: ffffff
      - name: B2-important
        description: important
        color: ffffff
      - name: J1-junk1
        description: junk1
        color: ffffff
      - name: J2-junk2
        description: junk2
        color: ffffff
      - name: D1-disable1
        description: diable1
        color: ffffff

    # tag::rules[]
    rules:
      - name: Some topics (X labels)
        spec:
          require: !some_of
            - X1
            - X2
            - J2
    # end::rules[]

      - name: Exactly one visibility label
        id: b_rules
        tags: [ CI ]
        priority: 1000
        description: |
          This rule ensures we have a single visibility label.
          It is important to void conflicts such as having a PR
          labels as both important and unsubstantial.
        spec:
          require: !one_of
            - B*
          exclude: ~


      - name: Note Worthy need one Prio label
        id: b_need_p
        tags: [v2]
        priority: 100
        spec:
          # when we have one of the B labels
          when: !one_of
            - B*
          # we need exactly one of the P labels
          require: !one_of
            - P*
          # but we don't want the P1 labels
          exclude: !all_of
            - P1

      - name: Note Worthy implies no J label
        id: b_excludes_j
        spec:
          # when:
          #   - B*
          exclude: !all_of
            - T*

      - name: Exclude all Ds
        disabled: false   # default
        priority: 100         # default
        spec:
          exclude: !all_of
            - D*

      - name: Require all of J
        spec:
          require: !all_of
            - J*

      - name: Require 1 P and no X
        spec:
          require: !one_of
            - P*
          exclude: !all_of ["X1", "X2"]

## Test

    name: Name of the test
    spec_file: specs.yaml

    specs:
      - name: Fail
        skip: true
        labels:
          - B1-note_worthy
          - B0-silent
          - X1-foo
        expected: false

      - name: Pass
        labels: [ B0-silent, X1-bar, X2-bar, X3-foobar ]
        expected: true

      - name: Missing topics
        skip: true
        labels:
          - B0-silent
        expected: false

      - name: Fail
        only: true
        labels:
          - A1
          - B1
        expected: false

## Glossary

In order to understand the terminology and what are `LabelMatch`, `LabelMatchSet`, `Labels`, `LabelId`, etc…​ please refer to the Rust documenation. You can generate and open it using:

    cargo doc --no-deps --open

## Licence

    Copyright 2021-2022 - Wilfried Kopp aka. Chevdor <chevdor@gmail.com>

    Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
