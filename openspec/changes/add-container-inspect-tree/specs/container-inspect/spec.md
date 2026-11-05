# Spec Delta

## Purpose

Defines how a user browses and searches a container's inspect data.

## ADDED Requirements

### Requirement: Inspect data is shown as a collapsible tree
The system SHALL show the complete inspect response of the selected container as a tree when `i` or `K` is pressed in the Containers section. Top-level keys SHALL start visible with their branches collapsed. Loading SHALL NOT block the UI.

#### Scenario: Opening the tree
- **WHEN** a container is selected and the user presses `i`
- **THEN** a loading state appears, followed by the container's top-level inspect keys

#### Scenario: Expanding and collapsing
- **WHEN** the cursor is on a collapsed branch and the user presses `l`
- **THEN** its children become visible, and `h` on the expanded branch hides them again

#### Scenario: Moving to the parent
- **WHEN** the cursor is on a leaf or a collapsed branch and the user presses `h`
- **THEN** the cursor moves to the parent node

#### Scenario: Expanding and collapsing a whole branch
- **WHEN** the cursor is on a branch and the user presses `L`
- **THEN** the branch and all branches nested in it are expanded, and `H` collapses them all again

#### Scenario: Whole-branch keys on a leaf
- **WHEN** the cursor is on a leaf inside a branch and the user presses `L` or `H`
- **THEN** the key acts on the leaf's parent branch, and after `H` the cursor is on that branch

#### Scenario: Inspect fails
- **WHEN** the inspect call fails
- **THEN** the view closes and an error toast is shown

#### Scenario: Stale response
- **WHEN** a response arrives for a container other than the one currently open
- **THEN** the response is ignored

### Requirement: Search finds keys and scalar values
The system SHALL search case-insensitively over keys and scalar values as the user types after `/`, and SHALL reveal the match under the cursor by expanding its ancestors. Array indices SHALL NOT match.

#### Scenario: Match inside a collapsed branch
- **WHEN** the user searches for a value nested in collapsed branches
- **THEN** the cursor moves to it and its ancestors are expanded

#### Scenario: Stepping through matches
- **WHEN** a search is active and the user presses `n` or `N`
- **THEN** the cursor moves to the next or previous match, wrapping around

#### Scenario: Clearing a search
- **WHEN** a search is active and the user presses `Esc`
- **THEN** the search is cleared and the tree stays open; a further `Esc` closes it

### Requirement: Full value of the current row is readable
The system SHALL show the full path and the complete value of the row under the cursor, regardless of the row's width.

#### Scenario: Value wider than the view
- **WHEN** the cursor is on a value longer than the view is wide
- **THEN** the complete value is shown wrapped in the detail pane
