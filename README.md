# Ruled Labels

{Ruled-Labels} is compatible with the `glabel` yaml format and allow specifying additionnal rules regarding the labels. Those rules are described in a yaml file.

{Ruled-Labels} is aware of:

-   the list of labels and their properties

-   the parsing rules

-   the label rules

## cli

The cli can be used as by passing the labels extracted from an issue/PR:

    rl list --rules labels.yaml
    rl lint --rules labels.yaml

    rl check --label "B0-silent" --label "B1-note_worthy" --rules labels.yaml
    rl check --labels "B0-silent,B1-note_worthy" --rules labels.yaml --only b_rules --only b_excludes_j
    rl check --labels "B0-silent,B1-note_worthy" --rules labels.yaml --exclude b_rules --only b_excludes_j

    rl test --tests tests.yaml

If all the rules pass, `rl` returns 0.

## Weights

In some case, you may want to enforce that some rules are applied before or after others.
By default, rules have a priority of `100`. Any rule with a lower priority will be applied before those with higher prio.

## Vscode yaml

Add to the yaml pluggin config:

        "yaml.customTags": [
            "!none_of sequence",
            "!one_of sequence",
            "!some_of sequence",
            "!all_of sequence",
        ]
