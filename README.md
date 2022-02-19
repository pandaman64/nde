Nix Development Envionment
===

Nix Development Envionment (NDE) aims to improve Nix's development/debugging experience greatly.

# Goals

## Instant Feedback, Everywhere

- The environment must give feedback on errors as soon as a user types characters.
- The environment must give feedback for code paths which will not (yet) be executed by normal Nix invocation.

## Error Insights

- The environment must provide a helpful explanation why an error occurs.
  - The explanation should present a possible fix.
  - The explanation should incorporate run-time information.
  - The explanation must provide necessary contexts more helpful than dynamic call stacks.

# Approach

## Flexible evaluator

- NDE evaluates a given Nix expression incrementally from every position.
- The evaluator collects run-time information such as errors and types for providing rich contexts.

## Post-hoc type analysis

- When a type error happens, NDE constructs an explanation why the evaluation leads to the error.