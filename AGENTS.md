<!-- OPENSPEC:START -->
# OpenSpec Workflow for OrderBook-rs

These instructions keep humans and AI assistants aligned while working in this repository. Follow them before writing or modifying code.

## When to Use OpenSpec
- Mentioned work involves new capabilities, behavior changes, architecture shifts, migrations, performance/security work, or anything ambiguous.
- Requests use words like "spec", "proposal", "plan", "change", or "requirements".
- Unsure whether the work alters expected behavior — default to creating an OpenSpec change.
- Skip proposals only for scoped fixes that restore existing behavior (typos, formatting, dependency bumps, flaky-test fixes).

## Required Actions Before Coding
- Read `openspec/project.md` for project context and conventions.
- Run `openspec list` to review active change proposals and `openspec list --specs` to understand existing capabilities.
- Open `openspec/specs/<capability>/spec.md` plus any relevant `openspec/changes/<change-id>/` folders.
- Capture questions or ambiguities in the proposal before touching implementation.

## Change Lifecycle
1. **Draft proposal**: Create a unique verb-led `change-id` (e.g., `add-depth-snapshots`) and scaffold `proposal.md`, `tasks.md`, optional `design.md`, and delta specs under `openspec/changes/<change-id>/`.
2. **Align on specs**: Document deltas with `## ADDED|MODIFIED|REMOVED Requirements` and at least one `#### Scenario:` per requirement. Iterate until the proposal is approved.
3. **Implement tasks**: Work through `tasks.md` sequentially. Keep checklists accurate and note deviations in the change folder.
4. **Validate**: Run `openspec validate <change-id> --strict` before and after implementation. Resolve all issues.
5. **Archive**: Once shipped, run `openspec archive <change-id> --yes` (or equivalent manual steps) to move the change into `openspec/changes/archive/` and merge spec updates into `openspec/specs/`.

## CLI Quick Reference
- `openspec list` – active changes (use `--json` for automation).
- `openspec list --specs` – registered capabilities/specs.
- `openspec show <item>` – inspect a change or spec; add `--type` or `--json` as needed.
- `openspec validate <item> --strict` – check formatting and structure.
- `openspec archive <change-id> --yes` – archive after deployment (optionally `--skip-specs` for tooling-only changes).
- Prefer `openspec` commands over raw file edits; use `rg` only for full-text searches when necessary.

## File Map & Resources
- Root instructions: this file (`AGENTS.md`). Keep the `<!-- OPENSPEC:START -->` and `<!-- OPENSPEC:END -->` markers intact so `openspec update` can refresh content.
- Detailed guidance: `@/openspec/AGENTS.md`.
- Project context & conventions: `@/openspec/project.md` (keep current).
- Specs source of truth: `@/openspec/specs/`.
- Active proposals & implementation plans: `@/openspec/changes/`.

Always surface blocking questions or missing specs before implementing. Treat the spec as the contract—code should never diverge from approved OpenSpec changes.
<!-- OPENSPEC:END -->
