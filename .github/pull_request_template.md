## Result

Describe the problem or missing hardware and what this change lets a user inspect or insert.
Keep one asset family, correction or feature in this PR. Link a related issue if there is one.

## Hardware and evidence

- Manufacturer/model/variant or canonical IDs:
- Source URLs, document pages/revisions, or specimen measurement method:
- Given dimensions, derived values and photo estimates:
- Remaining unknowns or conflicting source values:
- Compatibility and installed-pose evidence, if changed:

For a documentation-only change, mark the unrelated fields as not applicable.
Link vendor media rather than adding PDF, image or CAD copies to the repository.

## Validation

List the exact checks and results. Use `cargo xtask ci`; when available,
`SOLIDCRAFT_CLI=/absolute/path/to/solidcraft-cli cargo xtask ci` also checks the host.
State anything that could not run. Do not report skipped checks as passed.

## Actual CAD evidence

Attach your own screenshots or recording for a visible geometry/UI change, with
the parameters or minimal script used. Show the affected output, mounting, rear
or cable interface and uncertainty labels. Mention save/reopen results when
relevant. Documentation-only changes need no screenshot.

Changes a default: No, or describe the previous and new behavior and compatibility path.
