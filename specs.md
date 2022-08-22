# Label Documentation for chevdor/glabel

This is the documenation for `chevdor/glabel` version 0.1.0.

It contains 14 labels and 6 rules.

## Description

Import from chevdor/glabel

## Labels: 14

The following list is sorted alphabetically.

- A1-foo
- A2-bar
- B0-silent
- B1-note_worthy
- B2-important
- D1-disable1
- J1-junk1
- J2-junk2
- P1-low_prio
- P2-low_prio
- P3-low_prio
- X1-frontend
- X2-backend
- X3-documentation


## Rules: 6

The following list is unsorted.

### Exactly one visibility label




This rule ensures we have a single visibility label.
It is important to void conflicts such as having a PR
labels as both important and unsubstantial.




- Require:

  - B*





### Some topics (X labels)






- Require:

  - X1
  - X2
  - J2





### WIP Note Worthy need one Prio label






- Require:

  - P*




- Exclude:

  - P1

### WIP Note Worthy implies no J label









- Exclude:

  - T*

### Exclude all Ds









- Exclude:

  - D*

### Disabled sample


⚠️ This rule is DISABLED.








- Exclude:

  - D*



