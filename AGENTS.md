# AGENTS.md

Guidance for AI coding agents (Claude, Codex, etc.) working on **Draught of FPL**.
Humans are welcome to read it too.

## What this is

An intermediary server for a single **Fantasy Premier League Draft** league
(https://draft.premierleague.com). It polls the public FPL Draft API, computes a
live league table (including projected bonus points and projected auto-subs), and
serves it as JSON plus a small Vue frontend.

- **Focus: classic scoring leagues.** H2H code exists (`Scoring::H2H`, `h2h_info`,
  `matches`) but is not a priority — don't break it, don't extend it unless asked.
- **Most users are on phones.** Every GUI change must look good at ~375px wide first,
  desktop second.

## Architecture

```
Config (file or env) ─► initializer ─► Client (web or local files)
                                          │
              ┌───────────────────────────┴───────────────┐
   fetcher task (every 60s)                     computer task (every 30s)
   FPL API ─► FplEndpointsUpdate ─► merge into   clone FplEndpoints ─► compute
              Arc<RwLock<FplEndpoints>>          LeagueTable ─► Arc<RwLock<LeagueTable>>
                                                                   │
                                Axum server: GET /table (JSON), / (static www/vue)
                                                                   │
                                Browser: www/vue polls /table every 30s
```

- `src/main.rs` – CLI (`-f <config>`), wiring, Axum routes (`/table`, `/fpl/player/:id`, static files).
- `src/initializer.rs` – `AppConfig` (TOML file or `DOF_*` env vars) and `AppContext` (league id, team ids, intervals).
- `src/client/` – HTTP/local-file client for each FPL endpoint.
- `src/fetcher.rs` – periodic fetch; failed endpoints are `None` and keep their previous value (`FplEndpoints::update`).
- `src/computer.rs` – builds `LeagueTable`: entries, totals, projected points, auto-sub projection (`calculate_play_status`).
- `src/propcomp.rs` – small "property computations" over `FplEndpoints` (names, points, fixtures, bonus calculation).
- `src/storage/endpoints.rs` – `FplEndpoints` (raw cached API data) and its partial-update type.
- `src/storage/table.rs` – **`LeagueTable`, the JSON contract with the frontend.**
- `src/structs/` – serde models of the raw FPL API responses.
- `www/vue/` – frontend: `index.html`, `js/app.js` (Vue app + components), `js/functions.js`, `js/config.js`, `styles.css`.
- `db/create-db.py` – snapshots the FPL API into `db/api/` for offline runs (`local_fetch = true`).
- `db/snapshot.py` – saves the FPL API responses to `tests/snapshots/<name>/api/...` with member names replaced; `--complete` includes all gameweeks so far.
- `docker/`, `stack/` – container build and the Traefik + docker-compose deployment (`stack/DEPLOYREADME.md`).

## Commands

```bash
cargo build
cargo run -- -f Config.toml        # without -f, config comes from DOF_* env vars
cargo test
cargo clippy --all-targets
cargo fmt
python db/create-db.py <league_id> # run from db/; builds db/api for local_fetch
python db/snapshot.py <league_id> <name> [--complete] # save a test snapshot
```

Server listens on `server_port` (default 8000) and serves the frontend from
`asset_path` (default `./www/vue`), so run from the repo root.

Config keys (`Config.toml` / env): `league_id` / `DOF_LEAGUE_ID` (required),
`local_fetch` / `DOF_LOCAL_FETCH`, `local_url` / `DOF_LOCAL_URL`,
`server_port` / `DOF_SERVER_PORT`, `asset_path` / `DOF_ASSET_PATH`.

**Before every commit**, `cargo clippy --all-targets` must be warning-free and the code
`cargo fmt`-formatted. If a lint exception is genuinely more convenient, use
`#[expect(clippy::<lint>, reason = "...")]` (not `#[allow]`) so it warns once it's
no longer needed. The user runs tests manually; there is no CI.
Rust edition 2021; the Docker image builds with `rust:1.92`, so don't use features
newer than that without bumping `docker/app.Dockerfile` in the same branch.

**Agent sandboxes may not reach crates.io.** If you cannot build, say so and ask the
user to run `cargo test` / `cargo clippy` and paste the output — never claim code
compiles or tests pass without having seen it.

## FPL API notes and quirks

Base: `https://draft.premierleague.com/api`. Endpoints used: `/game`,
`/bootstrap-static`, `/league/{id}/details`, `/entry/{entry_id}/public`,
`/entry/{entry_id}/event/{gw}`, `/event/{gw}/live`.

- **Two ids per league team.** `league_entries[].entry_id` is used for `/entry/...`
  URLs and as `team_code` in our table; `league_entries[].id` is used in
  `standings` and H2H `matches`. Convert with `propcomp::get_team_id_from_entry_id` /
  `get_entry_id_from_team_id`.
- **`live.elements[id].explain`** is `Vec<[points_vec, fixture_id]>` — index 0 is the
  list of point sources, index 1 is the fixture id (see `PointsOrFixture`).
- **Picks:** `position` 1–11 is the starting XI, 12–15 the bench in priority order.
  `subs` lists auto-subs FPL has already made.
- **Bonus points** are only added by FPL after a match; until then we project them
  from BPS in `calculate_bonus_points`. Projected = official points + projected bonus
  + projected auto-subs.
- `finished_provisional` is what we treat as "fixture is over".
- Be polite to FPL: don't shorten fetch intervals or add per-player requests without asking.

## Frontend rules

- **Plain JS, no build step.** Vue 3 global build loaded from a CDN in `index.html`;
  no npm, no bundler, no TypeScript. Don't add one without a very strong reason
  and explicit approval.
- Components are registered in `js/app.js` with template strings.
- Mobile first: check layout at ~375px; avoid hover-only interactions; keep touch
  targets reasonably large.
- If you change the shape of `LeagueTable` (`src/storage/table.rs`), update
  `www/vue/js/app.js` in the same branch, and vice versa.

## Dependencies

- Keep dependencies current. Upgrade **one crate (or one tightly coupled group,
  e.g. axum + tower-http) per commit**, with any code changes the upgrade needs.
- Ask before adding a new crate. Prefer the standard library and existing deps.
- Frontend dependencies (Vue) are loaded from a CDN; pin exact versions.

## Testing

- **Tests must not need the network or a local `db/api` snapshot.** Prefer small,
  hand-written fixtures: build structs inline with `serde_json::json!`
  or load trimmed JSON files from a fixtures folder.
- **Full-flow regression tests** live in `src/full_flow_tests/`, one file per snapshot in
  `tests/snapshots/`. They run the client, fetcher, computer and router on saved responses,
  with one explicit expect line per value and no loops, so a failure names what broke.
  Run them with `cargo test full_flow`.
- Preferred direction: keep computations as pure functions over `FplEndpoints`
  (or smaller inputs) so tests can build that state from fixtures without a
  `Client`. Put shared JSON fixtures in `tests/fixtures/` and builder helpers in a
  `#[cfg(test)]` module. If the fetcher needs testing, put the client behind a trait
  with a fake implementation rather than reading files from disk.
- Logic that most needs tests: `calculate_play_status` (auto-sub projection),
  `calculate_bonus_points`, projected-point totals.
- When fixing a bug, add a test that reproduces it first when practical.
- Tests that hit the real FPL API must be `#[ignore]`.

## Git workflow

- **One branch per feature/fix**, branched from `master`. Existing naming:
  `issue-<number>-<short-description>` when there is a GitHub issue, otherwise a
  short kebab-case description.
- **Agents may create branches and commit to them**; the user reviews the commits.
- **Small commits.** One logical change per commit so each is easy to review.
  Don't mix formatting/refactoring with behaviour changes. Mechanical sweeps
  (a repo-wide `cargo fmt`, a batch of clippy fixes) go in one commit of their own.
- **Commits made by an AI agent start with `Vibe: `** (e.g.
  `Vibe: Add fixture tests for bonus point calculation`).
- **Never push, force-push, merge to `master`, or open PRs without asking first.**
- **No devops changes without asking first:** git hooks, CI/CD workflows, Docker and
  deployment files (`docker/`, `stack/`). The user doesn't want CI/CD.
- Never commit secrets: `stack/.env`, `stack/acme.json`, local `Config.toml` values.

## Planned work

Planned work and known bugs are tracked as GitHub issues, not in this file. Ask
which issue to work on, and name the branch after it.
Drafts not yet on GitHub may sit in the gitignored `issues/` folder.
